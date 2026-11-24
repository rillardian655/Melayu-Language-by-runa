# ==============================================================================
# DEMONSTRASI AKALML: KERANGKA KERJA PEMBELAJARAN MESIN BAHASA MELAYU (.ML)
# ==============================================================================

ambil "akal"

tunjuk "=================================================="
tunjuk "         AKALML v1.0.0 - BAHASA MELAYU (.ML)      "
tunjuk "  Kerangka Kerja Pembelajaran Mesin Serumpun      "
tunjuk "=================================================="
tunjuk ""

# ------------------------------------------------------------------------------
# 1. OPERASI MATRIKS NATIVE (PANTAS & TEPAT)
# ------------------------------------------------------------------------------
tunjuk "--- 1. OPERASI ALJABAR LINEAR & MATRIKS NATIVE ---"

ada A = Matriks([
    [1.0, 2.0],
    [3.0, 4.0]
])

ada B = Matriks([
    [2.0, 0.0],
    [1.0, 2.0]
])

ada C = A.darab(B)
tunjuk "Matriks A didarab Matriks B:"
C.paparkan()

ada At = A.transposisi()
tunjuk "Transposisi Matriks A (A^T):"
At.paparkan()

ada A_inv = A.songsang()
tunjuk "Songsangan Matriks A (A^-1):"
A_inv.paparkan()
tunjuk ""

# ------------------------------------------------------------------------------
# 2. REGRESI LINEAR (PREDIKSI HARGA RUMAH)
# Formula: y = w1*keluasan + w2*bilik + bias
# ------------------------------------------------------------------------------
tunjuk "--- 2. MODEL REGRESI LINEAR (PREDIKSI HARGA) ---"

# Ciri: [Keluasan (m2), Bilangan Bilik]
ada X_rumah = [
    [50.0, 1.0],
    [70.0, 2.0],
    [90.0, 2.0],
    [120.0, 3.0],
    [150.0, 3.0],
    [180.0, 4.0],
    [220.0, 4.0],
    [250.0, 5.0]
]

# Sasaran: Harga (dalam Ribu Ringgit / Juta Rupiah)
ada y_harga = [150.0, 205.0, 260.0, 340.0, 420.0, 500.0, 610.0, 690.0]

ada regresi = RegresiLinear(betul)
regresi.latih(X_rumah, y_harga)

tunjuk "Pintas (Bias Model) :", regresi.pintas
tunjuk "Pekali Ciri (Bobot) :", regresi.pekali

ada rumah_baru = [
    [100.0, 2.0], # Rumah 100m2, 2 bilik
    [200.0, 4.0]  # Rumah 200m2, 4 bilik
]

ada ramalan_harga = regresi.ramal(rumah_baru)
tunjuk "Ramalan Rumah 100m2, 2 bilik :", bundar(ramalan_harga[0]), "k"
tunjuk "Ramalan Rumah 200m2, 4 bilik :", bundar(ramalan_harga[1]), "k"
tunjuk "Skor Ketepatan R2 Model      :", regresi.skor(X_rumah, y_harga)
tunjuk ""

# ------------------------------------------------------------------------------
# 3. PENGELAS KNN (KLASIFIKASI KEMIRIPAN)
# ------------------------------------------------------------------------------
tunjuk "--- 3. MODEL PENGELAS KNN (PENGELASAN KEMIRIPAN) ---"

# Contoh: Pengelasan Risiko Pelanggan (Pendapatan (k), Hutang (k))
ada X_pelanggan = [
    [2.0, 8.0],
    [3.0, 7.0],
    [2.5, 9.0],
    [8.0, 2.0],
    [9.0, 1.5],
    [7.5, 3.0]
]

ada y_risiko = ["Tinggi", "Tinggi", "Tinggi", "Rendah", "Rendah", "Rendah"]

ada model_knn = PengelasKNN(3)
model_knn.latih(X_pelanggan, y_risiko)

ada pemohon_baru = [
    [2.2, 7.5], # Hampir ke kelompok risiko tinggi
    [8.5, 2.2]  # Hampir ke kelompok risiko rendah
]

ada hasil_knn = model_knn.ramal(pemohon_baru)
tunjuk "Pemohon 1 (Pendapatan Rendah, Hutang Tinggi) -> Risiko :", hasil_knn[0]
tunjuk "Pemohon 2 (Pendapatan Tinggi, Hutang Rendah) -> Risiko :", hasil_knn[1]
tunjuk "Akurasi Latihan KNN :", (model_knn.skor(X_pelanggan, y_risiko) * 100), "%"
tunjuk ""

# ------------------------------------------------------------------------------
# 4. PENGELOMPOKAN K-MEANS (UNSUPERVISED MACHINE LEARNING)
# ------------------------------------------------------------------------------
tunjuk "--- 4. PENGELOMPOKAN K-MEANS (SEGMENTASI DATA) ---"

ada data_titik = [
    [1.0, 1.0], [1.5, 2.0], [2.0, 1.0], # Kelompok 1
    [8.0, 8.0], [9.0, 7.0], [8.5, 9.0]  # Kelompok 2
]

ada kmeans = PengelompokanKMeans(2, 20)
kmeans.latih(data_titik)

tunjuk "Pusat Kelompok (Centroids) Berjaya Dijumpai:"
ada c_idx = 0
untuk pusat dalam kmeans.pusat
    tunjuk "  Pusat", c_idx, ":", pusat
    c_idx = c_idx + 1
selesai

ada titik_baru = [[1.2, 1.3], [8.8, 8.2]]
ada ramalan_kelompok = kmeans.ramal(titik_baru)
tunjuk "Titik [1.2, 1.3] dimasukkan ke kelompok :", ramalan_kelompok[0]
tunjuk "Titik [8.8, 8.2] dimasukkan ke kelompok :", ramalan_kelompok[1]
tunjuk ""

tunjuk "Semua ujian modul AkalML berjaya disempurnakan dengan pantas!"

