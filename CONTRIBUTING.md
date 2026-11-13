# 🤝 Panduan Menyumbang (Contributing Guide)

Terima kasih kerana berminat untuk menyumbang kepada pembangunan **MELAYU**! Projek ini adalah usaha bersama untuk memperkasakan pembangun di rantau Nusantara.

---

## 🛠️ Langkah Bermula

### 1. Keperluan Sistem
- Rust 1.75 atau lebih baharu ([rustup.rs](https://rustup.rs/))
- Git

### 2. Fork & Klon Repositori
```bash
git clone https://github.com/rillardian655/Melayu-Language-by-runa.git
cd Melayu-Language-by-runa
```

### 3. Membina & Menguji Projek
Sebelum membuat sebarang perubahan, pastikan semua ujian sedia ada lulus:

```bash
cargo build
cargo test
```

Semak pemformatan dan linting:
```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

---

## 🌿 Aliran Kerja Git (Git Workflow)

1. Cipta cawangan (branch) baharu dengan nama yang deskriptif:
   ```bash
   git checkout -b feat/tambah-kaedah-rentetan
   # atau untuk pembaikan pepijat:
   git checkout -b fix/ralat-pembahagian-sifar
   ```
2. Lakukan perubahan kod berserta ujian unit baharu di folder `tests/`.
3. Format kod sebelum commit:
   ```bash
   cargo fmt
   ```
4. Lakukan commit dengan mesej mengikut konvensyen *Conventional Commits*:
   - `feat: sokongan kaedah rentetan .pecah()`
   - `fix: betulkan kiraan lajur semasa ralat lexer`
   - `docs: kemas kini panduan sintaks kamus`
   - `test: tambah ujian integrasi gelung julat negatif`
5. Tolak (push) ke repositori fork anda dan buka Pull Request (PR) ke cawangan `main`.

---

## 📐 Garis Panduan Kod

- **Kod Sumber & Pengulasan**: Pengkodan enjin dalam Rust ditulis dengan gaya idiomatik standard Rust.
- **Mesej Ralat**: Mesej ralat yang dipaparkan kepada pengguna akhir MESTI dalam bahasa Melayu yang ramah, menghormati penutur serumpun, dan mengandungi cadangan penyelesaian (`with_suggestion`).
- **Ujian Baharu**: Setiap penambahan sintaks atau ciri baharu wajib disertakan sekurang-kurangnya 1 ujian unit atau integrasi.

