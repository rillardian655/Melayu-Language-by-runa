# ==============================================================================
# CONTOH PENUH: PANGKALAN DATA + DEEP LEARNING ADAM (PANGKALAN + AKALML)
# ==============================================================================
# Menunjukkan penggunaan:
#   1. `ambil "pangkalan"` - Simpan dan cari rekod nasabah secara gigih
#   2. `ambil "akal"`      - Latih RangkaianAdam (ReLU + Softmax + Adam) pada Iris
# ==============================================================================

ambil "pangkalan"
ambil "akal"

benihRawak(42)

tunjuk "=========================================================="
tunjuk "   SISTEM AI + PANGKALAN DATA BAHASA MELAYU"
tunjuk "=========================================================="

# 1. Latih RangkaianAdam pada Dataset Iris (150 rekod, 3 kelas)
ada iris = muatIris()
ada penyekala = SkalaMinMaks()
ada X_skala = penyekala.latihTransformasi(iris["ciri"])
ada pecah = bahagiLatihUji(X_skala, iris["sasaran"], 0.2)

ada model = RangkaianAdam([4, 10, 3], 0.05)
model.latih(pecah[0], pecah[2], 200, salah)
ada akurasi = model.skor(pecah[1], pecah[3])
tunjuk "[1/3] Akurasi RangkaianAdam (ReLU + Softmax + Adam): " + teks(bundar(akurasi * 100)) + "%"

# 2. Simpan log ramalan ke dalam Pangkalan Data Terbenam
ada fail_db = "rekod_ramalan_db.json"
ada db = Pangkalan(fail_db)
db.ciptaJadual("ramalan_iris")

ada ramalan_uji = model.ramal(pecah[1])
ada i = 0
selagi i < 5 && i < panjang(ramalan_uji)
    ada kelas_teks = iris["nama_kelas"][ramalan_uji[i]]
    db.sisip("ramalan_iris", {
            "sampel": i + 1,
            "ramalan": kelas_teks,
            "sebenar": iris["nama_kelas"][pecah[3][i]]
    })
    i = i + 1
selesai

tunjuk "[2/3] Rekod disimpan dalam pangkalan: " + teks(db.bilang("ramalan_iris")) + " baris."
ada rekod_setosa = db.cari("ramalan_iris", {"ramalan": "Setosa"})
tunjuk "[3/3] Rekod diramal 'Setosa': " + teks(panjang(rekod_setosa))

padamFail(fail_db)
tunjuk "Selesai! Pangkalan + RangkaianAdam beroperasi dengan sempurna!"
