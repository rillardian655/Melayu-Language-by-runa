# kalkulator.ml
# Program kalkulator fungsi aritmetik mudah dalam bahasa Melayu

kerja tambahNombor(a, b)
    pulang a tambah b
selesai

kerja tolakNombor(a, b)
    pulang a tolak b
selesai

kerja darabNombor(a, b)
    pulang a kali b
selesai

kerja bahagiNombor(a, b)
    jika b sama dengan 0
        baling "Ralat: Pembahagi sifar dikesan!"
    selesai
    pulang a bahagi b
selesai

tunjuk "=== KALKULATOR MELAYU ==="
ada x = 20
ada y = 4

tunjuk "Nombor pertama: " + x
tunjuk "Nombor kedua  : " + y
tunjuk "Hasil Tambah  : " + tambahNombor(x, y)
tunjuk "Hasil Tolak   : " + tolakNombor(x, y)
tunjuk "Hasil Darab   : " + darabNombor(x, y)
tunjuk "Hasil Bahagi  : " + bahagiNombor(x, y)

cuba
    ada ralat = bahagiNombor(10, 0)
tangkap e
    tunjuk "Ujian keselamatan: " + e
selesai

