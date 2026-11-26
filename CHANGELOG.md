# Changelog

Semua perubahan ketara pada projek ini akan didokumenkan dalam fail ini mengikut format [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Belum Dikeluarkan]

### Ditambah (Added)
- Pilihan `--had-langkah <n>` dan pemboleh ubah persekitaran `MELAYU_HAD_LANGKAH` untuk menaikkan atau mematikan (`0`) had 10 juta langkah.
- `benihRawak(n)` untuk keputusan `rawak()` dan `rawakTensor()` yang boleh diulang.
- `cariPecahanTerbaik(X, y, senaraiCiri)`: carian pecahan Gini asli yang digunakan oleh `PokokKeputusan`.
- `MELAYU_GPU=0` memaksa pendaraban matriks menggunakan CPU. `MELAYU_GPU=fp32` memilih FP32 pada GPU: lebih laju, tetapi hasil hanya tepat kira-kira 7 digit. Lalai kekal FP64.
- Nombor dalam notasi saintifik (`1e21`, `5.9e-05`).
- Ungkapan boleh disambung ke baris seterusnya selepas operator (`+`, `dan`, `==`, dll.).
- Kamus boleh memanggil fungsi yang disimpan di dalamnya (`Modul.fungsi()` selepas `ambil ... sebagai Modul`).

### Diubah (Changed)
- `kamus` dan sifat objek kini mengekalkan susunan kunci mengikut urutan masuk; `tulisJSON` mengikut susunan yang sama.
- `infoPerkakas()` memaparkan `ketepatan_gpu`.
- Senarai nama fail sensitif diperluas (`.aws`, `.npmrc`, `.netrc`, `.docker`, `.kube`, `.gnupg`, `.pypirc`, `.git-credentials`).
- `HutanRawak` menimbang `sqrt(bilangan ciri)` ciri rawak pada setiap pecahan.
- `.isih()` menolak senarai yang mengandungi jenis bercampur.
- `muatIris()` kini memulangkan kesemua 150 rekod dataset Iris (sebelum ini 30).

### Dibaiki (Fixed)
- Proses tamat dengan stack overflow apabila memaparkan, membandingkan atau melepaskan nilai yang merujuk diri sendiri atau sangat dalam, atau apabila kod sangat bersarang.
- Sandbox fail: pautan simbolik (fail, folder atau rosak, dengan laluan relatif atau mutlak) tidak lagi boleh membaca, menulis, memadam atau mengimport di luar direktori kerja, atau mencapai fail sensitif seperti `.env`.
- Semakan anti-SSRF boleh dipintas dengan bentuk alamat lain (`2852039166`, `0xa9fea9fe`, IPv4 terbenam dalam IPv6, NAT64, 6to4 dan Teredo).
- Hasil darab matriks yang melebihi had tensor kini memberi ralat dan tidak lagi menamatkan proses.
- `PengelasBayesNaif` menggunakan log ketumpatan dan pelicinan varians, dan kini memberi ramalan yang sama dengan scikit-learn pada iris, wine dan breast cancer.
- Ralat dalam fungsi modul menunjukkan baris dan nama modul yang betul.
- Panik semasa memaparkan ralat pada baris yang sangat panjang.
- Templat `melayu mula --mvc` gagal dijalankan.
- `bahagiLatihUji` dan `pengesahanSilang` tidak mengocok data, jadi pada dataset yang tersusun mengikut kelas (seperti Iris) set ujian hanya mengandungi satu kelas dan skor K-Fold menjadi 0.
- Prestasi: badan fungsi dan data tensor tidak lagi disalin pada setiap bacaan (fib(30) kira-kira 2.7x lebih pantas).



## [0.3.0] - 2026-11-23

### Ditambah (Added)
- **Sistem Pengaturcaraan Berorientasikan Objek (OOP) Natif**:
  - `kelas <Nama> ... selesai`: Pengisytiharan struktur kelas.
  - `bina(<parameter>) ... selesai`: Pembina objek (*constructor*) dengan pemulaan sifat instans.
  - Instansiasi objek melalui pemanggilan kelas: `ada obj = NamaKelas(<hujah>)`.
  - Kaedah instans (*methods*): Menggunakan blok `kerja <nama>() ... selesai` di dalam kelas.
  - Kata kunci rujukan instans `diri` (`this` / `self`): Mengakses dan mengubah sifat objek (`diri.nama = nama`).
  - Kata kunci pewarisan `warisi` (`extends`): Mewarisi kaedah dan struktur daripada kelas induk (`kelas Anak warisi Induk`).
  - Kata kunci rujukan induk `induk` (`super`): Memanggil pembina induk (`induk.bina(...)`) dan memanggil kaedah induk (`induk.kaedah(...)`).
  - Sifat dinamik: Membaca dan menetapkan sifat objek luaran secara langsung dengan sintaks titik (`obj.sifat = nilai`) atau pengindeksan rentetan (`obj["sifat"] = nilai`).
- **Integrasi Fungsi Terbina-Dalam**:
  - `jenis(obj)`: Mengembalikan nama kelas objek (contoh: `"Pengguna"`, `"Pentadbir"`).
  - `tulisJSON(obj)`: Menyokong serialisasi automatik sifat instans objek terus ke format JSON standard (*pretty* atau *compact*).
- **Pengoptimuman Memori Aras Enjin (Zero Stack Overflow)**:
  - Mengubah penyimpanan memori data kelas dan objek kepada `Rc<KelasData>`, `Rc<ObjekData>`, `Rc<IndukData>`, dan `Box<DeklarasiKelasAST>`.
  - Mengurangkan saiz jejak stack frame sebanyak lebih 60%, memastikan pelaksanaan rekursi berantai kekal pantas dan bebas limpahan tindanan.
