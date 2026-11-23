# Seni Bina & Reka Bentuk Enjin MELAYU

Dokumen ini memperincikan saluran paip (*execution pipeline*) dan model reka bentuk dalaman bagi penterjemah bahasa pengaturcaraan **MELAYU**.

## Saluran Paip Kompilasi & Pelaksanaan

```mermaid
flowchart LR
    KodSumber["Kod Sumber (.ml)"] --> Pengimbas["Pengimbas (Lexer)"]
    Pengimbas --> SenaraiToken["Aliran Token (Tokens)"]
    SenaraiToken --> Penghurai["Penghurai (Parser)"]
    Penghurai --> AST["Pokok Sintaks Abstrak (AST)"]
    AST --> Penganalisis["Penganalisis Semantik (Analyzer)"]
    Penganalisis --> Enjin["Jurubahasa Runtime (Interpreter)"]
```

### 1. Pengimbas Leksikal (`src/lexer.rs` & `src/tokens.rs`)
* Mengimbas bait demi bait rentetan aksara UTF-8 kod sumber.
* Mengasingkan kata kunci serumpun (`ada`, `kerja`, `jika`, `kelas`, `bina`, `diri`, `warisi`, `induk`).
* Menyokong kata kunci gabungan (`jika tidak jika`, `baca nombor`, `sama dengan`, `lebih sama`).
* Mengira kedudukan baris dan lajur (`LineCol`) bagi pemetaan ralat mendidik.
* Mengesan kesilapan ejaan biasa (*typo detection*) seperti `class` -> cadangkan `kelas`, `print` -> cadangkan `tunjuk`.

### 2. Penghurai Sintaks (`src/parser.rs` & `src/ast.rs`)
* Menggunakan algoritma *Recursive Descent Parsing* berkeutamaan pengendali (*Precedence Climbing / Pratt Parsing*).
* Menghasilkan nod AST (*Abstract Syntax Tree*) berstruktur kuat (`Stmt` dan `Expr`).
* Membungkus nod deklarasi kelas dalam `Box<DeklarasiKelasAST>` untuk menjimatkan saiz tindanan memori hos.

### 3. Penganalisis Semantik (`src/analyzer.rs`)
* Mengesahkan skop pembolehubah dan keterlihatan simbol sebelum pelaksanaan dijalankan.
* Mengesan pembolehubah tidak wujud, pengisytiharan semula simbol tetap (`tetap`), dan pengesanan gelung.
* Mengesahkan hierarki kelas induk semasa pewarisan (`warisi`).

### 4. Enjin Jurubahasa & Model Memori (`src/interpreter.rs` & `src/builtins.rs`)
* **Pengurusan Skop (`Environment`)**: Menggunakan struktur pokok skop bertingkat dengan penuding rujukan rantaian `Rc<RefCell<Environment>>`.
* **Objek Berorientasi (`Value::Objek` & `Value::Kelas`)**:
  - `Rc<KelasData>`: Menyimpan definisi kelas, peta kaedah, dan fungsi pembina.
  - `Rc<ObjekData>`: Menyimpan sifat instans dalam `Rc<RefCell<HashMap<String, Value>>>` yang membolehkan mutasi sifat dinamik secara selamat.
* **Perlindungan Keselamatan Hos**:
  - Had kedalaman rekursi: Menghalang *native stack overflow* melalui pemantauan `self.kedalaman_panggilan >= self.had_rekursi`.
  - Had langkah lelaran: Menamatkan gelung liar (*runaway loops*) melebihi had langkah selamat.
  - Sandbox I/O fail: Menyekat *path traversal* (`../`) dan mengehadkan saiz fail maksimum 5 MB.
