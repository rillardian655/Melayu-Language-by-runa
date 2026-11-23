# ==============================================================================
# POLA REKA BENTUK: PEMERHATI (OBSERVER / PUB-SUB PATTERN) - MELAYU v0.3.0
# ==============================================================================

kelas PelangganBerita
    bina(nama)
        diri.nama = nama
    selesai

    kerja terimaKhabar(tajuk)
        tunjuk "[" + diri.nama + "] Terima berita terkini: " + tajuk
    selesai
selesai

kelas SaluranBerita
    bina(namaSaluran)
        diri.namaSaluran = namaSaluran
        diri.senaraiPelanggan = []
    selesai

    kerja langgan(pelanggan)
        diri.senaraiPelanggan.tambah(pelanggan)
    selesai

    kerja siarkan(tajukBerita)
        tunjuk ">>> Memancarkan dari " + diri.namaSaluran + ": " + tajukBerita
        untuk p dalam diri.senaraiPelanggan
            p.terimaKhabar(tajukBerita)
        selesai
    selesai
selesai

tunjuk "=== POLA REKA BENTUK PEMERHATI (OBSERVER) ==="
ada warta = SaluranBerita("Warta Serumpun")
ada ahli1 = PelangganBerita("Ali")
ada ahli2 = PelangganBerita("Siti")
ada ahli3 = PelangganBerita("Budi")

warta.langgan(ahli1)
warta.langgan(ahli2)
warta.langgan(ahli3)

warta.siarkan("MELAYU v0.3.0 Dilancarkan dengan Sokongan Penuh OOP!")
