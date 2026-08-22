//! CONTRACT.md §12 — contract trial over the public dataset.
//!
//! CI gate engine: load the canonical knowledge manifest (highest vN
//! folder) + the embedded dataset (data/figures/*.json), then for every
//! signature-bearing figure:
//!   1. slots ∈ manifest vocabulary (fail-closed);
//!   2. bindings: INVALID = fail, UNKNOWN legal but recorded;
//!   3. witness protocol re-run (`run_protocol_auto`);
//!   4. ladder consistency: a claimed status without evidence FAILS
//!      (NO SILENT PROMOTION).
//!
//! Usage:
//!   cargo run -p figeometrica-rhetorica --bin sidang [-- --ci] [-- version N]
//!
//! Exit 0 = everything passes. `--ci` turns failures into exit code 1.

use figeometrica_core::{check_compatibility, run_protocol_auto, BindingVerdict};
use figeometrica_rhetorica::Rhetorica;
use std::path::PathBuf;

/// One knowledge-version manifest (subset used by this gate).
#[derive(serde::Deserialize)]
struct Manifest {
    version: u32,
    domains: Vec<Slot>,
    units: Vec<Slot>,
    scopes: Vec<Slot>,
    anchors: Vec<Slot>,
    payloads: Vec<Slot>,
    loci: Vec<Slot>,
    bindings: Vec<Binding>,
}

#[derive(serde::Deserialize)]
struct Slot {
    id: String,
}

#[derive(serde::Deserialize)]
struct Binding {
    domain_id: String,
    anchor_id: String,
    operation_id: String,
    payload_id: String,
    status: String,
}

/// Binding store over a JSON manifest — no database.
struct ManifestBindings<'m> {
    bindings: &'m [Binding],
}

impl figeometrica_core::BindingStore for ManifestBindings<'_> {
    fn lookup(
        &self,
        anchor: &str,
        payload: &str,
        operation: &str,
        domain: &str,
    ) -> BindingVerdict {
        match self
            .bindings
            .iter()
            .find(|b| {
                b.anchor_id == anchor
                    && b.payload_id == payload
                    && b.operation_id == operation
                    && b.domain_id == domain
            })
            .map(|b| b.status.as_str())
        {
            Some("valid") => BindingVerdict::Valid,
            Some("invalid") => BindingVerdict::Invalid,
            _ => BindingVerdict::Unknown,
        }
    }
}

/// Canonical version = highest `vN` folder number in data/knowledge/.
fn canonical_version(base: &std::path::Path) -> u32 {
    std::fs::read_dir(base)
        .ok()
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter_map(|e| {
                    let name = e.file_name().into_string().ok()?;
                    let n: u32 = name.strip_prefix('v')?.parse().ok()?;
                    Some(n)
                })
                .max()
                .unwrap_or(1)
        })
        .unwrap_or(1)
}

