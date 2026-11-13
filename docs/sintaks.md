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

