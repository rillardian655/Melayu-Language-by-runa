# ==============================================================================
# DEMONSTRASI AKALML LANJUTAN: PEMBELAJARAN MESIN TINGKAT MENENGAH-TINGGI (MID-HIGH)
# Bahasa Melayu (.ml) - Kerangka Kerja AkalML
# ==============================================================================

ambil "akal"

tunjuk "======================================================================"
tunjuk "          AKALML LANJUTAN: DEEP LEARNING & STATISTIK INFERENSI        "
tunjuk "           Bahasa Pengaturcaraan Melayu (.ml) - Versi 1.0.0           "
tunjuk "======================================================================"
tunjuk ""

# ==============================================================================
# BAHAGIAN 1: JARINGAN SARAF TIRUAN (DEEP NEURAL NETWORK / MULTI-LAYER PERCEPTRON)
# Menyelesaikan Masalah Tak-Linear Klasik XOR Melalui Perambatan Ke Belakang
# ==============================================================================
tunjuk "----------------------------------------------------------------------"
tunjuk " 1. JARINGAN SARAF TIRUAN (MULTI-LAYER PERCEPTRON & BACKPROPAGATION)  "
tunjuk "----------------------------------------------------------------------"
tunjuk "Membina arkitektur rangkaian saraf tiruan: [2 masukan -> 4 tersembunyi -> 1 keluaran]"
tunjuk "Fungsi Pengaktifan: Sigmoid"
tunjuk "Algoritma Pengoptimuman: Penurunan Kecerunan (Gradient Descent)"
tunjuk ""

# Dataset Masalah XOR (Eksklusif ATAU - Masalah Tak-Linear Klasik)
ada X_xor = [
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0]
]

ada y_xor = [
    [0.0],
    [1.0],
    [1.0],
    [0.0]
]

# Bina model: 2 neuron masukan, 4 neuron tersembunyi, 1 neuron keluaran, kadar belajar = 0.85
ada nn = JaringanSaraf([2, 4, 1], 0.85)

tunjuk "Memulakan latihan model (2500 zaman)..."
nn.latih(X_xor, y_xor, 2500, betul)
tunjuk ""

# Ramalan kebarangkalian dan pengkelasan
ada ramalan_kebarangkalian = nn.ramalKebarangkalian(X_xor)
ada ramalan_kelas = nn.ramal(X_xor)

tunjuk "Keputusan Ramalan Masalah XOR:"
ada i = 0
selagi i < panjang(X_xor)
    ada masukan_str = "[" + teks(X_xor[i][0]) + ", " + teks(X_xor[i][1]) + "]"
    ada kebarangkalian_str = teks(ramalan_kebarangkalian[i][0])
    ada kelas_str = teks(ramalan_kelas[i])
    ada sasaran_str = teks(y_xor[i][0])
    tunjuk "  Masukan: " + masukan_str + " -> Kebarangkalian: " + kebarangkalian_str + " | Ramal: " + kelas_str + " (Sebenar: " + sasaran_str + ")"
    i = i + 1
selesai

ada y_sebenar_vektor = [0, 1, 1, 0]
ada ketepatan_nn = nn.skor(X_xor, y_sebenar_vektor)
tunjuk ""
tunjuk "Ketepatan Jaringan Saraf (Akurasi): " + teks(ketepatan_nn * 100) + "%"
tunjuk ""

# ==============================================================================
# BAHAGIAN 2: PENGELAS BAYES NAIF GAUSSIAN (GAUSSIAN NAIVE BAYES)
# Klasifikasi Kebarangkalian Pelbagai Ciri Diagnosis Kesihatan
# ==============================================================================
tunjuk "----------------------------------------------------------------------"
tunjuk " 2. PENGELAS BAYES NAIF GAUSSIAN (GAUSSIAN NAIVE BAYES CLASSIFIER)   "
tunjuk "----------------------------------------------------------------------"
tunjuk "Diagnosis Risiko Penyakit Berdasarkan 3 Ciri Biometrik:"
tunjuk "  [Tekanan Darah Sistolik (mmHg), Glukosa Darah (mg/dL), Umur (Tahun)]"
tunjuk ""

# Dataset Latihan Diagnosis Pesakit
ada X_pesakit = [
    [115.0,  85.0, 25.0],  # Sihat
    [118.0,  90.0, 28.0],  # Sihat
    [120.0,  92.0, 32.0],  # Sihat
    [122.0,  88.0, 30.0],  # Sihat
    [125.0,  95.0, 35.0],  # Sihat
    [155.0, 160.0, 58.0],  # Berisiko
    [160.0, 175.0, 62.0],  # Berisiko
    [150.0, 165.0, 55.0],  # Berisiko
    [168.0, 180.0, 65.0],  # Berisiko
    [158.0, 170.0, 60.0]   # Berisiko
]