/// A claimed status is legitimate only when its evidence suffices.
fn status_supported(claim: &str, protocol_passed: bool, binding: BindingVerdict) -> bool {
    match claim {
        "EXTRACTED" | "UNDER_SPECIFIED" | "PROSE_ONLY" => true,
        "STRUCTURALLY_VALID" => binding != BindingVerdict::Invalid,
        "WITNESS_TESTED" | "INVERSE_VERIFIED" | "CONTRASTIVE_VERIFIED" | "USER_ACCEPTED"
        | "CANONICAL" => protocol_passed && binding != BindingVerdict::Invalid,
        // Side statuses (AMBIGUOUS/INVALID/...) are not judged here.
        _ => true,
    }
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mode_ci = args.iter().any(|a| a == "--ci");
    let forced_version: Option<u32> = args
        .iter()
        .position(|a| a == "version")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok());

    // ── Canonical manifest ───────────────────────────────────────────
    let kb = PathBuf::from("data").join("knowledge");
    let version = forced_version.unwrap_or_else(|| canonical_version(&kb));
    let manifest_path = kb.join(format!("v{version}")).join("manifest.json");
    let manifest_src = match std::fs::read_to_string(&manifest_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("FAILED to read {}: {e}", manifest_path.display());
            return std::process::ExitCode::FAILURE;
        }
    };
    let manifest: Manifest = match serde_json::from_str(&manifest_src) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("FAILED to parse manifest v{version}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    println!(
        "⚖️  Contract trial — knowledge v{} · {} bindings",
        manifest.version,
        manifest.bindings.len()
    );

    // ── Embedded dataset ─────────────────────────────────────────────
    let base = match Rhetorica::embedded() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("FAILED to load dataset: {e:?}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let mut checked = 0u32;
    let mut passed = 0u32;
    let mut n_unknown = 0u32;
    let mut failed: Vec<String> = Vec::new();

    for f in base.figures.iter() {
        let Some(sig) = &f.signature else { continue };
        checked += 1;
        let name = &f.name;
        let mut objections: Vec<String> = Vec::new();

        // 1. Slots ∈ manifest vocabulary.
        let required = [
            ("domain_id", &manifest.domains, sig.domain_id.clone()),
            ("unit_id", &manifest.units, sig.unit_id.clone()),
            ("anchor_id", &manifest.anchors, sig.anchor_id.clone()),
        ];
        for (label, list, value) in required {
            if !list.iter().any(|s| s.id == value) {
                objections.push(format!("{label} '{value}' not a member of manifest v{version}"));
            }
        }
        let optional = [
            ("scope_id", &manifest.scopes, &sig.scope_id),
            ("payload_id", &manifest.payloads, &sig.payload_id),
            ("locus_id", &manifest.loci, &sig.locus_id),
        ];
        for (label, list, value) in optional {
            if let Some(v) = value {
                if !list.iter().any(|s| s.id == *v) {
                    objections.push(format!("{label} '{v}' not a member of manifest v{version}"));
                }
            }
        }

        // 2. Bindings (CONTRACT §6).
        let store = ManifestBindings { bindings: &manifest.bindings };
        let verdict = check_compatibility(sig, &store);
        match verdict {
            BindingVerdict::Invalid => objections.push(format!(
                "binding INVALID for {}×{}×{}",
                sig.domain_id,
                sig.anchor_id,
                sig.operation.as_str()
            )),
            BindingVerdict::Unknown if sig.payload_id.is_some() => n_unknown += 1,
            _ => {}
        }

        // 3. Witness protocol re-run.
        let protocol_passed = match run_protocol_auto(sig) {
            Ok(report) => {
                if !report.passed {
                    objections.push(format!("witness protocol FAILED (inverse={:?})", report.inverse));
                }
                report.passed
            }
            // Outside deterministic reach: legal, but it automatically
            // voids any WITNESS_TESTED-and-above claim via step 4.
            Err(_) => false,
        };

        // 4. Ladder consistency (NO SILENT PROMOTION).
        let claim = f.epistemic.as_ref().map(|e| e.status.as_str()).unwrap_or("PROSE_ONLY");
        if !status_supported(claim, protocol_passed, verdict) {
            objections.push(format!("status '{claim}' unsupported by evidence"));
        }

        if objections.is_empty() {
            passed += 1;
        } else {
            for o in objections {
                failed.push(format!("{name}: {o}"));
            }
        }
    }

    // ── Report ───────────────────────────────────────────────────────
    println!(
        "signature-bearing figures: {checked} · fully passed: {passed} · binding-UNKNOWN recorded: {n_unknown}"
    );
    if failed.is_empty() {
        println!("✅ nothing failed the trial");
        std::process::ExitCode::SUCCESS
    } else {
        println!("\n❌ FAILED ({}):", failed.len());
        for f in &failed {
            println!("  · {f}");
        }
        if mode_ci {
            std::process::ExitCode::FAILURE
        } else {
            std::process::ExitCode::SUCCESS
        }
    }
}
