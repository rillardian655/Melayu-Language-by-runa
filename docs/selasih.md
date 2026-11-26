# Panduan Rujukan Kerangka Web Selasih (`ambil "selasih"`)

**Selasih** ialah kerangka kerja pelayan HTTP/REST API native tanpa kebergantungan luar, lengkap dengan sokongan penghalaan, pengantara (*middleware*), CORS, penapis XSS, dan klien HTTP/HTTPS.

## 1. Pelayan Web & Penghalaan

```melayu
ambil "selasih"

kerja kendaliUtama(req)
    pulang balasHTML("<h1>Selamat Datang ke Selasih</h1>", 200)
selesai

kerja kendaliAPI(req)
    ada data = req.json()
    pulang balasJSON({"diterima": data}, 200)
selesai

ada app = PelayanWeb(8080)
app.guna(pengantaraLogger)
app.guna(pengantaraKeselamatan)
app.dapatkan("/", kendaliUtama)
app.hantar("/api/data", kendaliAPI)
app.dengar()
```

## 2. Pembantu Respons & Keselamatan

* `balasJSON(data, status)`: Mengembalikan JSON dengan `Content-Type: application/json`.
* `balasHTML(html, status)`: Mengembalikan halaman HTML.
* `balasTeks(teks, status)`: Mengembalikan teks biasa.
* `balasCORS(status)`: Respons preflight `OPTIONS`.
* `selitHTML(input)`: Menyahkod aksara khas HTML (`&`, `<`, `>`, `"`, `'`) untuk mencegah serangan XSS.
* `pengantaraKeselamatan(req)`: Menyekat cubaan *path traversal* (`..`) dan suntikan `<script>` pada laluan URL.

## 3. Klien HTTP & HTTPS

```melayu
ada res = hantarPermintaanHTTP("GET", "https://example.com/")
tunjuk res["status"], res["badan"]
```
Menyokong `https://` melalui `rustls` serta dilengkapi sekatan anti-SSRF terhadap alamat metadata awan.
