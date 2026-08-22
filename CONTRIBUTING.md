# Contributing Guide · Panduan Berkontribusi

*One rule: every claim must carry evidence — and machines check it.*

*Satu aturan: setiap klaim harus berbukti — dan mesin yang memeriksanya.*

---

## 1 · Adding or fixing a figure = one JSON file

Everything lives in `data/figures/<name>.json`:

## 1 · Menambah / memperbaiki satu figur = satu file JSON

Semuanya ada di `data/figures/<nama>.json`:

```json
{
  "id": 91,
  "name": "apocope",
  "definition": "Cutting off final letter/syllable",
  "geometry":   { "...": "blok geometri warisan (opsional)" },
  "signature": {
    "domain_id": "textual",
    "unit_id": "word",
    "scope_id": null,
    "anchor_id": "final-segment",
    "operation": "detractio",
    "payload_id": null,
    "locus_id": null,
    "result": null,
    "constraints": {}
  },
  "epistemic": {
    "status": "WITNESS_TESTED"
  },
  "examples": {
    "positive": [["Photograph", "photo"]],
    "negative": [["The veterinarian examined the dog.",
                  "The vet examined the dog carefully and completely."]]
  }
}
```

### Rules for the `signature` block · Aturan blok `signature`
(CONTRACT.md §2, §12)

**EN** — Every slot must come from the canonical knowledge manifest
(`data/knowledge/vN/manifest.json`; highest N wins). Slots that don't
apply get `null`, never invented values. `scope` is not a junk drawer:
if unsure, leave it `null`.

**ID** — Semua slot wajib berasal dari manifest knowledge versi kanon
(`data/knowledge/vN/manifest.json`; N tertinggi adalah kanon). Slot yang
tidak relevan diisi `null`, bukan nilai karangan. `scope` bukan tempat
sampah: kalau tak yakin, biarkan `null`.

### Rules for the `epistemic` block · Aturan blok `epistemic`

| Your claim · Klaim Anda | What CI checks · Yang dicek CI |
|---|---|
| `EXTRACTED` | signature present & slots valid · ada & slot valid |
| `STRUCTURALLY_VALID` | + bindings not INVALID · bindings bukan INVALID |
| `WITNESS_TESTED` | + witness protocol re-run passes · protokol lulus ulang |
| `USER_ACCEPTED` / `CANONICAL` | + maintainer merge · merge oleh maintainer |

**EN** — Claiming a status without evidence fails CI with a message
explaining why. That's a feature, not a bug: *NO SILENT PROMOTION*.

**ID** — Mengklaim status tanpa bukti membuat PR gagal CI dengan pesan
yang menjelaskan kenapa. Itu fitur, bukan bug: *NO SILENT PROMOTION*.

### Prose definitions · Definisi prosa

**EN** — Write definitions in your own words. Text copied from
copyrighted sources will not be accepted.

**ID** — Tulis dengan kata-kata sendiri. Definisi yang disalin dari
sumber berhak cipta tidak diterima.

---

## 2 · Missing a vocabulary slot? Propose a knowledge version

**EN** — Never force an unrelated slot into place. Instead:

1. Copy the current highest version: `cp -r data/knowledge/v2 data/knowledge/v3`
2. Add your slot/binding in `v3/manifest.json`
3. Write `v3/README.md`: which slot, which figure demands it, and the
   experiment/example that revealed the need
4. Reference that folder in your PR

Old versions are never edited — they are the experiment log. Details in
[`data/knowledge/README.md`](data/knowledge/README.md).

**ID** — Jangan paksa slot lain menggantikan slot yang dibutuhkan.
Ajukan **versi knowledge baru**:

1. Salin versi tertinggi saat ini: `cp -r data/knowledge/v2 data/knowledge/v3`
2. Tambah slot/binding di `v3/manifest.json`
3. Tulis `v3/README.md`: slot apa, figur mana yang membutuhkan, dari
   eksperimen/contoh apa ditemukan
4. Rujuk folder itu di PR Anda

Versi lama tidak pernah diedit — mereka adalah rekam jejak eksperimen.
Detail: [`data/knowledge/README.md`](data/knowledge/README.md).

---

## 3 · Verify locally before pushing

## 3 · Verifikasi lokal sebelum push

```bash
cargo test --workspace
cargo run -q -p figeometrica-rhetorica --bin sidang -- --ci
cargo run -q -p figeometrica-rhetorica --bin validate
```

**EN** — If all three are green, CI will be green too.

**ID** — Kalau ketiganya hijau, CI juga akan hijau.

---

## 4 · Review is our Judgment Desk

**EN** — PRs that pass CI are reviewed by the maintainer. Merge is the act
of ratification: that's where a figure rises to `USER_ACCEPTED`, and only
this path reaches `CANONICAL`. The review history lives forever in the
PR thread — that thread *is* our ledger.

**ID** — PR yang lulus CI direview maintainer. Merge adalah tindakan
pengesahan: di situ figur naik ke `USER_ACCEPTED`, dan hanya lewat jalur
ini sebuah figur bisa mencapai `CANONICAL`. Riwayat review tersimpan
permanen di utas PR — utas itulah buku besar kami.
