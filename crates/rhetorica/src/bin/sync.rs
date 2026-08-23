//! Sync geometry AND contract blocks from a SARVA database dump into the
//! per-figure dataset — plus the reverse (`import`) direction.
//!
//! SARVA remains the source of truth for definitions + compiled geometry;
//! this repo owns examples and attribution. The dump carries the legacy
//! Indonesian DB convention (jangkar/kelas/satuan/operasi/minim_ulangan);
//! this tool translates it into the public English schema (`geometry` with
//! anchor/class/grain/operation/min_repeats) before writing.
//!
//! CONTRACT.md §12: the dump now also carries each figure's `signature`
//! and `epistemic` ladder state; both land in the dataset as contract
//! blocks. The reverse direction (`import`) emits SQL upserts from the
//! dataset's contract blocks so accepted contributions flow back into
//! the vault.
//!
//! Usage:
//!   1. Export the dump from your SARVA vault:
//!      sqlite3 -readonly "$HOME/.local/share/sarva/sarva_vault.db" \
//!        "SELECT json_group_array(json_object(
//!           'id',f.id,'name',f.name,'definition',f.definition,
//!           'geometri',f.geometri,
//!           'signature',json_object(
//!             'domain_id',s.domain_id,'unit_id',s.unit_id,
//!             'scope_id',s.scope_id,'anchor_id',s.anchor_id,
//!             'operation_id',s.operation_id,'payload_id',s.payload_id,
//!             'locus_id',s.locus_id),
//!           'epistemic',json_object(
//!             'status',COALESCE(e.epistemic_status,'PROSE_ONLY'),
//!             'legacy_status',e.legacy_status)))
//!         FROM figures f
//!         LEFT JOIN signatures s ON s.figure_id=f.id
//!         LEFT JOIN figure_epistemic_state e ON e.figure_id=f.id
//!         WHERE f.definition IS NOT NULL;" > /tmp/sarva_dump.json
//!   2. Run:
//!      cargo run -p figeometrica-rhetorica --bin sync -- /tmp/sarva_dump.json
//!      (reverse direction:)
//!      cargo run -p figeometrica-rhetorica --bin sync -- import /tmp/backflow.sql
//!   3. Review with: git diff && cargo run -p figeometrica-rhetorica --bin validate
//!      && cargo run -p figeometrica-rhetorica --bin sidang

use std::fs;
use std::path::{Path, PathBuf};

fn slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(serde::Deserialize)]
struct DumpEntry {
    name: String,
    #[serde(default)]
    definition: Option<String>,
    geometri: Option<serde_json::Value>,
    /// CONTRACT §2 signature (DB column names; translated to the public block).
    #[serde(default)]
    signature: Option<serde_json::Value>,
    /// CONTRACT §7 epistemic ladder.
    #[serde(default)]
    epistemic: Option<serde_json::Value>,
}

/// Dump-style `signature` block (DB columns) → public contract block.
/// Null in every slot = no real signature → None.
fn signature_block(raw: &serde_json::Value) -> Option<serde_json::Value> {
    let o = raw.as_object()?;
    let grab = |k: &str| o.get(k).and_then(|v| v.as_str()).filter(|s| !s.is_empty());
    let domain = grab("domain_id")?;
    let unit = grab("unit_id")?;
    let anchor = grab("anchor_id")?;
    let operation = grab("operation_id")?;
    let mut out = serde_json::Map::new();
    out.insert("domain_id".into(), serde_json::json!(domain));
    out.insert("unit_id".into(), serde_json::json!(unit));
    match grab("scope_id") {
        Some(s) => out.insert("scope_id".into(), serde_json::json!(s)),
        None => out.insert("scope_id".into(), serde_json::Value::Null),
    };
    out.insert("anchor_id".into(), serde_json::json!(anchor));
    out.insert("operation".into(), serde_json::json!(operation));
    match grab("payload_id") {
        Some(p) => out.insert("payload_id".into(), serde_json::json!(p)),
        None => out.insert("payload_id".into(), serde_json::Value::Null),
    };
    match grab("locus_id") {
        Some(l) => out.insert("locus_id".into(), serde_json::json!(l)),
        None => out.insert("locus_id".into(), serde_json::Value::Null),
    };
    Some(serde_json::Value::Object(out))
}

