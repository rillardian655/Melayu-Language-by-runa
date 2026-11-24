# ==============================================================================
# CONTOH PENGELAS POKOK KEPUTUSAN (DECISION TREE CLASSIFIER) - AKALML
# ==============================================================================
# Menunjukkan penggunaan algoritma Pokok Keputusan berasaskan Ketulenan Gini
# menggunakan dataset terbina-dalam Fisher Iris (muatIris).
# ==============================================================================

ambil "akal"

tunjuk "=========================================================="
tunjuk "   KLASIFIKASI BUNGA IRIS DENGAN POKOK KEPUTUSAN (AKALML)"
tunjuk "=========================================================="
tunjuk ""

# 1. Muat Dataset Terbina Iris
tunjuk "[1/4] Memuatkan dataset Iris..."
ada iris = muatIris()
ada X = iris["ciri"]
ada y = iris["sasaran"]
tunjuk "  Jumlah sampel : " + teks(panjang(X))
tunjuk "  Nama ciri     : " + iris["nama_ciri"].cantum(", ")
tunjuk "  Nama kelas    : " + iris["nama_kelas"].cantum(", ")

# 2. Bahagikan Dataset Latihan & Ujian (80% : 20%)
tunjuk "[2/4] Membahagi data latihan (80%) dan ujian (20%)..."
ada data_pecah = bahagiLatihUji(X, y, 0.2)
ada X_latih = data_pecah[0]
ada X_uji = data_pecah[1]
ada y_latih = data_pecah[2]
ada y_uji = data_pecah[3]

# 3. Bina & Latih Model Pokok Keputusan
tunjuk "[3/4] Melatih PokokKeputusan (Kedalaman Maksimum: 4)..."
ada model = PokokKeputusan(4)
model.latih(X_latih, y_latih)

# 4. Ramalan & Penilaian Prestasi
ada ramalan = model.ramal(X_uji)
ada ketepatan = model.skor(X_uji, y_uji)

tunjuk "[4/4] Hasil Ramalan ke atas Data Ujian:"
ada i = 0
selagi i < panjang(X_uji)
    ada kelas_ramal = iris["nama_kelas"][ramalan[i]]
    ada kelas_benar = iris["nama_kelas"][y_uji[i]]
    tunjuk "  Sampel #" + teks(i + 1) + " -> Ramalan: " + kelas_ramal + " | Sebenar: " + kelas_benar
    i = i + 1
selesai

tunjuk ""
tunjuk "Akurasi Model Pokok Keputusan: " + teks(bundar(ketepatan * 100)) + "%"

# 5. Uji Simpan dan Muat Model
ada fail_model = "model_pokok_iris.json"
model.simpan(fail_model)
ada model_muat = PokokKeputusan(4)
model_muat.muat(fail_model)
ada ramalan_sedia = model_muat.ramal([[5.0, 3.5, 1.4, 0.2]])
tunjuk "Inferens Sampel Baharu [5.0, 3.5, 1.4, 0.2] => " + iris["nama_kelas"][ramalan_sedia[0]]

# Padam fail sementara
padamFail(fail_model)
tunjuk "Ujian Pokok Keputusan berjaya sepenuhnya!"
