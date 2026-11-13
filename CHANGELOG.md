# Changelog

Semua perubahan ketara pada projek ini akan didokumenkan dalam fail ini mengikut format [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2026-11-13

### Ditambah (Added)
- **Lexer**: Tokenizer penuh untuk sintaks bahasa Melayu termasuk kata kunci gabungan (`jika tidak jika`, `sama dengan`, `lebih sama`, `kurang sama`, `baca nombor`).
- **Parser**: Penghurai rekursif (Recursive Descent) menghasilkan Abstract Syntax Tree (AST) yang lengkap.
- **Semantic Analyzer**: Pengesahan skop pembolehubah, pengesanan awal pembolehubah tidak wujud, pengesanan pembolehubah bertindih, dan perlindungan pembolehubah malar (`tetap`).
- **Tree-Walking Interpreter**: Enjin perlaksanaan AST dinamik dengan sokongan penutupan leksikal (closures) dan rekursi penuh.
- **Sistem Pengendalian Ralat**: Blok `cuba ... tangkap <ralat> ... akhirnya ... selesai` serta `baling <ralat>`.
- **Mesej Ralat Ramah**: Format ralat mesra komuniti ("Woy!", "Alamak!", "Aduh!", "Eh!") lengkap dengan baris sumber, penunjuk lajur (`^`), dan cadangan pembetulan (Levenshtein/typo suggestion).
- **Jenis Data Asas**: `teks`, `nombor`, `betul`, `salah`, `kosong`.
- **Struktur Data Terbina-Dalam**: `senarai` (dengan `.tambah()`, `.buang()`, `.panjang()`, `.berisi()`) dan `kamus` (dengan `.kunci()`, `.nilai()`, `.panjang()`).
- **I/O Terminal**: Pernyataan `tunjuk` serta ungkapan input `baca()` dan `baca nombor()`.
- **REPL Interaktif**: Konsol baris arahan interaktif dengan sokongan multi-baris dan output berwarna.
- **CLI**: Utiliti `melayu` menyokong arahan `jalankan`, `semak`, `bantuan`, dan `versi`.
- **Suite Ujian**: 14 ujian unit dan integrasi meliputi lexer, parser, analyzer, dan interpreter.
- **Aliran Kerja CI/CD**: Tindakan GitHub Actions untuk pemeriksaan binaan multi-OS (Linux, macOS, Windows) dan automasi rilis.
- **Contoh Program**: 5 program contoh siap dijalankan (`hello.ml`, `fizzbuzz.ml`, `kalkulator.ml`, `fibonacci.ml`, `tebak_angka.ml`).

