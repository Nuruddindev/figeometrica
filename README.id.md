# Figeometrica

*[English version](README.md) · Versi Indonesia*

**Figur itu geometris.** Setiap figur retorika — bila didefinisikan dengan
baik — adalah sebuah operasi atas deret: *operasi apa, pada jangkar mana,
atas satuan apa, diulang berapa kali*. Figeometrica mengubah tesis itu
menjadi infrastruktur yang tereksekusi.

Ini kerangka kompilasi-teori: teori-teori humaniora tentang gaya (mulai dari
retorika klasik — taksonomi analisis teks tertua di dunia) dikompilasi
menjadi spesifikasi terstruktur yang bisa diperiksa mesin, ditambah mesin
yang mengeksekusinya secara deterministik dan dapat diaudit.

**Mengapa ini penting?** Baca [manifestonya](MANIFESTO.id.md) — lima menit.

Tesis klasik —

```
figur = OPERASI × JANGKAR × SATUAN × PENGULANGAN
```

— terkompilasi, menurut [CONTRACT.md](CONTRACT.md), menjadi **signature**
yang tereksekusi; vocabulary slotnya hidup dalam manifest knowledge
berversi ([`data/knowledge/v1/manifest.json`](data/knowledge/v1/manifest.json)
adalah kanon hari ini):

```
FigureSignature = domain    ∈ {textual | conceptual | entity | argumentative}
                × unit      ∈ {grapheme … concept}
                × scope     ∈ {phonological-form … discourse}
                × anchor    ∈ {initial/final/medial-segment, insertion-point,
                               whole-unit, cross-boundary,
                               person | non-person | non-human | character}
                × operation ∈ {adjectio | detractio | immutatio |
                               transmutatio | repetitio}   ← set istilah beku
                × payload   ∈ {segment, letter, syllable, person, …}
                × locus     ∈ {initial, medial, terminal, response,
                               distributed, clustered, every,
                               cross_unit, alternating}
                [+ result, + constraints]
```

Contoh — `tmesis` ("abso-bloody-lutely"), seperti tersimpan di
[`data/figures/tmesis.json`](data/figures/tmesis.json):

```json
{
  "signature": {
    "domain_id": "textual",
    "unit_id": "word",
    "anchor_id": "insertion-point",
    "operation": "adjectio"
  },
  "epistemic": {
    "status": "STRUCTURALLY_VALID"
  },
  "geometry": {
    "anchor": "Insertion",
    "class": "Lexical",
    "grain": "word",
    "min_repeats": 1,
    "operation": "addition"
  }
}
```

## Crate

| Crate | Apa itu |
|---|---|
| [`figeometrica-core`](crates/core) | Format spesifikasi geometri (`FigurePattern`, `Anchor`, `ElementClass`, templat slot dengan kelas kesamaan) + matcher deterministik (`GeometryMatcher`) |
| [`figeometrica-pipeline`](crates/pipeline) | Pipeline analisis berjangkar provenance: chunk sadar-modalitas, trait tahap observasi/verifikasi LLM, temuan dengan bukti chunk+span |
| [`figeometrica-rhetorica`](crates/rhetorica) | Basis teori retorika klasik sebagai data: figur, spesifikasi geometri, kategori, loader |

## Prinsip desain

1. **Ontologi sebagai data, bukan prosa** — definisi dikompilasi ke spesifikasi
   formal; kriteria yang tak terpenuhi itu terhitung, jadi bukti negatif itu nyata.
2. **Deterministik bila mungkin, LLM bila perlu** — pencocokan geometri tidak
   pernah memanggil model; model mengamati fitur dan memverifikasi semantik,
   selalu dengan confidence dan status `indeterminate`.
3. **Provenance di mana-mana** — setiap temuan membawa `chunk_id + span`.
4. **Katalog dapat difalsifikasi** — definisi yang tak bisa ditulis dalam bentuk
   kanonik adalah definisi yang buruk, bukan "figur non-geometris".

## Status

Masih awal pengembangan. Matcher inti menguasai 9 pola (anaphora, epistrophe,
symploce, anadiplosis, gradatio/climax, antimetabole, chiasmus, tmesis,
parenthesis); basis teori retorika digeometrisasi bertahap.

## Ikut berkontribusi

447 dari 456 figur masih menunggu geometrinya dikompilasi — dan mesin
memeriksa pekerjaan Anda: setiap kontribusi membawa kalimat contoh yang
dijalankan CI lewat matcher deterministik. Tanpa kode; satu file JSON cukup.

**Cara berkontribusi (± 15 menit untuk figur pertama Anda):**

1. **Klaim satu figur** — buka issue dengan templat
   ["Geometrize a figure"](../../issues/new?template=geometrize-figure.md),
   atau pilih file mana pun di [`data/figures/`](data/figures) yang blok
   kontraknya belum ada (mis. `epizeuxis.json`).
2. **Isi blok `signature`** — semua slot wajib berasal dari manifest
   knowledge kanon (`data/knowledge/vN`, N tertinggi menang). Butuh slot
   yang belum ada? Usulkan versi knowledge baru lengkap dengan cerita apa
   yang membutuhkannya — lihat [CONTRIBUTING.id.md](CONTRIBUTING.id.md).
3. **Tetapkan klaim tangga di `epistemic`** — CI menjalankan ulang protokol
   witness sendiri: klaim status tanpa bukti gagal diperiksa.
   *No silent promotion.*
4. **Tambahkan contoh** — kalimat positif yang *wajib* memicu pola,
   near-miss negatif yang *wajib tidak* memicu. Inilah yang membuat entri
   Anda bisa diperiksa mesin.
5. **Periksa lokal**
   ```bash
   cargo run -p figeometrica-rhetorica --bin sidang -- --ci
   cargo run -p figeometrica-rhetorica --bin validate
   ```
6. **Buka PR** — CI memverifikasi otomatis: lulus = di-merge dengan nama
   Anda di field `attribution` entri; gagal = Anda menerima witness persis
   yang membuatnya gagal. Review adalah Meja Hakim kami — merge yang
   mengesahkan sebuah figur.

Pola di luar keluarga matcher saat ini (figur kelas konseptual seperti
chiasmus, sisipan seperti tmesis) tetap diterima — mereka dialihkan ke
review maintainer alih-alih verifikasi otomatis.

### Pasang di komputer Anda (opsional)

Anda sebenarnya tidak wajib pasang lokal — mengedit file lewat antarmuka web
GitHub dan membiarkan CI memvalidasi sudah cukup. Tapi menjalankan validator
lokal memberi umpan balik instan saat Anda mengerjakan contoh.

Persyaratan: [git](https://git-scm.com) dan Rust (stable terbaru mana pun).

```bash
# 1. Pasang Rust sekali (~5 menit)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Ambil proyeknya
git clone https://github.com/Nuruddindev/figeomatrica.git
cd figeomatrica

# 3. Pastikan semuanya jalan (harus berakhir "test result: ok")
cargo test --workspace

# 4. Saat berkontribusi: cek file figur Anda seketika
cargo run -p figeometrica-rhetorica --bin validate
```

Hanya itu — Rust murni, tanpa library sistem, tanpa akses jaringan saat
runtime. Pengguna Windows: pasang Rust lewat [rustup.exe](https://rustup.rs)
alih-alih perintah curl.

## Lisensi

MIT — lihat [LICENSE-MIT](LICENSE-MIT).
