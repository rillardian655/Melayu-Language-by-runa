# ==============================================================================
# SISTEM PENGURUSAN INVENTORI (OOP) - MELAYU v0.3.0
# ==============================================================================

kelas Produk
    bina(kod, nama, harga, stok)
        diri.kod = kod
        diri.nama = nama
        diri.harga = harga
        diri.stok = stok
    selesai

    kerja tambahStok(kuantiti)
        diri.stok = diri.stok + kuantiti
        pulang diri.stok
    selesai

    kerja tolakStok(kuantiti)
        jika kuantiti > diri.stok
            pulang -1
        selesai
        diri.stok = diri.stok - kuantiti
        pulang diri.stok
    selesai

    kerja nilaiStok()
        pulang diri.harga * diri.stok
    selesai
selesai

tunjuk "=== PENGURUSAN INVENTORI KEDAI ==="
ada barang1 = Produk("PRD-01", "Kopi Kampung", 15, 50)
ada barang2 = Produk("PRD-02", "Teh Tarik Segera", 12, 40)

tunjuk "Produk 1:", barang1.nama, "| Stok Awal:", barang1.stok
tunjuk "Nilai Stok Produk 1: RM" + teks(barang1.nilaiStok())

barang1.tambahStok(20)
tunjuk "Selepas masuk 20 unit:", barang1.stok

ada baki = barang1.tolakStok(15)
tunjuk "Jual 15 unit, baki stok:", baki
tunjuk "Nilai stok akhir: RM" + teks(barang1.nilaiStok())
