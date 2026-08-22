*[English version](CONTRIBUTING.md) · Versi Indonesia*

# Berkontribusi ke Figeometrica

Terima kasih! Panduan ini menjelaskan cara menyumbang **signature
geometris** — spesifikasi terstruktur yang membuat sebuah figur retoris
dapat dideteksi dan diaudit mesin. Anda tidak perlu menulis kode; satu
file JSON + contoh kalimat sudah cukup.

Aturan mainnya satu kalimat: **klaim harus berbukti.** Gerbangnya jalan
di CI setiap PR, dan versi lokalnya sama persis.

## Tesis dalam satu baris

> Setiap figur, bila didefinisikan dengan baik, adalah operasi atas deret:
> **operasi × jangkar × satuan × pengulangan** — kini dieksekusi sebagai
> `FigureSignature` yang diperiksa mesin.

## Anatomi satu file figur

Buka `data/figures/<nama>.json`. Strukturnya:

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

### Aturan blok `signature` (CONTRACT.md §2, §12)

- Semua slot **wajib** berasal dari manifest knowledge versi kanon:
  `data/knowledge/vN/manifest.json` — N tertinggi adalah kanon.
- Slot yang tidak relevan: `null`, bukan nilai karangan.
- `scope` bukan tempat sampah: kalau tak yakin, biarkan `null`.

### Aturan blok `epistemic` (tangga status)

| Klaim Anda | Yang dicek CI |
|---|---|
| `EXTRACTED` | signature ada & slot valid |
| `STRUCTURALLY_VALID` | + bindings bukan INVALID |
| `WITNESS_TESTED` | + protokol witness lulus (CI menjalankan ulang) |
| `USER_ACCEPTED` / `CANONICAL` | + merge oleh maintainer |

Mengklaim status tanpa bukti = PR gagal CI dengan pesan yang menjelaskan
kenapa. Itu fitur, bukan bug: *NO SILENT PROMOTION*.

## Aturan contoh (bagian terpenting)

- **Positif** HARUS memicu pola; **negatif** HARUS TIDAK memicu (mirip
  tapi bukan figur itu).
- Tiap contoh = array unit wacana (kalimat). Figur lintas-unit (anaphora,
  anadiplosis, climax) butuh beberapa unit; antimetabole cukup satu.
- Protokol witness dijalankan ulang oleh CI atas signature Anda. Lulus =
  klaim Anda terbukti; gagal = CI memberi tahu persis langkah mana yang
  gugur.
- Pola di luar keluarga matcher saat ini (mis. domain konseptual seperti
  chiasmus) tetap diterima — mereka menunggu jalur konstruktor untuk naik
  tangga secara berbukti.

### Definisi prosa

Tulis dengan kata-kata sendiri. Definisi yang disalin dari sumber
berhak cipta tidak diterima.

## Slot vocabulary yang dibutuhkan belum ada? Usulkan versi knowledge baru

Jangan paksa slot lain menggantikan slot yang dibutuhkan:

1. Salin versi tertinggi saat ini: `cp -r data/knowledge/v2 data/knowledge/v3`
2. Tambah slot/binding di `v3/manifest.json`
3. Tulis `v3/README.md`: slot apa, figur mana yang membutuhkan, dari
   eksperimen/contoh apa ditemukan
4. Rujuk folder itu di PR Anda

Versi lama tidak pernah diedit — mereka adalah rekam jejak eksperimen.
Detail: [`data/knowledge/README.md`](data/knowledge/README.md).

## Verifikasi lokal sebelum push

```bash
cargo test --workspace
cargo run -q -p figeometrica-rhetorica --bin sidang -- --ci
cargo run -q -p figeometrica-rhetorica --bin validate
```

Kalau ketiganya hijau, CI juga akan hijau.

## Review = Meja Hakim

PR yang lulus CI direview maintainer. Merge adalah tindakan pengesahan:
di situ figur naik ke `USER_ACCEPTED`, dan hanya lewat jalur ini sebuah
figur bisa mencapai `CANONICAL`. Riwayat review tersimpan permanen di
utas PR — utas itulah buku besar kami.

## Lisensi, atribusi & co-authorship

- Kontribusi dilisensikan **MIT** sejak dibuka PR-nya (inbound = outbound).
- Nama Anda tersimpan di field `attribution` entri + CONTRIBUTORS.md.
- Kontributor dengan **≥ 10 entri diterima** atau berperan sebagai
  validator masuk daftar co-author publikasi dataset. Kriteria final
  diumumkan sebelum paper ditulis dan tidak berlaku surut.
