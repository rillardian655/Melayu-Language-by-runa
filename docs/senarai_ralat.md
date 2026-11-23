# Katalog Ralat & Panduan Nyahpepijat (Error Catalogue)

Bahasa MELAYU menggunakan pendekatan **Mesej Ralat Berhemah & Mendidik** yang direka khas untuk membimbing pembangun mengenal pasti dan membaiki kesilapan kod.

## Kategori Mesej Ralat Utama

### 1. `Eh!` (Ralat Leksikal & Sintaks)
* **Punca**: Kata kunci salah eja, simbol tidak dijangka, atau blok kod tidak ditutup dengan `selesai`.
* **Contoh Paparan**:
  ```text
  Eh! Ada ralat di baris 1, lajur 1:
    tunjukkan "Salam"
    ^
    "tunjukkan" tu bukan keyword Melayu yang sah.
    -> Mungkin maksud kau "tunjuk"? Cuba betulkan dulu.
  ```

### 2. `Alamak!` (Ralat Semantik & Analisis Skop)
* **Punca**: Mengisytiharkan pembolehubah bertindih, membaca pembolehubah yang belum wujud, atau memanggil fungsi dengan bilangan hujah salah.
* **Contoh Paparan**:
  ```text
  Alamak! Ada ralat di baris 4, lajur 5:
    ada nama = "Ali"
    ^
    Pembolehubah 'nama' dah wujud, tak boleh dideklarasikan dua kali.
    -> Guna nama lain, atau buang 'ada' kalau kau cuma nak tukar nilai 'nama = ...'.
  ```

### 3. `Aduh!` (Ralat Matematik & Nilai Terlarang)
* **Punca**: Pembahagian dengan sifar (`bahagi 0`), punca kuasa nombor negatif, atau indeks senarai pecahan (`[1.5]`).
* **Contoh Paparan**:
  ```text
  Aduh! Ada ralat di baris 2, lajur 14:
    ada x = 10 bahagi 0
                   ^
    Pembahagian dengan kosong tak boleh lah bro.
    -> Cuba semak pembahagi kau, pastikan nilainya bukan 0.
  ```

### 4. `Woy!` (Ralat Aliran Kawalan & Keselamatan Sandbox)
* **Punca**: Menggunakan `hentikan` atau `teruskan` di luar gelung, percubaan *path traversal* (`../`), atau gelung liar melebihi had pelaksanaan.
* **Contoh Paparan**:
  ```text
  Woy! Had pelaksanaan selamat (10,000,000 langkah) dicapai.
  Kemungkinan terdapat gelung tak terhingga (infinite loop).
  ```