/// Legacy Indonesian value → public English value.
const ANCHOR: &[(&str, &str)] = &[
    ("Awal", "Initial"),
    ("Akhir", "Final"),
    ("Sisipan", "Insertion"),
    ("UnitUtuh", "WholeUnit"),
    ("AntarUnit", "CrossUnit"),
];
const CLASS: &[(&str, &str)] = &[
    ("Leksikal", "Lexical"),
    ("Akar", "Root"),
    ("Gramatikal", "Grammatical"),
    ("Konseptual", "Conceptual"),
];
const GRAIN: &[(&str, &str)] = &[
    ("grafem", "grapheme"),
    ("kata", "word"),
    ("frasa", "phrase"),
    ("unit", "unit"),
    ("wacana", "discourse"),
];
const OPERATION: &[(&str, &str)] = &[
    ("adjectio", "addition"),
    ("detractio", "deletion"),
    ("immutatio", "substitution"),
    ("transmutatio", "permutation"),
    ("repetitio", "repetition"),
    ("ordering", "ordering"),
];

fn en(value: &str, table: &[(&str, &str)]) -> String {
    table
        .iter()
        .find(|(legacy, _)| *legacy == value)
        .map(|(_, public)| public.to_string())
        .unwrap_or_else(|| value.to_string())
}

/// Translate a geometry object into the public English schema. Accepts the
/// value either as a real object or as a string containing JSON (the SQLite
/// dump wraps TEXT columns as strings). Returns `None` when required fields
/// are missing/malformed.
fn terjemahkan(geo: &serde_json::Value) -> Option<serde_json::Value> {
    let geo = match geo {
        serde_json::Value::String(s) => serde_json::from_str::<serde_json::Value>(s).ok()?,
        v => v.clone(),
    };
    let o = geo.as_object()?;
    fn as_str(v: Option<&serde_json::Value>) -> Option<&str> {
        v?.as_str()
    }
    let pick = |en_key: &str, id_key: &str| -> Option<&serde_json::Value> {
        o.get(en_key).or_else(|| o.get(id_key))
    };

    let mut out = serde_json::Map::new();
    out.insert(
        "anchor".into(),
        serde_json::Value::String(en(as_str(pick("anchor", "jangkar"))?, ANCHOR)),
    );
    out.insert(
        "class".into(),
        serde_json::Value::String(en(as_str(pick("class", "kelas"))?, CLASS)),
    );
    if let Some(s) = as_str(pick("grain", "satuan")) {
        out.insert("grain".into(), serde_json::Value::String(en(s, GRAIN)));
    }
    if let Some(s) = as_str(pick("operation", "operasi")) {
        out.insert("operation".into(), serde_json::Value::String(en(s, OPERATION)));
    }
    if let Some(n) = pick("min_repeats", "minim_ulangan").and_then(|v| v.as_u64()) {
        out.insert("min_repeats".into(), serde_json::Value::from(n));
    }
    if let Some(t) = pick("template", "template") {
        out.insert("template".into(), t.clone());
    }
    if let Some(arr) = pick("transforms", "transformasi").and_then(|v| v.as_array()) {
        let mapped: Vec<serde_json::Value> = arr
            .iter()
            .filter_map(|x| {
                let o = x.as_object()?;
                let axis = o.get("axis").or_else(|| o.get("sumbu"))?.as_str()?;
                let dir = o.get("direction").or_else(|| o.get("arah"))?.as_str()?;
                let direction = match dir {
                    "up" | "naik" => "up",
                    "down" | "turun" => "down",
                    _ => "neutral",
                };
                Some(serde_json::json!({ "axis": axis, "direction": direction }))
            })
            .collect();
        if !mapped.is_empty() {
            out.insert("transforms".into(), serde_json::Value::Array(mapped));
        }
    }
    if let Some(l) = as_str(pick("locus", "locus")) {
        let v = match l {
            "setiap" => "every",
            "respons" => "response",
            "ujung" => "terminal",
            "tersebar" => "distributed",
            "berumpun" => "clustered",
            "tengah" => "medial",
            other => other,
        };
        out.insert("locus".into(), serde_json::Value::String(v.into()));
    }
    if let Some(s) = as_str(pick("note", "catatan")) {
        out.insert("note".into(), serde_json::Value::String(s.into()));
    }
    Some(serde_json::Value::Object(out))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // ── Reverse direction: dataset → SQL upserts for the vault ──────
    if args.get(1).map(|a| a.as_str()) == Some("import") {
        let output = args.get(2).cloned().unwrap_or_else(|| "/tmp/sarva_import.sql".into());
        import(&PathBuf::from(&output));
        return;
    }

    // ── Main direction: vault dump → dataset ────────────────────────
    let Some(dump_path) = args.get(1) else {
        eprintln!("Usage: sync <sarva_dump.json>");
        eprintln!("       sync import <output.sql>");
        eprintln!("See the header comment of this file for how to create the dump.");
        std::process::exit(2);
    };

    let dump_raw = fs::read_to_string(dump_path).unwrap_or_else(|e| {
        eprintln!("Cannot read {dump_path}: {e}");
        std::process::exit(1);
    });
    let entries: Vec<DumpEntry> = serde_json::from_str(&dump_raw).unwrap_or_else(|e| {
        eprintln!("Dump is not a valid array: {e}");
        std::process::exit(1);
    });

    // Dataset lives at the repository root (same as build.rs reads).
    let dataset_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/figures");

    let mut updated = 0usize;
    let mut skipped = 0usize;
    let mut no_file = 0usize;

    for e in &entries {
        let path = dataset_dir.join(format!("{}.json", slug(&e.name)));
        if !path.exists() {
            no_file += 1;
            continue;
        }
        // Geometry is optional; a definition alone still syncs.
        let translated = match e.geometri.as_ref().filter(|g| !g.is_null()) {
            Some(geo) => match terjemahkan(geo) {
                Some(t) => Some(t),
                None => {
                    eprintln!("⚠ {}: geometry not translatable, skipped", e.name);
                    None
                }
            },
            None => None,
        };
        if translated.is_none() && e.definition.is_none() {
            skipped += 1;
            continue;
        }
        if apply(
            &path,
            translated.as_ref().unwrap_or(&serde_json::Value::Null),
            e.definition.as_deref(),
            e.signature.as_ref().and_then(signature_block),
            e.epistemic.as_ref(),
        ) {
            println!("↺ {}: geometry/definition/contract copied from SARVA", e.name);
            updated += 1;
        } else {
            skipped += 1;
        }
    }

    println!("{updated} files updated, {skipped} already equal/skipped, {no_file} figures without a dataset file.");
    if updated > 0 {
        println!("Next: git diff data/figures && cargo run -p figeometrica-rhetorica --bin validate && cargo run -p figeometrica-rhetorica --bin sidang");
    }
}