- **Ekstensi VS Code v0.3.0**:
  - Pewarnaan sintaks tatabahasa TextMate menyokong kata kunci OOP: `kelas`, `bina`, `warisi`, `diri`, dan `induk`.
  - Snippets pintar baharu untuk VS Code: `kelas`, `kelaswarisi`, dan `bina`.
- **Suite Ujian & Contoh Program**:
  - Ujian integrasi OOP menyeluruh di `tests/interpreter_test.rs` (jumlah keseluruhan 29 ujian lulus 100%).
  - Atur cara contoh baharu `examples/oop_kelas_dan_objek.ml` memperagakan kelas kenderaan, pewarisan kereta elektrik, sifat dinamik, dan eksport JSON.

## [0.2.0] - 2026-11-23

### Ditambah (Added)
- **Operasi Fungsian Senarai (Higher-Order Functions)**:
  - `.petakan(fungsi)` (Map): Mengubah setiap elemen senarai kepada hasil pemanggilan fungsi penukar.
  - `.tapis(fungsi)` (Filter): Menyaring elemen senarai yang mengembalikan nilai benar (`betul`).
  - `.kiraSemua(fungsi, awal)` (Reduce / Fold): Mengumpul elemen senarai menjadi satu nilai terakumulasi.
  - `.cari(fungsi)` (Find): Mencari elemen pertama yang menepati syarat fungsi carian.
  - `.indeks(item)` (IndexOf): Mencari kedudukan indeks elemen atau mengembalikan `-1` jika tiada.
  - `.setiapSatu(fungsi)` (ForEach): Menjalankan callback bagi setiap elemen senarai secara berurutan.
- **Enjin Format JSON Asli Selamat (Zero-Dependency JSON Engine)**:
  - `bacaJSON(teks)` / `dariJSON(teks)`: Menghurai teks JSON standard secara rekursif kepada struktur objek MELAYU (`kamus`, `senarai`, `teks`, `nombor`, `betul`, `salah`, `kosong`).
  - Dilengkapi perlindungan limpahan kedalaman struktur JSON terhad maksimum 500 tingkat.
  - `tulisJSON(nilai, cantik?)` / `keJSON(nilai, cantik?)`: Menukar objek data MELAYU kepada format rentetan JSON standard dengan pilihan pemformatan kemas (*pretty-print*).
- **Utiliti Tarikh & Waktu Kalendar Sivil Gregory**:
  - `capMasa()`: Mengembalikan cap masa Unix semasa (saat).
  - `masaKini()`: Mengembalikan masa jam sistem semasa dalam format `"HH:MM:SS"`.
  - `tarikhKini()`: Mengembalikan tarikh kalendar sivil Gregory semasa dalam format `"YYYY-MM-DD"`.
- **Kaedah Manipulasi Teks & Kamus Baharu**:
  - Teks: `.ulang(n)`, `.selitDepan(panjang, aksara?)`, `.selitBelakang(panjang, aksara?)`, `.hurufAwalBesar()`.
  - Kamus: `.adaKunci(kunci)`, `.buang(kunci)`.
- **Ekstensi VS Code v0.2.0**:
  - Kemas kini tatabahasa TextMate menyokong semua fungsi terbina-dalam baharu.
  - Penambahan snippets: `bacajson`, `tulisjson`, `petakan`, `tapis`, `kirasemua`, `capmasa`, `masakini`.
- **Suite Ujian Menyeluruh**:
  - Penambahan ujian fungsian senarai, kamus, teks lanjutan, enjin JSON, dan fungsi masa di `tests/interpreter_test.rs` (jumlah keseluruhan 28 ujian lulus 100%).
- **Program Contoh Baharu**:
  - `examples/pemprosesan_data_dan_json.ml` memperagakan pemprosesan pesanan jualan dengan operasi fungsian, manipulasi teks berformat, dan serialisasi JSON.

## [0.1.2] - 2026-11-16

### Ditambah (Added)
- **Sistem Modul & Import Multi-Fail (`ambil`)**:
  - Menyokong import fail penuh: `ambil "matematik.ml"` atau `ambil "matematik"`.
  - Menyokong import ruang nama sebagai kamus: `ambil "matematik.ml" sebagai mat`.
  - Menyokong import simbol terpilih: `ambil kiraDiskaun, adakahGenap daripada "matematik.ml"`.
  - Dilindungi sepenuhnya oleh benteng keselamatan sandbox anti-traversal.
- **Ekstensi Rasmi VS Code (`editors/vscode`)**:
  - Definisi tatabahasa TextMate (`melayu.tmLanguage.json`) untuk pewarnaan sintaks lengkap.
  - Konfigurasi autolengkap tanda kurung, lipatan kod, dan aturan indentasi pintar (`language-configuration.json`).
  - Koleksi snippets siap guna untuk semua struktur blok kod (`snippets/melayu.json`).
- **Alat Manipulasi Teks & Koleksi Lanjutan**:
  - Kaedah rentetan teks baharu: `.pecah(pemisah)`, `.ganti(lama, baru)`, `.mulaDengan(awalan)`, `.tamatDengan(akhiran)`, `.mengandungi(subteks)`.
  - Kaedah senarai baharu: `.isih()` (mengisih turutan), `.balik()` (membalikkan susunan), `.cantum(pemisah)` (menggabungkan senarai menjadi rentetan teks).
  - Utiliti matematik terbina-dalam baharu: `bundar()`, `lantai()`, `siling()`, `mutlak()`, `maksimum()`, `minimum()`.
- **Contoh Program Baharu**: `examples/matematik_modul.ml` dan `examples/demo_modul.ml`.
- **Ujian Unit & Integrasi**: Menambah ujian integrasi modul dan kaedah rentetan/koleksi (jumlah 23 ujian lulus).

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

