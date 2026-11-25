# ==============================================================================
# PIPELINE MACHINE LEARNING LENGKAP: JADUAL DATA + HUTAN RAWAK (RANDOM FOREST)
# ==============================================================================
# Menunjukkan aliran kerja data sains sebenar dalam Bahasa Melayu:
#   1. Pembersihan Data dengan JadualData (DataFrame ala Pandas)
#   2. Pengkodan Kategori dengan PengkodLabel
#   3. Latihan Ensemble HutanRawak (Random Forest)
#   4. Penilaian Metrik (Akurasi, Matriks Kekeliruan, Skor F1)
#   5. Simpan & Muat Model ke Cakera
# ==============================================================================

ambil "akal"

tunjuk "=========================================================="
tunjuk "   PIPELINE AKALML: JADUAL DATA & HUTAN RAWAK (ENSEMBLE)"
tunjuk "=========================================================="

# 1. Sediakan JadualData (mengandungi data teks & nilai kosong)
ada jadual = JadualData(
    ["umur", "pendapatan_juta", "skor_kredit", "bandar", "kelulusan"],
    [
        [24.0, 4.5,  510.0, "KL",        "Ditolak"],
        [35.0, 15.0, 760.0, "Jakarta",   "Diluluskan"],
        [22.0, kosong, 490.0, "KL",      "Ditolak"],
        [42.0, 22.0, 810.0, "Singapura", "Diluluskan"],
        [29.0, 6.0,  550.0, "Jakarta",   "Ditolak"],
        [39.0, 18.0, 790.0, "KL",        "Diluluskan"],
        [45.0, 25.0, 830.0, "Singapura", "Diluluskan"],
        [21.0, 3.8,  480.0, "Jakarta",   "Ditolak"]
    ]
)

tunjuk "[1/5] Membersihkan nilai hilang dan meringkaskan JadualData..."
jadual.isiKosong("pendapatan_juta", 4.0)
ada info = jadual.ringkasan()
tunjuk "  Bilangan baris : " + teks(info["bilangan_baris"])
tunjuk "  Purata skor    : " + teks(bundar(info["lajur"]["skor_kredit"]["purata"]))

# 2. Tukar sasaran teks kepada angka dengan PengkodLabel
tunjuk "[2/5] Mengekod label kategori sasaran..."
ada pengkod = PengkodLabel()
ada y = pengkod.latihTransformasi(jadual.dapatkanLajur("kelulusan"))
ada X = jadual.pilihLajur(["umur", "pendapatan_juta", "skor_kredit"]).keMatriks()

# 3. Latih Ensemble HutanRawak (9 pokok keputusan, kedalaman maksimum 3)
tunjuk "[3/5] Melatih HutanRawak (9 Pokok Keputusan)..."
ada hutan = HutanRawak(9, 3)
hutan.latih(X, y)

# 4. Penilaian Model & Matriks Kekeliruan
ada ramalan_latih = hutan.ramal(X)
ada akurasi = hutan.skor(X, y)
ada f1 = skorF1(y, ramalan_latih, 1)
tunjuk "[4/5] Akurasi HutanRawak: " + teks(bundar(akurasi * 100)) + "% | Skor F1: " + teks(f1)

# 5. Simpan, Muat Semula & Uji Ramalan Baharu
ada fail_model = "model_hutan_kredit.json"
hutan.simpan(fail_model)
ada hutan_sedia = HutanRawak(9, 3)
hutan_sedia.muat(fail_model)

ada pemohon_baharu = [
    [40.0, 19.5, 800.0],
    [23.0, 4.2,  500.0]
]
ada kod_ramalan = hutan_sedia.ramal(pemohon_baharu)
ada keputusan_teks = pengkod.songsang(kod_ramalan)

tunjuk "[5/5] Ramalan Pemohon Baharu (Model Dimuatkan):"
tunjuk "  Pemohon 1 [40 thn, 19.5jt, skor 800] -> " + keputusan_teks[0]
tunjuk "  Pemohon 2 [23 thn, 4.2jt,  skor 500] -> " + keputusan_teks[1]

padamFail(fail_model)
tunjuk "Selesai! Pipeline JadualData + HutanRawak berfungsi cemerlang!"

