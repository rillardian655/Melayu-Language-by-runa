# keselamatan_dan_fail.ml
# Contoh penggunaan ciri keselamatan & operasi fail selamat (sandboxed) dalam MELAYU

tunjuk "=== DEMO KESELAMATAN & OPERASI FAIL DALAM MELAYU ==="

# 1. Penjanaan Nombor Rawak Selamat
ada dadu = rawak(1, 6)
tunjuk "1. Baligan dadu (rawak 1-6):", dadu

# 2. Operasi Fail Selamat dalam Sandbox
ada nama_fail = "nota_rahsia.txt"
ada kandungan = "Bahasa Melayu selamat & pantas!"

tulisFail(nama_fail, kandungan)
tunjuk "2. Fail berjaya ditulis:", nama_fail

jika wujudFail(nama_fail)
    ada bacaan = bacaFail(nama_fail)
    tunjuk "   Kandungan fail dibaca:", bacaan
    padamFail(nama_fail)
    tunjuk "   Fail sementara berjaya dipadam."
selesai

# 3. Sekatan Keselamatan: Percubaan Membaca Fail Luar Sandbox (Path Traversal)
tunjuk "3. Menguji sekatan keselamatan path traversal:"
cuba
    # Percubaan mengakses fail di luar direktori projek
    bacaFail("../../../etc/passwd")
tangkap ralat
    tunjuk "   [DISEKAT DENGAN SELAMAT]:", ralat
selesai

# 4. Pengesahan Indeks Ketat & Indeks Negatif
ada senarai_bahasa = ["Indonesia", "Malaysia", "Brunei", "Singapura"]
tunjuk "4. Bahasa serumpun terakhir (indeks -1):", senarai_bahasa[-1]

cuba
    # Mengakses indeks pecahan/perpuluhan yang tidak sah
    ada nilai_salah = senarai_bahasa[2.7]
tangkap ralat_indeks
    tunjuk "   [RALAT INDEKS DIPERIKSA]:", ralat_indeks
selesai

tunjuk "=== SEMUA UJIAN KESELAMATAN BERJAYA ==="