/// Overwrite the `geometry`, `definition`, `signature` and `epistemic`
/// fields when they differ.
fn apply(
    path: &Path,
    geometry_new: &serde_json::Value,
    definition_new: Option<&str>,
    signature_new: Option<serde_json::Value>,
    epistemic_new: Option<&serde_json::Value>,
) -> bool {
    let raw = fs::read_to_string(path).expect("dataset file readable");
    let mut v: serde_json::Value = serde_json::from_str(&raw).expect("valid dataset JSON");
    let geo_equal = match geometry_new {
        serde_json::Value::Null => true, // no geometry upstream — leave untouched
        g => v.get("geometry").map(|x| x == g).unwrap_or(false),
    };
    let def_equal = match (v.get("definition"), definition_new) {
        (Some(a), Some(b)) => a.as_str() == Some(b),
        (None, None) => true,
        _ => false,
    };
    let sig_equal = match (&signature_new, v.get("signature")) {
        (Some(s), Some(existing)) => s == existing,
        (Some(_), None) => false,
        (None, _) => true, // no upstream signature — leave old block alone
    };
    let epi_equal = match (epistemic_new, v.get("epistemic")) {
        (Some(e), Some(existing)) => {
            // compare status+legacy only; free-form note may differ
            let status = |o: &serde_json::Value| o.get("status").cloned().unwrap_or(serde_json::json!(""));
            let legacy = |o: &serde_json::Value| o.get("legacy_status").cloned();
            status(e) == status(existing) && legacy(e) == legacy(existing)
        }
        (Some(_), None) => false,
        (None, _) => true,
    };

    if geo_equal && def_equal && sig_equal && epi_equal {
        return false;
    }
    if !geo_equal {
        v["geometry"] = geometry_new.clone();
    }
    if let Some(d) = definition_new {
        v["definition"] = serde_json::Value::String(d.to_string());
    }
    if let Some(s) = signature_new {
        v["signature"] = s;
    }
    if let Some(e) = epistemic_new {
        let mut blok = serde_json::Map::new();
        blok.insert("status".into(), e.get("status").cloned().unwrap_or(serde_json::json!("PROSE_ONLY")));
        if let Some(l) = e.get("legacy_status") {
            if !l.is_null() {
                blok.insert("legacy_status".into(), l.clone());
            }
        }
        // pertahankan note lokal bila ada
        if let Some(note) = v.get("epistemic").and_then(|x| x.get("note")) {
            blok.insert("note".into(), note.clone());
        }
        v["epistemic"] = serde_json::Value::Object(blok);
    }
    // Drop the legacy Indonesian key so the entry never carries both
    // (serde treats field+alias as one target → duplicate-field error).
    if let Some(obj) = v.as_object_mut() {
        obj.remove("geometri");
        obj.retain(|_, val| !val.is_null());
    }
    fs::write(
        path,
        serde_json::to_string_pretty(&v).expect("serialize") + "\n",
    )
    .expect("write dataset file");
    true
}

