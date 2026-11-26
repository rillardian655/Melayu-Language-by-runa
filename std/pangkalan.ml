# ==============================================================================
# PANGKALAN DATA TERBENAM BAHASA MELAYU (EMBEDDED DOCUMENT / TABLE DB)
# Versi: 1.0.0
# ==============================================================================
# Menyediakan storan data berstruktur (Jadual & Rekod) dengan kegigihan fail
# JSON yang patuh sepenuhnya kepada sandbox keselamatan Bahasa Melayu.
# ==============================================================================

kelas Pangkalan
    bina(laluan_fail)
        diri.laluan = laluan_fail
        diri.data = {"jadual": {}, "auto_id": {}}
        jika wujudFail(laluan_fail)
            ada mentah = bacaFail(laluan_fail)
            jika panjang(mentah.potong()) > 0
                ada dimuat = bacaJSON(mentah)
                jika jenis(dimuat) == "kamus" && dimuat.adaKunci("jadual")
                    diri.data = dimuat
                selesai
            selesai
        selesai
    selesai

    kerja simpan()
        ada json_str = tulisJSON(diri.data, betul)
        tulisFail(diri.laluan, json_str)
    selesai

    kerja ciptaJadual(nama_jadual)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            diri.data["jadual"][nama_jadual] = []
            diri.data["auto_id"][nama_jadual] = 1
            diri.simpan()
        selesai
        pulang diri
    selesai

    kerja senaraiJadual()
        pulang diri.data["jadual"].kunci()
    selesai

    kerja padamJadual(nama_jadual)
        jika diri.data["jadual"].adaKunci(nama_jadual)
            diri.data["jadual"].buang(nama_jadual)
            jika diri.data["auto_id"].adaKunci(nama_jadual)
                diri.data["auto_id"].buang(nama_jadual)
            selesai
            diri.simpan()
            pulang betul
        selesai
        pulang salah
    selesai

    kerja sisip(nama_jadual, rekod)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            diri.ciptaJadual(nama_jadual)
        selesai

        ada id_semasa = diri.data["auto_id"][nama_jadual]
        diri.data["auto_id"][nama_jadual] = id_semasa + 1

        ada rekod_baru = {}
        rekod_baru["_id"] = id_semasa
        ada kunci_senarai = rekod.kunci()
        ada i = 0
        selagi i < panjang(kunci_senarai)
            ada k = kunci_senarai[i]
            rekod_baru[k] = rekod[k]
            i = i + 1
        selesai

        diri.data["jadual"][nama_jadual].tambah(rekod_baru)
        diri.simpan()
        pulang rekod_baru
    selesai

    kerja padanSyarat(rekod, syarat)
        jika syarat == kosong
            pulang betul
        selesai
        ada kunci_syarat = syarat.kunci()
        ada i = 0
        selagi i < panjang(kunci_syarat)
            ada k = kunci_syarat[i]
            jika bukan rekod.adaKunci(k) || rekod[k] != syarat[k]
                pulang salah
            selesai
            i = i + 1
        selesai
        pulang betul
    selesai

    kerja cariSemua(nama_jadual)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            pulang []
        selesai
        pulang diri.data["jadual"][nama_jadual]
    selesai

    kerja cari(nama_jadual, syarat)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            pulang []
        selesai
        ada semua = diri.data["jadual"][nama_jadual]
        ada hasil = []
        ada i = 0
        selagi i < panjang(semua)
            jika diri.padanSyarat(semua[i], syarat)
                hasil.tambah(semua[i])
            selesai
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja cariSatu(nama_jadual, syarat)
        ada senarai = diri.cari(nama_jadual, syarat)
        jika panjang(senarai) > 0
            pulang senarai[0]
        selesai
        pulang kosong
    selesai

    kerja kemaskini(nama_jadual, syarat, data_baru)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            pulang 0
        selesai
        ada semua = diri.data["jadual"][nama_jadual]
        ada bilangan = 0
        ada kunci_baru = data_baru.kunci()
        ada i = 0
        selagi i < panjang(semua)
            jika diri.padanSyarat(semua[i], syarat)
                ada j = 0
                selagi j < panjang(kunci_baru)
                    ada k = kunci_baru[j]
                    jika bukan (k == "_id")
                        semua[i][k] = data_baru[k]
                    selesai
                    j = j + 1
                selesai
                bilangan = bilangan + 1
            selesai
            i = i + 1
        selesai
        jika bilangan > 0
            diri.simpan()
        selesai
        pulang bilangan
    selesai

    kerja padam(nama_jadual, syarat)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            pulang 0
        selesai
        ada semua = diri.data["jadual"][nama_jadual]
        ada baki = []
        ada dipadam = 0
        ada i = 0
        selagi i < panjang(semua)
            jika diri.padanSyarat(semua[i], syarat)
                dipadam = dipadam + 1
            jika tidak
                baki.tambah(semua[i])
            selesai
            i = i + 1
        selesai
        jika dipadam > 0
            diri.data["jadual"][nama_jadual] = baki
            diri.simpan()
        selesai
        pulang dipadam
    selesai

    kerja bilang(nama_jadual)
        jika bukan diri.data["jadual"].adaKunci(nama_jadual)
            pulang 0
        selesai
        pulang panjang(diri.data["jadual"][nama_jadual])
    selesai
selesai

