# ==============================================================================
# CONTOH PENUH FULL-STACK: SELASIH WEB + PANGKALAN DB + RANGKAIAN ADAM
# ==============================================================================
# Menunjukkan integrasi lengkap 3 kerangka kerja rasmi Bahasa Melayu:
#   1. `akal`      - Latih model RangkaianAdam pada dataset Iris
#   2. `pangkalan` - Simpan log inferens ke pangkalan data JSON tempatan
#   3. `selasih`   - Dedahkan endpoint REST API (/ramal, /sejarah) dengan middleware
# ==============================================================================

ambil "akal"
ambil "pangkalan"
ambil "selasih"

benihRawak(42)

tunjuk "=========================================================="
tunjuk "   SERVIS FULL-STACK: SELASIH + PANGKALAN + RANGKAIAN ADAM"
tunjuk "=========================================================="

# 1. Latih model RangkaianAdam
ada iris = muatIris()
ada penyekala = SkalaMinMaks()
ada X_skala = penyekala.latihTransformasi(iris["ciri"])
ada model = RangkaianAdam([4, 10, 3], 0.05)
model.latih(X_skala, iris["sasaran"], 180, salah)
tunjuk "[1/3] Model RangkaianAdam sedia (Akurasi: " + teks(bundar(model.skor(X_skala, iris["sasaran"]) * 100)) + "%)"

# 2. Sediakan Pangkalan Data untuk menyimpan sejarah ramalan
ada fail_db = "sejarah_servis_ai.json"
ada db = Pangkalan(fail_db)
db.ciptaJadual("sejarah")

# 3. Pengendali laluan Web Selasih
kerja kendaliSejarah(req)
    ada rekod = db.cariSemua("sejarah")
    pulang balasJSON({"jumlah": panjang(rekod), "data": rekod}, 200)
selesai

kerja kendaliRamal(req)
    ada badan = req.json()
    jika badan == kosong || bukan badan.adaKunci("ciri")
        pulang balasJSON({"ralat": "Sila hantar JSON {'ciri': [f1, f2, f3, f4]}"}, 400)
    selesai
    ada ciri_skala = penyekala.transformasi([badan["ciri"]])
    ada kelas_idx = model.ramal(ciri_skala)[0]
    ada nama_kelas = iris["nama_kelas"][kelas_idx]
    ada rekod = db.sisip("sejarah", {"input": badan["ciri"], "ramalan": nama_kelas})
    pulang balasJSON({"kejayaan": betul, "ramalan": nama_kelas, "rekod_id": rekod["_id"]}, 200)
selesai

ada app = PelayanWeb(8090)
app.guna(pengantaraLogger)
app.guna(pengantaraKeselamatan)
app.dapatkan("/sejarah", kendaliSejarah)
app.hantar("/ramal", kendaliRamal)

# Uji simulasi tempatan
ada ujian_skala = penyekala.transformasi([[5.1, 3.5, 1.4, 0.2]])
ada ujian_kelas = iris["nama_kelas"][model.ramal(ujian_skala)[0]]
db.sisip("sejarah", {"input": [5.1, 3.5, 1.4, 0.2], "ramalan": ujian_kelas})
tunjuk "[2/3] Ujian inferens & simpan DB: " + ujian_kelas + " (Jumlah rekod: " + teks(db.bilang("sejarah")) + ")"
padamFail(fail_db)

tunjuk "[3/3] Servis sedia dihidupkan melalui app.dengar() pada port 8090."
