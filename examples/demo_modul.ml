# demo_modul.ml
# Demonstrasi sistem modul 'ambil', manipulasi teks & senarai lanjutan

tunjuk "=== DEMO SISTEM MODUL & ALAT LANJUTAN MELAYU ==="

# 1. Mengimport fungsi terpilih daripada modul luar
ambil kiraDiskaun, adakahGenap, purata daripada "matematik_modul.ml"

ada hargaAsal = 100000
ada hargaDiskaun = kiraDiskaun(hargaAsal, 25)
tunjuk "1. Harga asal RM" + hargaAsal + " selepas diskaun 25%: RM" + hargaDiskaun

ada angka = 14
tunjuk "   Adakah " + angka + " nombor genap?", adakahGenap(angka)

ada senaraiGaji = [3500, 4200, 5000, 6100]
tunjuk "   Purata gaji pekerja:", purata(senaraiGaji)

# 2. Mengimport modul sebagai ruang nama (Kamus / Namespace)
ambil "matematik_modul.ml" sebagai ModulMatematik
tunjuk "2. Nilai PI daripada modul:", ModulMatematik["PI"]

# 3. Manipulasi Teks & Koleksi Lanjutan
ada dataMentah = "kopi,teh,cendol,bandung"
ada minuman = dataMentah.pecah(",")
tunjuk "3. Senarai asal:", minuman

minuman.isih()
tunjuk "   Selepas disusun (.isih()):", minuman.cantum(" -> ")

minuman.balik()
tunjuk "   Selepas dibalikkan (.balik()):", minuman.cantum(" <- ")

ada mesej = "Selamat Datang ke MelayuLang"
ada mesejBaharu = mesej.ganti("MelayuLang", "MELAYU v0.1.2")
tunjuk "4. Penggantian teks:", mesejBaharu
tunjuk "   Mula dengan 'Selamat'?", mesej.mulaDengan("Selamat")
tunjuk "   Mengandungi 'Datang'?", mesej.mengandungi("Datang")

# 5. Fungsi Matematik Terbina-Dalam
tunjuk "5. Pembundaran 3.7:", bundar(3.7)
tunjuk "   Lantai 3.9:", lantai(3.9)
tunjuk "   Siling 3.1:", siling(3.1)
tunjuk "   Maksimum(12, 99, 45):", maksimum(12, 99, 45)

tunjuk "=== SELESAI ==="

