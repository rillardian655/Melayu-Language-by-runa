horas
# Program hello world
tunjuk "Selamat datang, dunia!"

# Variabel
ada nama = "Budi"
tetap umur = 17

# Kondisional
jika umur lebih 17
    tunjuk "Dewasa"
jika tidak
    tunjuk "Bujang lagi"
selesai

# Loop
ada nombor = 0
selagi nombor kurang 5
    tunjuk "Kiraan: " + nombor
    nombor = nombor tambah 1
selesai

# Fungsi
kerja kiraGaji(jam, kadar)
    pulang jam kali kadar
selesai

tunjuk kiraGaji(8, 50000)

# Senarai
ada buah = ["rambutan", "durian", "mangga"]
untuk b dalam buah
    tunjuk b
selesai

# Kamus
ada pelajar = kamus {
    "nama": "Siti",
    "umur": 20
}
tunjuk pelajar["nama"]

# Error handling
cuba
    ada hasil = 10 bahagi 0
tangkap e
    tunjuk "Alamak, error: " + e
akhirnya
    tunjuk "Dah selesai cuba"
selesai

mangan

