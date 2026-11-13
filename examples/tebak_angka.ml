# tebak_angka.ml
# Permainan tebak nombor menggunakan baca nombor()

ada rahsia = 42
ada tekaan = 0
ada cubaan = 0

tunjuk "Selamat datang ke permainan tebak angka!"
tunjuk "Saya dah simpan satu nombor rahsia antara 1 hingga 100."

# Nota: Dalam persekitaran interaktif, gelung ini akan membaca input anda
# Di sini kita simulasikan satu tekaan:
tunjuk "Masukkan tekaan kau (contoh 42):"
tekaan = 42
cubaan = cubaan tambah 1

jika tekaan kurang rahsia
    tunjuk "Terlalu kecil, bro!"
jika tidak jika tekaan lebih rahsia
    tunjuk "Terlalu besar, bro!"
jika tidak
    tunjuk "Tahniah! Betul tepat! Kau tebak dalam " + cubaan + " cubaan."
selesai

