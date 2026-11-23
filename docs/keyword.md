# 🗂️ Jadual Rujukan Kata Kunci (Keyword Reference)

Jadual komprehensif bagi seluruh kata kunci bahasa MELAYU v0.1.0:

| Kata Kunci | Padanan Bahasa Inggeris | Keterangan Singkat |
|---|---|---|
| `ada` | `let` / `var` | Isytihar pembolehubah mutabel |
| `tetap` | `const` | Isytihar pemalar tidak boleh diubah |
| `teks` | `string` | Jenis data teks rentetan |
| `nombor` | `number` / `float` | Jenis data angka (integer & perpuluhan) |
| `betul` | `true` | Nilai boolean benar |
| `salah` | `false` | Nilai boolean palsu |
| `kosong` | `null` / `nil` | Nilai tiada / null |
| `senarai` | `list` / `array` | Struktur data senarai berindeks |
| `kamus` | `dict` / `map` | Struktur data pasangan kunci-nilai |
| `jika` | `if` | Syarat kondisional |
| `jika tidak jika` | `else if` | Syarat alternatif |
| `jika tidak` | `else` | Syarat lalai |
| `selagi` | `while` | Gelung berasaskan syarat |
| `untuk` | `for` | Gelung lelaran |
| `dalam` | `in` | Operator keanggotaan koleksi |
| `dari` | `from` | Had mula julat |
| `hingga` | `to` | Had akhir julat |
| `langkah` | `step` | Kenaikan julat |
| `hentikan` | `break` | Hentikan gelung serta-merta |
| `teruskan` | `continue` | Teruskan ke lelaran seterusnya |
| `kerja` | `function` / `def` | Isytihar fungsi |
| `pulang` | `return` | Memulangkan nilai dari fungsi |
| `selesai` | `end` | Menutup mana-mana blok kod |
| `tunjuk` | `print` | Mencetak output ke terminal |
| `baca` | `input` | Membaca rentetan dari terminal |
| `baca nombor` | `input number` | Membaca nombor dari terminal |
| `cuba` | `try` | Memulakan blok perlindungan ralat |
| `tangkap` | `catch` | Menangkap dan mengendalikan ralat |
| `baling` | `throw` | Menjana atau melontar ralat baharu |
| `akhirnya` | `finally` | Blok penutup yang sentiasa dijalankan |
| `horas` | shebang | Penanda pembuka hormat Nusantara (pilihan) |
| `mangan` | main | Penanda penutup eksekusi Nusantara (pilihan) |

## Operator Bersinonim
Bahasa Melayu menyokong penggunaan perkataan mahupun simbol antarabangsa secara berselang-seli:

* `tambah` $\equiv$ `+`
* `tolak` $\equiv$ `-`
* `kali` $\equiv$ `*`
* `bahagi` $\equiv$ `/`
* `modulo` $\equiv$ `%`
* `sama dengan` $\equiv$ `==`
* `tidak sama` $\equiv$ `!=`
* `lebih` $\equiv$ `>`
* `kurang` $\equiv$ `<`
* `lebih sama` $\equiv$ `>=`
* `kurang sama` $\equiv$ `<=`
* `dan` $\equiv$ `&&`
* `atau` $\equiv$ `||`
* `bukan` $\equiv$ `!`

## Fungsi Terbina-Dalam (Built-in Functions) & Keselamatan

Bahasa MELAYU menyertakan fungsi standard yang dilindungi sistem keselamatan sandbox:

