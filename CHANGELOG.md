# Changelog

Semua perubahan ketara pada projek ini akan didokumenkan dalam fail ini mengikut format [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.1] - 2026-11-16

### Keselamatan & Pengukuhan Enjin (Security & Hardening)
- **Perlindungan Rekursi Melampau (CWE-674)**: Menetapkan had kedalaman rekursi fungsi maksimum (`1000`) untuk mengelakkan *native stack overflow* dan *crash* pada proses hos.
- **Pencegahan 'Runaway Loop' / DoS (CWE-400)**: Pengira langkah pelaksanaan selamat (`10,000,000` langkah lalai) untuk menamatkan gelung tak terhingga (*infinite loop*) secara anggun.
- **Pengesahan Indeks Ketat (Strict Index Validation)**:
  - Menolak percubaan indeks bukan terhingga (`NaN`, `Infiniti`) dan pecahan/perpuluhan (`s[1.5]`) dengan mesej ralat `MelayuError` yang ramah.
  - Memperbaiki pengiraan indeks negatif pada senarai dan rentetan teks bagi menghalang *underflow*.
- **Sandbox Operasi Fail Selamat**:
  - Fungsi terbina-dalam `bacaFail`, `tulisFail`, `wujudFail`, dan `padamFail` disaring dengan pemeriksaan *Path Traversal* (menolak `..` dan laluan mutlak di luar direktori kerja).
  - Had saiz bacaan dan penulisan fail maksimum 5 MB bagi mengelakkan keletihan memori (*Out Of Memory* / OOM).
- **Penjana Nombor Rawak Berentropi Tinggi (`rawak`)**: Fungsi `rawak()` dan `rawak(min, max)` berasaskan algoritma SplitMix64 tanpa kebergantungan luar.
- **Had Memori Cantuman**: Pemeriksaan saiz maksimum 16 MB untuk cantuman teks dan 1,000,000 elemen untuk cantuman senarai.
- **Suite Ujian Keselamatan**: Menambah fail ujian khas `tests/security_test.rs` dengan 6 ujian keselamatan lengkap (jumlah 20 ujian lulus).

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

