# 🗺️ Pelan Hala Tuju (Roadmap) MELAYU

Bahasa MELAYU mempunyai pelan pembangunan berperingkat untuk memastikan kestabilan dan kematangan ekosistem.

---

### ☑️ v0.1.0 — MVP (Rilis Semasa)
- [x] Pembolehubah mutabel (`ada`) dan malar (`tetap`).
- [x] Jenis data primitif: `teks`, `nombor`, `betul`, `salah`, `kosong`.
- [x] Struktur kawalan: `jika`, `jika tidak jika`, `jika tidak`, `selesai`.
- [x] Gelung: `selagi` (while), `untuk ... dalam` (for-in), `untuk ... dari ... hingga` (range).
- [x] Fungsi pengguna: `kerja` + `pulang` (menyokong rekursi & penutupan leksikal).
- [x] Struktur data asas: `senarai` (list) & `kamus` (map).
- [x] Output `tunjuk` dan input terminal `baca()` / `baca nombor()`.
- [x] Pengendalian ralat: `cuba`, `tangkap`, `baling`, `akhirnya`.
- [x] REPL interaktif dengan sokongan multi-baris.
- [x] Mesej ralat ramah dengan cadangan penyelesaian.

---

### ⏳ v0.2.0 — Modul & I/O Sistem Fail
- [ ] Sistem import fail modul: `ambil "matematik"` dan `dari "fail" ambil foo`.
- [ ] Fail I/O: `bacaFail("data.txt")` dan `tulisFail("data.txt", kandungan)`.
- [ ] Pengurusan rentetan lanjutan: `.pecah(",")`, `.cantum()`, `.ganti()`.
- [ ] Pakej matematik terbina-dalam: trigonometri, logaritma, penjana nombor rawak.
- [ ] CLI linter & pemformat kod automatik (`melayu susun`).

---

### ⏳ v0.3.0 — Pengaturcaraan Berorientasikan Objek (OOP)
- [ ] Takrifan kelas: `kelas <Nama> ... selesai`.
- [ ] Pembina objek: `bina(...)`.
- [ ] Pewarisan kelas: `warisi`.
- [ ] Rujukan diri: `diri`.
- [ ] Pengendalian jenis statik opsional (Gradual Typing).

---

### ⏳ v0.4.0 — Perpustakaan Standard Lengkap (StdLib)
- [ ] Modul `masa` (tarikh, waktu, zon masa serumpun).
- [ ] Modul `rangkaian` (klien HTTP GET/POST mudah).
- [ ] Sokongan format data JSON & CSV terbina-dalam.

---

### 🚀 v1.0.0 — Kestabilan Pengeluaran & Web WASM
- [ ] Penyusun Bytecode & Mesin Maya (VM) berprestasi tinggi.
- [ ] Pengurus pakej rasmi Melayu (`rempah` / `pasar`).
- [ ] Laman Web Interaktif (Playground) berasaskan WebAssembly (WASM).
- [ ] Pemalam editor (VS Code extension) untuk pewarnaan sintaks dan LSP.