/// Reverse direction (`import`): read contract blocks from the dataset,
/// emit SQL upserts the maintainer can apply to the SARVA vault —
/// contributions ratified on the public ledger flow back into the lab.
fn import(output: &Path) {
    let dataset_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/figures");
    let paths: Vec<PathBuf> = fs::read_dir(&dataset_dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dataset_dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();

    let mut sql = String::from("BEGIN;\n");
    let mut n_sig = 0usize;
    let mut n_epi = 0usize;

    for path in paths {
        let raw = match fs::read_to_string(&path) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else { continue };
        let Some(name) = v.get("name").and_then(|n| n.as_str()) else { continue };

        if let Some(sig) = v.get("signature").filter(|s| s.is_object()) {
            let grab = |k: &str| sig.get(k).and_then(|x| x.as_str()).unwrap_or("");
            if grab("domain_id").is_empty() || grab("anchor_id").is_empty() {
                continue; // fail-closed: a required slot is missing → skip
            }
            let opt = |k: &str| {
                sig.get(k)
                    .and_then(|x| x.as_str())
                    .map(|s| format!("'{s}'"))
                    .unwrap_or_else(|| "NULL".into())
            };
            sql.push_str(&format!(
                "INSERT INTO signatures (figure_id, domain_id, unit_id, scope_id, anchor_id, operation_id, payload_id, locus_id, provenance) \
                 SELECT id, '{d}', '{u}', {sc}, '{a}', '{op}', {pl}, {lo}, 'public-ledger' FROM figures WHERE name='{name}' \
                 ON CONFLICT(figure_id) DO UPDATE SET domain_id='{d}', unit_id='{u}', scope_id={sc}, anchor_id='{a}', operation_id='{op}', payload_id={pl}, locus_id={lo}, provenance='public-ledger';\n",
                d = grab("domain_id"), u = grab("unit_id"), sc = opt("scope_id"),
                a = grab("anchor_id"), op = grab("operation"),
                pl = opt("payload_id"), lo = opt("locus_id"),
            ));
            n_sig += 1;
        }

        if let Some(epi) = v.get("epistemic").filter(|e| e.is_object()) {
            let status = epi.get("status").and_then(|s| s.as_str()).unwrap_or("PROSE_ONLY");
            let legacy = epi
                .get("legacy_status")
                .and_then(|s| s.as_str())
                .map(|s| format!("'{s}'"))
                .unwrap_or_else(|| "NULL".into());
            sql.push_str(&format!(
                "INSERT INTO figure_epistemic_state (figure_id, epistemic_status, legacy_status) \
                 SELECT id, '{status}', {legacy} FROM figures WHERE name='{name}' \
                 ON CONFLICT(figure_id) DO UPDATE SET epistemic_status='{status}', legacy_status={legacy}, updated_at=datetime('now');\n"
            ));
            n_epi += 1;
        }
    }

    sql.push_str("COMMIT;\n");
    fs::write(output, sql).expect("write sql");
    println!("import: {n_sig} signatures + {n_epi} states → {}", output.display());
    println!("Apply manually after review: sqlite3 vault.db < {path}", path = output.display());
}