| Fungsi | Penerangan | Contoh |
|---|---|---|
| `panjang(x)` | Mengira saiz senarai, kamus, atau panjang teks | `panjang([1, 2, 3])` |
| `jenis(x)` | Menyemak nama jenis data | `jenis(42)` $\to$ `"nombor"` |
| `hurufBesar(t)` | Menukar rentetan teks kepada huruf besar | `hurufBesar("hai")` $\to$ `"HAI"` |
| `hurufKecil(t)` | Menukar rentetan teks kepada huruf kecil | `hurufKecil("HAI")` $\to$ `"hai"` |
| `puncaKuasaDua(n)` | Mengira punca kuasa dua ($\sqrt{n}$) | `puncaKuasaDua(16)` $\to$ `4` |
| `teks(x)` | Menukar sebarang nilai kepada teks | `teks(100)` $\to$ `"100"` |
| `nombor(x)` | Menukar teks angka kepada nombor | `nombor("3.14")` $\to$ `3.14` |
| `rawak()` | Menjana nombor perpuluhan rawak 0.0 - 1.0 | `rawak()` |
| `rawak(min, max)` | Menjana nombor bulat rawak dalam julat [min, max] | `rawak(1, 100)` |
| `bacaFail(laluan)` | Membaca fail teks dalam sandbox (had 5 MB) | `bacaFail("data.txt")` |
| `tulisFail(laluan, t)` | Menulis fail teks dalam sandbox (had 5 MB) | `tulisFail("log.txt", "OK")` |
| `wujudFail(laluan)` | Menyemak sama ada fail wujud | `wujudFail("data.txt")` |
| `padamFail(laluan)` | Memadam fail dalam direktori kerja semasa | `padamFail("log.txt")` |
| `bundar(n)` | Membundarkan angka kepada integer terdekat | `bundar(3.7)` $\to$ `4` |
| `lantai(n)` | Mengira nilai lantai matematik ($\lfloor n \rfloor$) | `lantai(3.9)` $\to$ `3` |
| `siling(n)` | Mengira nilai siling matematik ($\lceil n \rceil$) | `siling(3.1)` $\to$ `4` |
| `mutlak(n)` | Mengira nilai mutlak ($|n|$) | `mutlak(-42)` $\to$ `42` |
| `maksimum(a, b, ...)` | Mengembalikan nilai tertinggi | `maksimum(10, 50, 30)` $\to$ `50` |
| `minimum(a, b, ...)` | Mengembalikan nilai terendah | `minimum(10, 50, 5)` $\to$ `5` |
| `bacaJSON(teks)` | Menghurai teks JSON kepada struktur data asli MELAYU | `bacaJSON("{\"a\": 1}")` |
| `tulisJSON(nilai, cantik?)` | Menukar data MELAYU ke format teks JSON piawai | `tulisJSON(data, betul)` |
| `capMasa()` | Mengembalikan cap masa Unix semasa dalam saat | `capMasa()` |
| `masaKini()` | Mengembalikan masa semasa sistem format "HH:MM:SS" | `masaKini()` |
| `tarikhKini()` | Mengembalikan tarikh semasa sistem format "YYYY-MM-DD" | `tarikhKini()` |

## Kaedah Struktur Data (Methods)

| Sasaran | Kaedah | Penerangan | Contoh |
|---|---|---|---|
| **Senarai** | `.petakan(fungsi)` | Memetakan setiap elemen (*map*) | `nombor.petakan(ganda)` |
| | `.tapis(fungsi)` | Menyaring elemen mengikut syarat (*filter*) | `nombor.tapis(genap)` |
| | `.kiraSemua(fungsi, awal)` | Meringkaskan senarai ke satu nilai (*reduce*) | `nombor.kiraSemua(tambah, 0)` |
| | `.cari(fungsi)` | Mencari elemen pertama yang sepadan (*find*) | `nombor.cari(adakahWujud)` |
| | `.indeks(item)` | Mencari indeks elemen (`-1` jika tiada) | `senarai.indeks("durian")` |
| | `.setiapSatu(fungsi)` | Menjalankan tindakan bagi setiap elemen (*forEach*) | `senarai.setiapSatu(papar)` |
| | `.isih()` | Menyusun elemen secara menaik | `senarai.isih()` |
| | `.balik()` | Membalikkan susunan elemen | `senarai.balik()` |
| | `.cantum(pemisah)` | Menggabungkan elemen menjadi teks | `senarai.cantum(", ")` |
| **Kamus** | `.adaKunci(kunci)` | Menyemak kewujudan kunci dalam kamus | `kamus.adaKunci("nama")` |
| | `.buang(kunci)` | Memadam kunci dan memulangkan nilainya | `kamus.buang("umur")` |
| | `.kunci()` | Mengembalikan senarai semua kunci | `kamus.kunci()` |
| | `.nilai()` | Mengembalikan senarai semua nilai | `kamus.nilai()` |
| **Teks** | `.ulang(n)` | Mengulang teks sebanyak n kali | `"*".ulang(10)` |
| | `.selitDepan(panjang, aksara?)` | Melapik aksara dari depan (*padStart*) | `"42".selitDepan(5, "0")` |
| | `.selitBelakang(panjang, aksara?)` | Melapik aksara dari belakang (*padEnd*) | `"Tajuk".selitBelakang(15, ".")` |
| | `.hurufAwalBesar()` | Menukar huruf pertama menjadi huruf besar | `"melayu".hurufAwalBesar()` |
| | `.pecah(pemisah)` | Memecah teks menjadi senarai rentetan | `"a,b,c".pecah(",")` |
| | `.ganti(lama, baru)` | Menggantikan teks padanan | `"hai dunia".ganti("hai", "salam")` |

> **Nota Keselamatan Sandbox:** Operasi fail dihalang daripada mengakses laluan di luar direktori kerja (Path Traversal Protection) demi mengelakkan pencerobohan sistem fail.



