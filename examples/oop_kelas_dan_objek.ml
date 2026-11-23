# ==============================================================================
# DEMONSTRASI PENGATURCARAAN BERORIENTASIKAN OBJEK (OOP) - MELAYU v0.3.0
# Kata kunci: kelas, bina, diri, warisi, induk
# ==============================================================================

tunjuk "=== 1. PENETAPAN KELAS ASAS (Kenderaan) ==="

kelas Kenderaan
    bina(jenama, kelajuan_maks)
        diri.jenama = jenama
        diri.kelajuan_maks = kelajuan_maks
        diri.kelajuan_semasa = 0
    selesai

    kerja pandu(tambah_laju)
        diri.kelajuan_semasa = diri.kelajuan_semasa + tambah_laju
        jika diri.kelajuan_semasa > diri.kelajuan_maks
            diri.kelajuan_semasa = diri.kelajuan_maks
        selesai
        pulang diri.kelajuan_semasa
    selesai

    kerja tunjukInfo()
        pulang "Kenderaan [" + diri.jenama + "] - Kelajuan Semasa: " + teks(diri.kelajuan_semasa) + " km/j (Maks: " + teks(diri.kelajuan_maks) + " km/j)"
    selesai

    kerja hon()
        pulang "Hon kenderaan: Pon! Pon!"
    selesai
selesai

ada kenderaan_asas = Kenderaan("Proton", 180)
tunjuk "Objek baru dibina:", kenderaan_asas
tunjuk "Jenis objek:", jenis(kenderaan_asas)
tunjuk kenderaan_asas.tunjukInfo()
kenderaan_asas.pandu(60)
tunjuk "Selepas dipandu 60 km/j:", kenderaan_asas.tunjukInfo()
tunjuk kenderaan_asas.hon()

tunjuk ""
tunjuk "=== 2. PEWARISAN KELAS & INDUK (Kereta Elektrik) ==="

kelas KeretaElektrik warisi Kenderaan
    bina(jenama, kelajuan_maks, kapasiti_bateri)
        # Panggil pembina kelas induk dengan 'induk.bina'
        induk.bina(jenama, kelajuan_maks)
        diri.bateri = 100
        diri.kapasiti_bateri = kapasiti_bateri
    selesai

    # Menindih (override) kaedah induk
    kerja hon()
        pulang "Hon Kereta Elektrik: Tiit! Tiit! (Senyap & Canggih)"
    selesai

    # Menambah kaedah baharu khusus Kereta Elektrik
    kerja cas(peratusan)
        diri.bateri = diri.bateri + peratusan
        jika diri.bateri > 100
            diri.bateri = 100
        selesai
        pulang diri.bateri
    selesai

    kerja tunjukInfo()
        # Menggabungkan maklumat induk dan bateri
        ada info_induk = induk.tunjukInfo()
        pulang info_induk + " | Paras Bateri: " + teks(diri.bateri) + "%"
    selesai
selesai

ada ev = KeretaElektrik("Perodua EV", 160, 60)
tunjuk "Objek EV dibina:", ev
tunjuk "Jenis EV:", jenis(ev)
tunjuk ev.tunjukInfo()

tunjuk "Memandu EV laju..."
ev.pandu(120)
ev.bateri = 85
tunjuk ev.tunjukInfo()
tunjuk ev.hon()

tunjuk ""
tunjuk "=== 3. SIFAT DINAMIK & SERIALISASI JSON ==="
# Menambah sifat secara dinamik
ev.pemilik = "Runa"
ev["plat"] = "MY 2026 X"

tunjuk "Pemilik EV:", ev.pemilik
tunjuk "Nombor Plat:", ev["plat"]

# Menukar objek ke format JSON
ada json_ev = tulisJSON(ev, betul)
tunjuk "Hasil tulisJSON(ev, betul):"
tunjuk json_ev

tunjuk ""
tunjuk "=== DEMO OOP MELAYU SELESAI DENGAN JAYANYA ==="
