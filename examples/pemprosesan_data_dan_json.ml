# ==============================================================================
# Program Contoh: Pemprosesan Data, Format JSON & Utiliti Masa (v0.2.0)
# ==============================================================================

tunjuk "=".ulang(60)
tunjuk " DEMO PEMPROSESAN DATA & ENJIN JSON BAHASA MELAYU v0.2.0"
tunjuk "=".ulang(60)

# 1. Tarikh dan Masa Semasa
ada tarikh = tarikhKini()
ada masa = masaKini()
ada cap = capMasa()
tunjuk "Tarikh Hari Ini : " + tarikh
tunjuk "Masa Semasa     : " + masa
tunjuk "Cap Masa Unix   : " + cap
tunjuk ""

# 2. Data Mentah JSON
ada teksPayload = "{\"syarikat\": \"Nusantara Tech\", \"transaksi\": [{\"id\": 1, \"produk\": \"Kopi Luwak\", \"harga\": 35.0, \"status\": \"selesai\"}, {\"id\": 2, \"produk\": \"Teh Tarik\", \"harga\": 8.5, \"status\": \"selesai\"}, {\"id\": 3, \"produk\": \"Dodol Durian\", \"harga\": 25.0, \"status\": \"batal\"}, {\"id\": 4, \"produk\": \"Keropok Lekor\", \"harga\": 15.0, \"status\": \"selesai\"}]}"

tunjuk "[1] Menghurai Data JSON Mentah..."
ada pesanan = bacaJSON(teksPayload)
tunjuk "Nama Syarikat : " + pesanan["syarikat"]
tunjuk "Jumlah Rekod  : " + pesanan["transaksi"].panjang()
tunjuk ""

# 3. Operasi Fungsian Senarai (Filter, Map, Reduce)
tunjuk "[2] Memproses Transaksi Yang Berjaya (Tapis & Petakan)..."

kerja hanyaSelesai(item)
    pulang item["status"] == "selesai"
selesai

ada transaksiSah = pesanan["transaksi"].tapis(hanyaSelesai)
tunjuk "Bilangan transaksi berjaya: " + transaksiSah.panjang()

kerja ambilHarga(item)
    pulang item["harga"]
selesai

ada senaraiHarga = transaksiSah.petakan(ambilHarga)
tunjuk "Senarai harga yang dipetakan: " + senaraiHarga.cantum(", ")

kerja jumlahkan(terkumpul, nilai)
    pulang terkumpul tambah nilai
selesai

ada jumlahJualan = senaraiHarga.kiraSemua(jumlahkan, 0)
tunjuk "Jumlah Pendapatan Bersih: RM" + jumlahJualan
tunjuk ""

# 4. Pemformatan Teks Lanjutan
tunjuk "[3] Menjana Invois Kemas..."
tunjuk "-".ulang(45)
tunjuk "ID".selitBelakang(6) + "PRODUK".selitBelakang(22) + "HARGA (RM)".selitDepan(15)
tunjuk "-".ulang(45)

kerja paparItem(trx)
    ada idStr = teks(trx["id"]).selitDepan(3, "0")
    ada namaProd = trx["produk"].hurufAwalBesar().selitBelakang(22)
    ada hargaStr = teks(trx["harga"]).selitDepan(12)
    tunjuk idStr.selitBelakang(6) + namaProd + hargaStr
selesai

transaksiSah.setiapSatu(paparItem)
tunjuk "-".ulang(45)
tunjuk "JUMLAH KESELURUHAN:".selitBelakang(28) + teks(jumlahJualan).selitDepan(15)
tunjuk "-".ulang(45)
tunjuk ""

# 5. Menukar Semula Ke Format JSON Padat & Cantik
tunjuk "[4] Menjana Ringkasan JSON Baharu..."
ada ringkasan = {
    "tarikh": tarikh,
    "masa": masa,
    "jumlahPesanan": transaksiSah.panjang(),
    "totalPendapatan": jumlahJualan
}

ada jsonRingkasan = tulisJSON(ringkasan, betul)
tunjuk jsonRingkasan
tunjuk ""
tunjuk "Program pemprosesan data berjaya disempurnakan!".hurufAwalBesar()
