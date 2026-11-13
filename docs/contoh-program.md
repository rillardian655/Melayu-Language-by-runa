# 💻 Contoh-Contoh Program MELAYU

## 1. Algoritma FizzBuzz

```melayu
# fizzbuzz.ml
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

---

## 2. Jujukan Fibonacci Rekursif

```melayu
# fibonacci.ml
kerja fibo(n)
    jika n kurang sama 0
        pulang 0
    selesai
    jika n sama dengan 1
        pulang 1
    selesai
    pulang fibo(n tolak 1) tambah fibo(n tolak 2)
selesai

untuk k dari 0 hingga 9
    tunjuk "F(" + k + ") = " + fibo(k)
selesai
```

---

## 3. Manipulasi Kamus & Senarai Data Murid

```melayu
ada senaraiMurid = [
    kamus { "nama": "Aisyah", "markah": 92 },
    kamus { "nama": "Rizal", "markah": 78 },
    kamus { "nama": "Dewi", "markah": 88 }
]

ada jumlahMarkah = 0
untuk murid dalam senaraiMurid
    tunjuk "Nama: " + murid["nama"] + " | Markah: " + murid["markah"]
    jumlahMarkah = jumlahMarkah tambah murid["markah"]
selesai

ada purata = jumlahMarkah bahagi panjang(senaraiMurid)
tunjuk "Purata Keseluruhan: " + purata
```

