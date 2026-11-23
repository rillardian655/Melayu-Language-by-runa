# ==============================================================================
# STRUKTUR DATA KLASIK (TIMBUNAN & GILIRAN) - MELAYU v0.3.0
# Implementasi Berorientasikan Objek: Stack (LIFO) & Queue (FIFO)
# ==============================================================================

# 1. Struktur Data Timbunan (Stack - Masuk Terakhir, Keluar Dahulu)
kelas Timbunan
    bina()
        diri.item = []
    selesai

    kerja sorong(nilai)
        diri.item.tambah(nilai)
    selesai

    kerja pop()
        jika diri.item.panjang() sama dengan 0
            pulang kosong
        selesai
        ada indeksTerakhir = diri.item.panjang() tolak 1
        pulang diri.item.buang(indeksTerakhir)
    selesai

    kerja intai()
        jika diri.item.panjang() sama dengan 0
            pulang kosong
        selesai
        pulang diri.item[-1]
    selesai

    kerja saiz()
        pulang diri.item.panjang()
    selesai

    kerja adakahKosong()
        pulang diri.item.panjang() sama dengan 0
    selesai
selesai

tunjuk "=== 1. UJIAN TIMBUNAN (STACK) ==="
ada timbun = Timbunan()
timbun.sorong("Buku Sejarah")
timbun.sorong("Buku Sains")
timbun.sorong("Buku Matematik")

tunjuk "Saiz timbunan:", timbun.saiz()
tunjuk "Elemen teratas (intai):", timbun.intai()
tunjuk "Keluarkan teratas (pop):", timbun.pop()
tunjuk "Keluarkan seterusnya (pop):", timbun.pop()
tunjuk "Baki saiz:", timbun.saiz()

# 2. Struktur Data Giliran (Queue - Masuk Dahulu, Keluar Dahulu)
kelas Giliran
    bina()
        diri.elemen = []
    selesai

    kerja masuk(nilai)
        diri.elemen.tambah(nilai)
    selesai

    kerja keluar()
        jika diri.elemen.panjang() sama dengan 0
            pulang kosong
        selesai
        pulang diri.elemen.buang(0)
    selesai

    kerja depan()
        jika diri.elemen.panjang() sama dengan 0
            pulang kosong
        selesai
        pulang diri.elemen[0]
    selesai

    kerja saiz()
        pulang diri.elemen.panjang()
    selesai
selesai

tunjuk ""
tunjuk "=== 2. UJIAN GILIRAN (QUEUE) ==="
ada giliranKaunter = Giliran()
giliranKaunter.masuk("Pelanggan A - Ali")
giliranKaunter.masuk("Pelanggan B - Siti")
giliranKaunter.masuk("Pelanggan C - Budi")

tunjuk "Bilangan dalam giliran:", giliranKaunter.saiz()
tunjuk "Pelanggan paling depan:", giliranKaunter.depan()
tunjuk "Layan pelanggan:", giliranKaunter.keluar()
tunjuk "Layan pelanggan:", giliranKaunter.keluar()
tunjuk "Baki dalam giliran:", giliranKaunter.saiz()
