# ==============================================================================
# PAPAN PEMUKA WEB FULL-STACK: SELASIH (HTML/CSS/JS) + PANGKALAN + AKALML
# ==============================================================================
# Menunjukkan cara "collab" Bahasa Melayu sebagai pelayan backend bersama
# antaramuka web moden (HTML + CSS + JavaScript) menggunakan `balasHTML`
# dan REST API JSON (`balasJSON`).
# ==============================================================================

ambil "akal"
ambil "pangkalan"
ambil "selasih"

benihRawak(42)

tunjuk "=========================================================="
tunjuk "   PAPAN PEMUKA AI FULL-STACK: SELASIH + HTML/CSS/JS"
tunjuk "=========================================================="

# 1. Latih model RangkaianAdam pada dataset Iris (150 rekod)
ada iris = muatIris()
ada penyekala = SkalaMinMaks()
ada X_skala = penyekala.latihTransformasi(iris["ciri"])
ada model = RangkaianAdam([4, 10, 3], 0.05)
model.latih(X_skala, iris["sasaran"], 180, salah)
ada akurasi = bundar(model.skor(X_skala, iris["sasaran"]) * 100)
tunjuk "[1/3] Model RangkaianAdam sedia (Akurasi: " + teks(akurasi) + "%)"

# 2. Sediakan Pangkalan Data JSON tempatan
ada fail_db = "papan_pemuka_ai_db.json"
ada db = Pangkalan(fail_db)
db.ciptaJadual("log_ramalan")

# 3. Antaramuka HTML + CSS + JavaScript yang dilayani oleh Selasih
ada HALAMAN_HTML = "<!DOCTYPE html>
<html lang=\"ms\">
<head>
<meta charset=\"UTF-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
<meta name=\"color-scheme\" content=\"dark\" />
<title>Papan Pemuka AI — Bahasa Melayu + Selasih</title>
<style>
:root { color-scheme: dark; --bg: #0b1120; --card: #111827; --border: #1f2937; --accent: #22d3ee; --text: #f3f4f6; --muted: #9ca3af; }
    * { box-sizing: border-box; font-family: system-ui, -apple-system, sans-serif; }
    body { margin: 0; padding: 24px; background: var(--bg); color: var(--text); }
    .wrap { max-width: 860px; margin: 0 auto; display: grid; gap: 20px; }
    .card { background: var(--card); border: 1px solid var(--border); border-radius: 12px; padding: 20px; }
    h1 { margin: 0 0 6px; font-size: 1.4rem; color: var(--accent); }
    .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 12px; margin: 14px 0; }
    label { display: flex; flex-direction: column; gap: 6px; font-size: 0.85rem; color: var(--muted); }
    input { padding: 8px 10px; border-radius: 8px; border: 1px solid var(--border); background: #030712; color: var(--text); }
        button { padding: 10px 18px; border-radius: 8px; border: 0; background: var(--accent); color: #082f49; font-weight: 700; cursor: pointer; }
            table { width: 100%; border-collapse: collapse; margin-top: 10px; font-size: 0.9rem; }
            th, td { text-align: left; padding: 8px 10px; border-bottom: 1px solid var(--border); }
            </style>
            </head>
            <body>
            <div class=\"wrap\">
            <div class=\"card\">
            <h1>Papan Pemuka AI Bahasa Melayu</h1>
            <p style=\"color:var(--muted);margin:0\">Backend: Selasih + Pangkalan + RangkaianAdam | Frontend: HTML + CSS + JS</p>
            </div>
            <div class=\"card\">
            <h2 style=\"margin-top:0;font-size:1.05rem\">Uji Ramalan Bunga Iris</h2>
            <div class=\"grid\">
            <label>Panjang Kelopak (cm)<input id=\"f1\" type=\"number\" step=\"0.1\" value=\"5.1\" /></label>
            <label>Lebar Kelopak (cm)<input id=\"f2\" type=\"number\" step=\"0.1\" value=\"3.5\" /></label>
            <label>Panjang Mahkota (cm)<input id=\"f3\" type=\"number\" step=\"0.1\" value=\"1.4\" /></label>
            <label>Lebar Mahkota (cm)<input id=\"f4\" type=\"number\" step=\"0.1\" value=\"0.2\" /></label>
            </div>
            <button type=\"button\" onclick=\"hantarRamalan()\">Ramalkan &amp; Simpan ke DB</button>
            <p id=\"hasil\" style=\"margin-top:12px;font-weight:600\"></p>
            </div>
            <div class=\"card\">
            <h2 style=\"margin-top:0;font-size:1.05rem\">Sejarah Ramalan (Pangkalan DB)</h2>
            <table>
            <thead><tr><th>ID</th><th>Input Ciri</th><th>Ramalan</th></tr></thead>
            <tbody id=\"senarai\"></tbody>
            </table>
            </div>
            </div>
            <script>
            async function muatSejarah() {
                const res = await fetch('/api/sejarah');
                const data = await res.json();
                const tbody = document.getElementById('senarai');
                tbody.innerHTML = data.data.map(r =>
                    '<tr><td>' + r._id + '</td><td>[' + r.input.join(', ') + ']</td><td>' + r.ramalan + '</td></tr>'
                ).join('');
            }
            async function hantarRamalan() {
                const ciri = ['f1','f2','f3','f4'].map(id => parseFloat(document.getElementById(id).value));
                const res = await fetch('/api/ramal', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ ciri })
                });
                const data = await res.json();
                document.getElementById('hasil').textContent = data.kejayaan
                ? 'Ramalan: ' + data.ramalan + ' (Rekod #' + data.rekod_id + ')'
                    : 'Ralat: ' + data.ralat;
                    await muatSejarah();
                }
                muatSejarah();
                </script>
                </body>
                </html>"

                # 4. Pengendali Laluan Web Selasih
                kerja kendaliUtama(req)
                    pulang balasHTML(HALAMAN_HTML, 200)
                selesai

                kerja kendaliSejarah(req)
                    ada rekod = db.cariSemua("log_ramalan")
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
                    ada rekod = db.sisip("log_ramalan", {"input": badan["ciri"], "ramalan": nama_kelas})
                    pulang balasJSON({"kejayaan": betul, "ramalan": nama_kelas, "rekod_id": rekod["_id"]}, 200)
                selesai

                ada app = PelayanWeb(8095)
                app.guna(pengantaraLogger)
                app.guna(pengantaraKeselamatan)
                app.dapatkan("/", kendaliUtama)
                app.dapatkan("/api/sejarah", kendaliSejarah)
                app.hantar("/api/ramal", kendaliRamal)

                # 5. Simulasi ujian tempatan (tanpa menyekat terminal)
                ada ujian_skala = penyekala.transformasi([[6.7, 3.0, 5.2, 2.3]])
                ada ujian_kelas = iris["nama_kelas"][model.ramal(ujian_skala)[0]]
                db.sisip("log_ramalan", {"input": [6.7, 3.0, 5.2, 2.3], "ramalan": ujian_kelas})
                tunjuk "[2/3] Ujian inferens & simpan DB: " + ujian_kelas + " (Jumlah rekod: " + teks(db.bilang("log_ramalan")) + ")"
                padamFail(fail_db)

                tunjuk "[3/3] Papan pemuka web siap dihidupkan melalui app.dengar() pada port 8095."
