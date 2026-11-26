# tebak_angka.ml
# Permainan tebak nombor menggunakan rawak() dan logik keadaan

ada rahsia = rawak(1, 100)
ada tekaan = 0
ada cubaan = 0

tunjuk "Selamat datang ke permainan tebak angka!"
tunjuk "Saya dah simpan satu nombor rahsia rawak antara 1 hingga 100: " + rahsia

# Simulasi tekaan pertama
tekaan = rahsia
cubaan = cubaan tambah 1

jika tekaan kurang rahsia
    tunjuk "Terlalu kecil, bro!"
jika tidak jika tekaan lebih rahsia
    tunjuk "Terlalu besar, bro!"
jika tidak
    tunjuk "Tahniah! Betul tepat! Kau tebak dalam " + cubaan + " cubaan."
selesai
