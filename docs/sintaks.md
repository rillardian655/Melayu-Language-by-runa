# 📖 Panduan Sintaks Lengkap MELAYU

## 1. Pembolehubah & Pemalar

Deklarasi pembolehubah yang boleh diubah (mutabel) menggunakan `ada`, manakala pemalar (konstanta tidak boleh ubah) menggunakan `tetap`.

```melayu
ada nama = "Budi"
ada umur = 17
tetap KADAR_CUKAI = 0.06

# Boleh menggunakan anotasi jenis pilihan
ada tinggi: nombor = 175.5
ada status: teks = "Aktif"
ada selesaiUjian: betul = betul
```

---

## 2. Struktur Kawalan Aliran

Blok pernyataan ditutup secara eksplisit dengan kata kunci `selesai`.

```melayu
ada markah = 85

jika markah lebih sama 90
    tunjuk "Gred: A"
jika tidak jika markah lebih sama 80
    tunjuk "Gred: B"
jika tidak jika markah lebih sama 70
    tunjuk "Gred: C"
jika tidak
    tunjuk "Gred: D (Cuba lagi ya!)"
selesai
```

---

## 3. Gelung (Loops)

### Gelung `selagi` (While Loop)
```melayu
ada kiraan = 1
selagi kiraan kurang sama 5
    tunjuk "Kiraan semasa: " + kiraan
    kiraan = kiraan tambah 1
selesai
```

### Gelung Julat `untuk ... dari ... hingga` (Range Loop)
```melayu
untuk i dari 1 hingga 5
    tunjuk i
selesai

# Dengan langkah (step)
untuk i dari 0 hingga 10 langkah 2
    tunjuk i
selesai
```

### Gelung Elemen `untuk ... dalam` (For-Each Loop)
```melayu
ada senaraiBuah = ["rambutan", "durian", "cempedak"]
untuk buah dalam senaraiBuah
    tunjuk "Saya suka makan " + buah
selesai
```

---

## 4. Fungsi (`kerja`)

Fungsi diisytiharkan menggunakan kata kunci `kerja` dan memulangkan nilai melalui `pulang`.

```melayu
kerja kiraGaji(jam, kadar)
    pulang jam kali kadar
selesai

ada gajiSaya = kiraGaji(8, 50000)
tunjuk "Gaji hari ini: RM" + gajiSaya
```

---

## 5. Koleksi: Senarai & Kamus

### Senarai (Array/List)
```melayu
ada angka = [10, 20, 30]
tunjuk angka[0]          # 10
angka.tambah(40)         # Menambah elemen
tunjuk panjang(angka)    # 4
```

### Kamus (Map/Dictionary)
```melayu
ada pekerja = kamus {
    "nama": "Ahmad",
    "jabatan": "Teknologi",
    "gaji": 6500
}

tunjuk pekerja["nama"]   # Ahmad
pekerja["gaji"] = 7000
tunjuk pekerja.kunci()   # ["nama", "jabatan", "gaji"]
```

---

## 6. Pengendalian Ralat (`cuba ... tangkap`)

```melayu
cuba
    ada hasil = 100 bahagi 0
    tunjuk hasil
tangkap ralat
    tunjuk "Ralat berjaya ditangkap: " + ralat
akhirnya
    tunjuk "Blok pembersihan selesai dilaksanakan."
selesai
```

---

## 7. Ciri Keselamatan & Sandbox Fail (v0.1.1)

Bahasa MELAYU dilengkapi dengan benteng keselamatan aras enjin untuk melindungi sistem hos:

1. **Perlindungan 'Stack Overflow'**: Had kedalaman rekursi fungsi ditetapkan kepada 1,000 tahap secara lalai.
2. **Perlindungan 'Runaway Loop' DoS**: Had langkah pelaksanaan memastikan gelung tak terhingga ditamatkan secara anggun. Had lalai ialah 10 juta langkah; ubah dengan `melayu --had-langkah <n> fail.ml` atau `MELAYU_HAD_LANGKAH=<n>` (`0` bermaksud tiada had).
3. **Pengesahan Indeks Ketat**:
   - Menolak indeks tidak terhingga (`NaN`, `Infiniti`) atau perpuluhan (`s[1.5]`).
   - Menyokong indeks negatif yang sah (contoh `senarai[-1]` untuk elemen terakhir).
