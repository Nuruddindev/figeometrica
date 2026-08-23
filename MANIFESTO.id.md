# Manifesto Figeometrica

*[English version](MANIFESTO.md) · Versi Indonesia*

*Figur itu geometris — dan begitu terlihat, cara kita menganalisis teks berubah bentuk.*

---

## 1 · Masalah

Dua puluh empat abad silam, Aristoteles menyistematisasi seni meyakinkan.
Retorikus Romawi mengkatalogkan ornamen tutur; buku sekolah Renaisans
membiasakan para murid dengan ratusan figur. Kita mewarisi kurang lebih 456
figur bahasa yang bernama — **taksonomi analisis teks tertua yang pernah
ada**, terus diasah dari zaman kuno hingga Renaisans.

Dan semuanya didefinisikan dalam prosa.

Prosa tidak bisa dieksekusi. Tanyakan ke ruangan penuh peneliti NLP berapa
banyak teks dalam korpus yang memuat anafora — tak seorang pun bisa menjawab
tanpa membaca satu per satu. Tanyakan bagian mana yang meningkat menuju
klimaks, yang Anda dapat adalah opini. Metode komputasional modern
melewatkan lapisan ini sama sekali: Rhetorical Structure Theory sengaja
membuang bentuk permukaan demi relasi semantik; stilometri mereduksi gaya
menjadi statistik kata fungsi; model bahasa besar bisa *meniru* gaya tetapi
tidak bisa *mengauditnya* — tanyakan mengapa sebuah paragraf terasa
berirama, yang turun adalah kesan-kesanan.

Akibatnya: teori tertua dan tersaring terbaik tentang bagaimana teks
dibentuk duduk menganggur, tak tersentuh mesin.

---

## 2 · Wawasan

Baca definisinya dengan teliti, dan definisi itu membocorkan algoritma.

Anafora: *"pengulangan kata yang sama di awal klausa-klausa berturutan."*
Itu bukan prosa yang menyamar jadi definisi — itu operasi: sisipkan token
yang sama pada **jangkar awal** unit-unit berturutan, diulang minimal dua
kali. Antimetabole: balikkan sebuah frasa — permutasi. Tmesis: belah sebuah
kata dan sisipkan kata lain di dalamnya — aditio pada satuan grafem.
Kiasmus: balikkan dua peran konseptual lintas giliran — permutasi atas makna,
bukan kata.

Orang Romawi sudah tahu. Empat *operae* mereka — **adjectio** (penambahan),
**detractio** (penghapusan), **immutatio** (penggantian), **transmutatio**
(permutasi) — ditambah repetisi, adalah set operator yang lengkap. Setiap
figur dalam katalog adalah parameterisasi dari operasi-operasi itu: titik
jangkar, satuan, jumlah ulangan, kadang sebuah templat slot.

Maka kami nyatakan tesisnya apa adanya:

> Definisi figur adalah algoritma yang belum dikompilasi. Dan definisi yang
> *tidak bisa* ditulis sebagai operasi semacam itu bukan "figur non-geometris"
> — melainkan definisi yang ditulis buruk.

Untuk pertama kalinya dalam sejarah panjangnya, seluruh katalog 456 figur
itu menjadi dapat difalsifikasi.

---

## 3 · Langkahnya

Kompilasilah.

Setiap definisi ditulis ulang ke dalam bentuk kanonik. Tesis klasik —

> figur = OPERASI × JANGKAR × SATUAN × PENGULANGAN

— terkompilasi, menurut [CONTRACT.md](CONTRACT.md), menjadi signature yang
tereksekusi:

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

`tmesis` ("abso-bloody-lutely") menjadi:

```json
{
  "domain_id": "textual",
  "unit_id": "word",
  "anchor_id": "insertion-point",
  "operation": "adjectio"
}
```

Vocabulary slot tidak dikodekan mentah di mana pun — ia hidup dalam manifest
knowledge berversi ([`data/knowledge/vN`](data/knowledge/README.md)). Versi
tertinggi adalah kanon; versi lama tetap abadi sebagai rekam eksperimen.
Ketika sebuah penemuan tak muat di slot lama, Anda mengusulkan `vN+1`
bersama cerita apa yang membutuhkannya — *no silent promotion*, termasuk
untuk vocabulary.

Begitu terkompilasi, segalanya berubah:

- **Deteksi bersifat deterministik.** Matcher tidak pernah memanggil model.
  Diberi teks, ia menemukan polanya atau tidak — dengan rentang bukti
  presisi-byte.
- **Katalog bisa di-query.** "Figur apa yang menutup sebuah pidato?" →
  saring jangkar Akhir. "Apa yang bisa meningkat menuju puncak?" → gradatio.
  Semua itu sebelum dokumen mana pun dianalisis.
- **Kontribusi dicek mesin.** Setiap entri membawa contoh kalimat positif
  dan negatif; CI menjalankan matcher atas contoh itu. Kontributor tidak
  mungkin mengajukan spesifikasi yang bertentangan dengan contohnya sendiri.
- **Bukti negatif menjadi nyata.** "Tidak ada kiasmus di paragraf ini"
  berhenti menjadi kesan dan menjadi klaim yang bisa diperiksa.

---

## 4 · Kenapa ini penting

**Bagi NLP dan humaniora komputasional:** inilah jembatan yang hilang antara
stilistik klasik dan komputasi. Penelusuran berdasarkan fungsi retoris —
"temukan teks yang membangun momentum", "temukan bagian yang mengalah sebelum
membantah" — alih-alih penelusuran berbasis kata kunci. Analisis gaya dengan
provenance, bukan kesan-kesanan.

**Bagi penulis dan pengajar:** figur berhenti menjadi hafalan dan menjadi
gerakan yang bisa dilihat, dinamai, dilatih. Pidato murid bisa diperiksa:
apakah dibuka dengan struktur paralel? Apakah meningkat ke puncak? Di mana
ditutup? Gaya menjadi bisa diajarkan karena menjadi terlihat.

**Bagi sistem AI:** pipeline hibrida tempat geometri menjadi lapisan bukti
yang deterministik, dan model bahasa melakukan apa yang memang ia kuasai —
interpretasi — di atas bukti yang tidak bisa ia palsukan. Setiap temuan
dapat diaudit sampai ke offset byte-nya.

**Bagi humaniora secara luas:** sebuah demonstrasi bahwa teori menjadi
kumulatif dan teruji bila dikompilasi. Bukan dengan merekayasa menjadi
angka, melainkan dengan mengambil klaim strukturalnya cukup serius untuk
dieksekusi.

---

## 5 · Bingkai besar

Retorika adalah pilotnya, bukan batasnya.

Kerangkanya — basis teori sebagai data berversi, format kompilasi kanonik,
mesin deterministik, kontribusi massal yang divalidasi mesin — berlaku untuk
teori humaniora mana pun yang klaimnya memiliki struktur. Fallaciae datang
berikutnya: *apparent enthymeme* Aristoteles, argumen yang tampak sah namun
tidak, menunggu perlakuan yang sama. Lalu prosodi, skema argumen, gerakan
naratif.

Mesin tidak menggantikan retorikus. Mesin memberi pengamatan-pengamatan
tertua mereka tubuh yang tereksekusi — agar apa yang ditemukan secara manual
selama dua puluh empat abad akhirnya bisa diverifikasi secara skala.

---

*447 figur menunggu. Pilih satu, kompilasi, biarkan mesin memeriksa
pekerjaanmu.*
