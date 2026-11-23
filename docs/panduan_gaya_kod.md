# Panduan Gaya Penulisan Kod MELAYU (Style Guide)

Panduan ini menetapkan konvensyen penulisan kod yang bersih, konsisten, dan mudah dibaca oleh komuniti pembangun bahasa **MELAYU**.

## 1. Konvensyen Penamaan

| Elemen Kod | Format Penamaan | Contoh Yang Disyorkan | Contoh Yang Dielakkan |
|---|---|---|---|
| **Pembolehubah** | camelCase | `ada jumlahJualan = 100` | `ada Jumlah_Jualan` |
| **Pemalar** | UPPER_SNAKE_CASE | `tetap KADAR_CUKAI = 0.06` | `tetap kadarCukai` |
| **Fungsi / Kaedah** | camelCase kata kerja | `kerja kiraDiskaun(harga) ...` | `kerja D() ...` |
| **Kelas (OOP)** | PascalCase | `kelas PelangganVIP ...` | `kelas pelanggan_vip` |
| **Fail Atur Cara** | snake_case `.ml` | `proses_pesanan.ml` | `ProsesPesanan.ML` |

## 2. Struktur Blok & Indentasi

* Gunakan **4 ruang aksara (4 spaces)** bagi setiap aras indentasi di dalam blok.
* Setiap blok wajib ditutup dengan kata kunci `selesai` pada lajur permulaan blok:

```melayu
# Disyorkan
jika umur lebih sama 18
    tunjuk "Dewasa"
jika tidak
    tunjuk "Bawah Umur"
selesai
```

## 3. Penulisan Pengaturcaraan Berorientasikan Objek (OOP)

* Pembina instans dinamakan `bina`.
* Sifat dalaman diikat menggunakan rujukan `diri`:
* Jika mewarisi kelas lain, panggil `induk.bina` sebelum mengikat sifat baharu:

```melayu
kelas AkaunPelajar warisi Akaun
    bina(nama, idPelajar)
        induk.bina(nama)
        diri.idPelajar = idPelajar
    selesai
selesai
```

## 4. Baris Komen & Dokumentasi

Gunakan simbol tanda pagar `#` untuk penerangan kod:

```melayu
# Mengira jumlah pesanan bersih selepas potongan diskaun ahli
kerja kiraJumlahBersih(hargaAsal, diskaun)
    pulang hargaAsal tolak (hargaAsal kali diskaun)
selesai
```