4. **Perlindungan 'Path Traversal'**: Operasi fail (`bacaFail`, `tulisFail`, `wujudFail`, `padamFail`) disekat daripada mengakses luar direktori kerja projek.
5. **Had Saiz Fail**: Bacaan dan penulisan fail dihadkan kepada saiz maksimum 5 MB.
6. **Sempadan sandbox fail ialah direktori kerja semasa** (tempat arahan `melayu` dijalankan), bukan folder fail `.ml`. Jalankan program dari folder projek.
7. **Anti-SSRF**: `hantarPermintaanHTTP` menyemak alamat IP sebenar hos dan menyekat alamat link-local dan metadata awan. Localhost dan rangkaian peribadi dibenarkan untuk pembangunan tempatan. Hanya `http://` disokong (tiada TLS).

---

## 8. Sistem Modul & Import (`ambil`) (v0.1.2)

Bahasa MELAYU menyokong pengurusan kod multi-fail modular:

### Bentuk 1: Import Fail Penuh
```melayu
ambil "matematik.ml"
# Semua fungsi dan pembolehubah daripada fail boleh diakses terus
```

### Bentuk 2: Import Ruang Nama (Namespace / Kamus)
```melayu
ambil "matematik.ml" sebagai mat
ada hasil = mat["kiraDiskaun"](100, 10)
```

### Bentuk 3: Import Simbol Terpilih
```melayu
ambil kiraDiskaun, adakahGenap daripada "matematik.ml"
ada diskaun = kiraDiskaun(200, 15)
```

---

## 9. Kaedah Manipulasi Teks & Senarai Lanjutan (v0.1.2)

### Kaedah Rentetan Teks:
* `.pecah(pemisah)`: Memecahkan teks menjadi senarai rentetan.
* `.ganti(lama, baru)`: Menggantikan teks sasaran dengan teks baharu.
* `.mulaDengan(awalan)`: Menyemak sama ada teks bermula dengan awalan tertentu (`betul`/`salah`).
* `.tamatDengan(akhiran)`: Menyemak sama ada teks berakhir dengan akhiran tertentu.
* `.mengandungi(subteks)`: Menyemak kehadiran subteks.
* `.hurufBesar()`, `.hurufKecil()`, `.panjang()`, `.potong()`.

### Kaedah Senarai:
* `.isih()`: Menyusun elemen senarai mengikut turutan menaik.
* `.balik()`: Membalikkan susunan elemen senarai.
* `.cantum(pemisah)`: Menggabungkan elemen senarai menjadi satu rentetan teks.
* `.tambah(item)`, `.buang(indeks)`, `.panjang()`, `.berisi(item)`.

### Utiliti Matematik Terbina-Dalam:
* `bundar(n)`: Pembundaran angka ke nombor bulat terdekat.
* `lantai(n)`: Nilai lantai matematik ($\lfloor n \rfloor$).
* `siling(n)`: Nilai siling matematik ($\lceil n \rceil$).
* `mutlak(n)`: Nilai mutlak ($|n|$).
* `maksimum(a, b, ...)`: Mencari nilai terbesar.
* `minimum(a, b, ...)`: Mencari nilai terkecil.

---

## 10. Pemprosesan Data Fungsian, Enjin JSON & Utiliti Masa (v0.2.0)

Bermula versi v0.2.0, bahasa MELAYU menyokong pemprosesan data aras tinggi dan pertukaran format web moden secara terbina-dalam tanpa sebarang kebergantungan luar:

### Operasi Fungsian Senarai (Higher-Order Functions):
* `.petakan(fungsi)`: Mengubah setiap elemen senarai kepada nilai baharu menggunakan fungsi penukar (*map*).
* `.tapis(fungsi)`: Menyaring senarai mengikut syarat fungsi logik (*filter*).
* `.kiraSemua(fungsi, nilaiAwal)`: Mengumpul dan meringkaskan elemen senarai menjadi satu nilai (*reduce / fold*).
* `.cari(fungsi)`: Mencari elemen pertama yang memenuhi syarat fungsi (*find*).
* `.indeks(item)`: Mengembalikan indeks elemen atau `-1` jika tiada (*indexOf*).
* `.setiapSatu(fungsi)`: Menjalankan fungsi bagi setiap elemen tanpa memulangkan senarai (*forEach*).

```melayu
ada nombor = [1, 2, 3, 4, 5, 6]

kerja ganda(x)
    pulang x kali 2
selesai

kerja genap(x)
    pulang (x % 2) == 0
selesai

ada nomborGenap = nombor.tapis(genap)           # [2, 4, 6]
ada digandakan = nomborGenap.petakan(ganda)     # [4, 8, 12]
```

