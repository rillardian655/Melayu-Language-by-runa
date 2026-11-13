<div align="center">

```text
  __  __ _____ _        _ __   ___   _ 
 |  \/  | ____| |      / \\ \ / / | | |
 | |\/| |  _| | |     / _ \\ V /| | | |
 | |  | | |___| |___ / ___ \| | | |_| |
 |_|  |_|_____|_____/_/   \_\_|  \___/ 
```

# 🌴 MELAYU

**Bahasa programming serumpun, untuk serumpun developer.**

[![CI](https://github.com/rillardian655/Melayu-Language-by-runa/actions/workflows/ci.yml/badge.svg)](https://github.com/rillardian655/Melayu-Language-by-runa/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

[Pengenalan](#-pengenalan) • [Pemasangan](#-pemasangan) • [Contoh Cepat](#-contoh-cepat) • [Dokumentasi](#-dokumentasi) • [Roadmap](#-roadmap) • [Sumbangan](#-sumbangan)

</div>

---

## 📖 Pengenalan

**MELAYU** (`.ml`) adalah bahasa pengaturcaraan sumber terbuka bertaraf pengeluaran (production-ready) yang direka khas untuk **pembangun berlatarbelakangkan bahasa Melayu** — Indonesia, Malaysia, Brunei, Singapura, dan sesiapa sahaja yang lebih selesa berfikir dalam bahasa ibunda.

Tujuannya jelas dan tegas: **belajar & membina pengaturcaraan tanpa halangan perbendaharaan kata asing.**

Kebanyakan bahasa pengaturcaraan memaksa pemula memahami dua beban kognitif sekali gus:
1. Konsep asas pengaturcaraan (pembolehubah, gelung, fungsi, skop)
2. Kosa kata bahasa Inggeris (`while`, `function`, `return`, `throw`, dsb.)

MELAYU mengasingkan kedua-duanya. Anda fokus sepenuhnya kepada **logika**, bahasanya menggunakan bahasa anda sendiri.

```melayu
# Program MELAYU yang sah & berjalan
ada nama = "Budi"
ada umur = 17

jika umur lebih sama 17
    tunjuk "Selamat datang, " + nama
    tunjuk "Kau dah dewasa!"
jika tidak
    tunjuk "Sabar dulu ya, " + nama
selesai
```

---

## ✨ Kenapa MELAYU?

| Ciri | MELAYU | Python | JavaScript |
|---|:---:|:---:|:---:|
| Kata Kunci Asli Bahasa Melayu | ✅ | ❌ | ❌ |
| Penutup Blok Eksplisit (`selesai`) | ✅ | ❌ | ✅ (`{}`) |
| Tanpa Lambang Mengelirukan (`{}`, `;`) | ✅ | ⚠️ | ❌ |
| Mesej Ralat Mendidik ("Woy!", "Alamak!") | ✅ | ❌ | ❌ |
| Sokongan REPL & CLI Multi-OS | ✅ | ✅ | ✅ |

### Falsafah Reka Bentuk:
- 🤝 **Gotong Royong** — Kod harus mudah dibaca oleh sesama insan, bukan sekadar "clever code".
- 📜 **Adat** — Menghormati struktur bahasa Melayu (SPOK, kata sendi nama, penyambung).
- 🌱 **Kesederhanaan** — Pemula tidak patut dipaksa bergelut dengan `;` atau `{}` pada hari pertama.
- 💬 **Kejujuran** — Mesej ralat memberitahu **di mana silapnya**, **apa masalahnya**, dan **cara membetulkannya**.

---

## 🚀 Pemasangan

### Prasyarat
- Rust 1.75+ ([Pasang Rust](https://rustup.rs/))

### Bina dari Sumber (Source)

```bash
git clone https://github.com/rillardian655/Melayu-Language-by-runa.git
cd Melayu-Language-by-runa
cargo build --release
```

Fail perduaan boleh didapati di `target/release/melayu`.

### Tambah ke PATH

**Linux / macOS:**
```bash
sudo cp target/release/melayu /usr/local/bin/
```

**Windows:**
Salin `target\release\melayu.exe` ke mana-mana folder yang didaftarkan dalam pembolehubah persekitaran `PATH`.

---

## 🎯 Contoh Cepat

### 1. Hello World (`hello.ml`)
```melayu
tunjuk "Selamat datang, dunia!"
```
Jalankan melalui terminal:
```bash
melayu jalankan examples/hello.ml
```

### 2. Algoritma FizzBuzz
```melayu
untuk i dari 1 hingga 20
    jika i modulo 15 sama dengan 0
        tunjuk "FizzBuzz"
    jika tidak jika i modulo 3 sama dengan 0
        tunjuk "Fizz"
    jika tidak jika i modulo 5 sama dengan 0
        tunjuk "Buzz"
    jika tidak
        tunjuk i
    selesai
selesai
```

### 3. Fungsi & Rekursi
```melayu
kerja faktorial(n)
    jika n kurang sama 1
        pulang 1
    selesai
    pulang n kali faktorial(n tolak 1)
selesai

tunjuk faktorial(5) # 120
```

### 4. REPL Interaktif
Cukup taip `melayu` tanpa sebarang argumen:
```bash
$ melayu
MELAYU v0.1.0 — REPL Interaktif
Taip kod kau di bawah. Taip "keluar" untuk berhenti.

melayu> ada x = 10
melayu> ada y = 20
melayu> tunjuk x tambah y
30
melayu> keluar
Jumpa lagi! Selamat bergotong-royong!
```

---

## 📚 Dokumentasi Penuh

Ketahui lebih lanjut melalui fail dokumentasi dalam [`docs/`](docs/):
- [Pengenalan](docs/pengenalan.md) — Matlamat dan latar belakang projek
- [Sintaks Penuh](docs/sintaks.md) — Rujukan gaya tatabahasa dan contoh binaan
- [Jadual Kata Kunci](docs/keyword.md) — Padanan keyword Melayu dan bahasa Inggeris
- [Contoh Program](examples/readme.md) — Koleksi program siap sedia untuk dicuba
- [Pelan Hala Tuju](docs/roadmap.md) — Perancangan versi v0.2.0 hingga v1.0.0

---

## 🗺️ Pelan Hala Tuju (Roadmap)

- [x] **v0.1.0** — MVP: Pembolehubah (`ada`, `tetap`), jenis primitif, gelung, fungsi, kamus/senarai, ralat ramah, REPL, CLI.
- [ ] **v0.2.0** — Sistem import modul (`ambil`), fail I/O, kaedah manipulasi teks lanjutan.
- [ ] **v0.3.0** — Pengaturcaraan Berorientasikan Objek (`kelas`, `bina`, `warisi`, `diri`).
- [ ] **v0.4.0** — Perpustakaan standard (matematik, masa, rangkaian, format JSON).
- [ ] **v1.0.0** — Enjin bytecode VM pantas, pengurus pakej, taman permainan web (WASM).

---

## 🛠️ Menyumbang (Contributing)

Kami mengalu-alukan sumbangan daripada semua komuniti serumpun!
1. Baca panduan di [CONTRIBUTING.md](CONTRIBUTING.md).
2. Fork repositori ini.
3. Cipta cawangan ciri: `git checkout -b feat/ciri-baharu`.
4. Pastikan `cargo test` lulus.
5. Hantar Pull Request anda!

---

## 📄 Lesen

Dilesenkan di bawah lesen [MIT](LICENSE) © 2026 xianly runa.

---

<div align="center">

**Dibuat dengan setulus hati untuk semua pembangun serumpun Nusantara.**

*"Kod dalam bahasa sendiri. Bangga berbahasa."*

</div>