ada y_pesakit = [
    "Sihat", "Sihat", "Sihat", "Sihat", "Sihat",
    "Berisiko", "Berisiko", "Berisiko", "Berisiko", "Berisiko"
]

ada model_bayes = PengelasBayesNaif()
model_bayes.latih(X_pesakit, y_pesakit)
tunjuk "Model Pengelas Bayes Naif berjaya dilatih ke atas sampel pesakit."

# Pesakit baharu untuk ujian diagnosis
ada pesakit_baharu = [
    [119.0,  89.0, 29.0], # Ciri pesakit A (dijangka Sihat)
    [162.0, 172.0, 61.0], # Ciri pesakit B (dijangka Berisiko)
    [123.0,  94.0, 31.0]  # Ciri pesakit C (dijangka Sihat)
]

ada diagnosis = model_bayes.ramal(pesakit_baharu)

tunjuk ""
tunjuk "Diagnosis Pesakit Baharu Menggunakan Bayes Naif:"
tunjuk "  Pesakit A [119 mmHg, 89 mg/dL, 29 thn]  -> Hasil Diagnosis: " + diagnosis[0]
tunjuk "  Pesakit B [162 mmHg, 172 mg/dL, 61 thn] -> Hasil Diagnosis: " + diagnosis[1]
tunjuk "  Pesakit C [123 mmHg, 94 mg/dL, 31 thn]  -> Hasil Diagnosis: " + diagnosis[2]
tunjuk ""

# ==============================================================================
# BAHAGIAN 3: METRIK PENILAIAN LANJUTAN (CONFUSION MATRIX, PRECISION, RECALL, F1)
# Mengukur Prestasi Model Pengelasan Secara Profesional
# ==============================================================================
tunjuk "----------------------------------------------------------------------"
tunjuk " 3. METRIK PENILAIAN MODEL LANJUTAN (EVALUATION METRICS)              "
tunjuk "----------------------------------------------------------------------"

# Data ujian untuk penilaian komprehensif
ada y_sebenar_ujian = ["Berisiko", "Berisiko", "Sihat", "Berisiko", "Sihat", "Sihat", "Berisiko", "Berisiko", "Sihat", "Berisiko"]
ada y_ramal_ujian   = ["Berisiko", "Berisiko", "Berisiko", "Berisiko", "Sihat", "Sihat", "Sihat",    "Berisiko", "Sihat", "Berisiko"]

ada matriks_keliru = matriksKekeliruan(y_sebenar_ujian, y_ramal_ujian, "Berisiko")
ada skor_kejituan  = kejituan(y_sebenar_ujian, y_ramal_ujian, "Berisiko")
ada skor_kepekaan  = kepekaan(y_sebenar_ujian, y_ramal_ujian, "Berisiko")
ada nilai_f1       = skorF1(y_sebenar_ujian, y_ramal_ujian, "Berisiko")
ada nilai_akurasi  = skorAkurasi(y_sebenar_ujian, y_ramal_ujian)

tunjuk "Matriks Kekeliruan (Confusion Matrix) untuk kelas 'Berisiko':"
tunjuk "  Benar Positif  (TP) : " + teks(matriks_keliru["TP"])
tunjuk "  Palsu Positif  (FP) : " + teks(matriks_keliru["FP"])
tunjuk "  Benar Negatif  (TN) : " + teks(matriks_keliru["TN"])
tunjuk "  Palsu Negatif  (FN) : " + teks(matriks_keliru["FN"])
tunjuk ""
tunjuk "Skor Metrik Statistik:"
tunjuk "  Kejituan / Precision (TP / (TP + FP)) : " + teks(skor_kejituan * 100) + "%"
tunjuk "  Kepekaan / Recall    (TP / (TP + FN)) : " + teks(skor_kepekaan * 100) + "%"
tunjuk "  Skor F1-Score        (Purata Harmonik): " + teks(nilai_f1)
tunjuk "  Akurasi Keseluruhan                   : " + teks(nilai_akurasi * 100) + "%"
tunjuk ""
tunjuk "======================================================================"
tunjuk "  Tahniah bro! Semua modul AkalML Mid-High berjaya beroperasi lancar!  "
tunjuk "======================================================================"