### Enjin Pertukaran Format JSON Terbina-Dalam:
* `bacaJSON(teks)`: Menghurai teks JSON standard kepada struktur objek kamus/senarai MELAYU. Dilindungi benteng kedalaman rekursi keselamatan sehingga 500 peringkat.
* `tulisJSON(nilai, cantik?)`: Menukar sebarang struktur data MELAYU kepada teks JSON. Pilihan boolean `betul` menghasilkan format inden kemas (*pretty-print*).

```melayu
ada teksData = "{\"nama\": \"Nusantara\", \"populasi\": 280000000}"
ada data = bacaJSON(teksData)
tunjuk data["nama"]     # Nusantara

data["status"] = "Maju"
ada jsonKemas = tulisJSON(data, betul)
```

### Kaedah Rentetan Teks & Kamus Lanjutan:
* `.ulang(n)`: Mengulang teks sebanyak `n` kali.
* `.selitDepan(panjang, aksara?)`: Melapik aksara di hadapan rentetan (*padStart*).
* `.selitBelakang(panjang, aksara?)`: Melapik aksara di belakang rentetan (*padEnd*).
* `.hurufAwalBesar()`: Menukar aksara pertama menjadi huruf besar (*capitalize*).
* `kamus.adaKunci(kunci)`: Menyemak kewujudan kunci dalam kamus (`betul`/`salah`).
* `kamus.buang(kunci)`: Memadam kunci daripada kamus dan mengembalikan nilainya.

### Utiliti Tarikh & Masa Berketepatan Tinggi:
* `capMasa()`: Mengembalikan cap masa Unix semasa (saat sejak era Unix 1970 UTC).
* `masaKini()`: Mengembalikan rentetan masa jam semasa mengikut format 24 jam `"HH:MM:SS"`.
* `tarikhKini()`: Mengembalikan rentetan tarikh kalendar Gregory semasa mengikut format piawai ISO `"YYYY-MM-DD"`.

---

## 11. Pengaturcaraan Berorientasikan Objek (OOP) (v0.3.0)

Bahasa MELAYU menyokong paradigma pengaturcaraan berorientasikan objek secara natif dengan kata kunci leksikal Melayu yang jelas:

### Pengisytiharan Kelas & Pembina (`kelas`, `bina`, `diri`):
```melayu
kelas Pengguna
    # Pembina objek (constructor)
    bina(nama, umur)
        diri.nama = nama
        diri.umur = umur
        diri.skor = 0
    selesai

    # Kaedah instans (method)
    kerja sapa()
        pulang "Hai, nama saya " + diri.nama
    selesai

    kerja tambahSkor(mata)
        diri.skor = diri.skor + mata
        pulang diri.skor
    selesai
selesai

# Pembinaan objek (instantiation)
ada ali = Pengguna("Ali", 25)
tunjuk ali.sapa()
ali.tambahSkor(50)
tunjuk "Skor terkini:", ali.skor
```

### Pewarisan Kelas & Rujukan Induk (`warisi`, `induk`):
```melayu
kelas Pentadbir warisi Pengguna
    bina(nama, umur, tahap)
        # Memanggil pembina kelas induk
        induk.bina(nama, umur)
        diri.tahap = tahap
    selesai

    # Menindih kaedah induk (override)
    kerja sapa()
        pulang induk.sapa() + " [Akaun: " + diri.tahap + "]"
    selesai
selesai

ada admin = Pentadbir("Siti", 30, "Ketua Pentadbir")
tunjuk admin.sapa()
```

### Ciri-Ciri Utama Sistem Objek:
* **`diri`** (`this` / `self`): Merujuk kepada instans objek semasa bagi membaca atau menetapkan sifat (`diri.sifat = nilai`).
* **`induk`** (`super`): Merujuk kepada kelas induk bagi memanggil pembina induk (`induk.bina(...)`) atau kaedah induk (`induk.kaedah(...)`).
* **Sifat Dinamik**: Sifat objek boleh diakses dan diumpukkan menggunakan sintaks titik (`obj.sifat = nilai`) atau indeks rentetan (`obj["sifat"] = nilai`).
* **Integrasi `jenis(obj)`**: Mengembalikan nama kelas instans berkenaan (contohnya `"Pengguna"`, `"Pentadbir"`).
* **Integrasi `tulisJSON(obj)`**: Menyokong penukaran terus struktur sifat objek kepada format teks JSON piawai.





