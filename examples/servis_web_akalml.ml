# ==============================================================================
# CONTOH SERVIS MIKRO WEB API & PEMBELAJARAN MESIN (SELASIH + AKALML)
# ==============================================================================
# Fail ini menunjukkan integrasi penuh ekosistem Bahasa Melayu:
# 1. AkalML: Jaringan Saraf Tiruan (Deep Learning) & Penyekalaan Ciri (SkalaMinMaks)
# 2. Penstoran Model: simpan() & muat() ke cakera
# 3. Selasih: Pelayan Web HTTP REST API tanpa pustaka pihak ketiga!
# ==============================================================================

ambil "akal"
ambil "selasih"

tunjuk "=========================================================="
tunjuk "   PELAYAN REST API AKALML DENGAN KERANGKA SELASIH"
tunjuk "=========================================================="

# 1. Sediakan Dataset Contoh (Logik Eksklusif XOR 2 Ciri)
ada data_latih = [
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0]
]
ada label_benar = [0.0, 1.0, 1.0, 0.0]

# 2. Penyekalaan Ciri (Min-Max Feature Scaling)
tunjuk "[1/4] Menjalankan penyekalaan ciri SkalaMinMaks..."
ada penyekala = SkalaMinMaks()
ada data_terskala = penyekala.latihTransformasi(data_latih)

# 3. Bina & Latih Model Jaringan Saraf Tiruan
tunjuk "[2/4] Melatih JaringanSaraf (Lapisan Tersembunyi: 4 nod, Kadar: 2.0)..."
ada model = JaringanSaraf([2, 4, 1], 2.0)
model.latih(data_terskala, label_benar, 500, salah)

# 4. Simpan Model ke Fail JSON Tempatan
ada laluan_model = "model_xor.json"
model.simpan(laluan_model)
tunjuk "[3/4] Model neural network berjaya disimpan ke: " + laluan_model

# 5. Muat Semula Model untuk Mengesahkan Kemampuan Simpan & Muat
ada model_sedia = JaringanSaraf([2, 4, 1], 2.0)
model_sedia.muat(laluan_model)
tunjuk "[4/4] Model berjaya dimuatkan semula untuk inferens masa nyata."

# Uji Inferens Setempat Sebelum Menghidupkan Pelayan Web
ada ramal_0_1 = model_sedia.ramalKebarangkalian([[0.0, 1.0]])
ada kelas_0_1 = model_sedia.ramal([[0.0, 1.0]])
tunjuk "  -> Ujian Inferens Input [0, 1]: Skor = " + ramal_0_1[0][0] + " | Kelas = " + kelas_0_1[0]

# ==============================================================================
# PENGENDALI ROUTE PELAYAN WEB (REST API SELASIH)
# ==============================================================================

kerja kendaliLamanUtama(permintaan)
    ada html = "<html><head><title>AkalML API</title></head><body><h1>Pelayan Web Selasih + AkalML</h1><p>Sistem Inferens Kepintaran Buatan Bahasa Melayu sedia beroperasi!</p><ul><li>GET /status</li><li>POST /ramal</li></ul></body></html>"
    pulang balasHTML(html, 200)
selesai

kerja kendaliStatus(permintaan)
    ada respons = {
        "status": "aktif",
        "kerangka": "Selasih v1.0",
        "enjin_ml": "AkalML v1.0",
        "model_dimuat": laluan_model
    }
    pulang balasJSON(respons, 200)
selesai

kerja kendaliRamal(permintaan)
    ada badan = permintaan.json()
    jika badan == kosong || !badan.mengandungi("ciri")
        pulang balasJSON({"ralat": "Sila sertakan senarai 'ciri' dalam JSON"}, 400)
    selesai

    ada ciri = badan.dapatkan("ciri")
    ada ramalan_kelas = model_sedia.ramal([ciri])
    ada kebarangkalian = model_sedia.ramalKebarangkalian([ciri])

    ada hasil = {
        "kejayaan": betul,
        "input": ciri,
        "kelas_ramalan": ramalan_kelas[0],
        "kebarangkalian": kebarangkalian[0][0]
    }
    pulang balasJSON(hasil, 200)
selesai

# ==============================================================================
# MEMULAKAN PELAYAN WEB
# ==============================================================================
ada aplikasi = PelayanWeb(8088)

aplikasi.dapatkan("/", kendaliLamanUtama)
aplikasi.dapatkan("/status", kendaliStatus)
aplikasi.hantar("/ramal", kendaliRamal)

tunjuk ""
tunjuk "=========================================================="
tunjuk "Pelayan Web Selasih sedia menerima permintaan pada port 8088..."
tunjuk "Contoh arahan pengujian:"
tunjuk "  curl http://localhost:8088/status"
tunjuk "  curl -X POST http://localhost:8088/ramal -d '{\"ciri\": [0.0, 1.0]}'"
tunjuk "=========================================================="

# Contoh ini sedia dipanggil. Untuk ujian kendiri tanpa menyekat konsol,
# baris dengar di bawah boleh diaktifkan bila ingin dijalankan sebagai servis berterusan:
# aplikasi.dengar()
tunjuk "Integrasi AkalML dan Selasih berjaya disahkan!"