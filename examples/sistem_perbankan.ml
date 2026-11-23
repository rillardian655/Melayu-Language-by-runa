# ==============================================================================
# SIMULASI SISTEM PERBANKAN BERORIENTASIKAN OBJEK (OOP) - MELAYU v0.3.0
# ==============================================================================

kelas AkaunBank
    bina(noAkaun, pemilik, bakiAwal)
        diri.noAkaun = noAkaun
        diri.pemilik = pemilik
        diri.baki = bakiAwal
        diri.sejarah = ["Akaun dibuka dengan baki RM" + teks(bakiAwal)]
    selesai

    kerja deposit(jumlah)
        jika jumlah <= 0
            pulang "Jumlah deposit mestilah positif."
        selesai
        diri.baki = diri.baki + jumlah
        diri.sejarah.tambah("Deposit: +RM" + teks(jumlah) + " (Baki: RM" + teks(diri.baki) + ")")
        pulang "Deposit berjaya."
    selesai

    kerja pengeluaran(jumlah)
        jika jumlah <= 0
            pulang "Jumlah pengeluaran tidak sah."
        selesai
        jika jumlah > diri.baki
            pulang "Ralat: Baki tidak mencukupi."
        selesai
        diri.baki = diri.baki - jumlah
        diri.sejarah.tambah("Pengeluaran: -RM" + teks(jumlah) + " (Baki: RM" + teks(diri.baki) + ")")
        pulang "Pengeluaran berjaya."
    selesai

    kerja dapatkanPenyata()
        pulang "Akaun [" + diri.noAkaun + "] " + diri.pemilik + " | Baki Semasa: RM" + teks(diri.baki)
    selesai
selesai

kelas AkaunSimpanan warisi AkaunBank
    bina(noAkaun, pemilik, bakiAwal, kadarFaedah)
        induk.bina(noAkaun, pemilik, bakiAwal)
        diri.kadarFaedah = kadarFaedah
    selesai

    kerja janaFaedah()
        ada untungFaedah = (diri.baki * diri.kadarFaedah) / 100
        diri.baki = diri.baki + untungFaedah
        diri.sejarah.tambah("Faedah bulanan: +RM" + teks(untungFaedah) + " (Baki: RM" + teks(diri.baki) + ")")
        pulang untungFaedah
    selesai
selesai

tunjuk "=== 1. AKAUN SIMPANAN ==="
ada akaunAli = AkaunSimpanan("SA-1001", "Ali Bin Ahmad", 1000, 3.5)
tunjuk akaunAli.dapatkanPenyata()

akaunAli.deposit(500)
akaunAli.pengeluaran(200)
ada faedah = akaunAli.janaFaedah()
tunjuk "Faedah dikreditkan: RM" + teks(faedah)
tunjuk akaunAli.dapatkanPenyata()

tunjuk ""
tunjuk "=== 2. SEJARAH TRANSAKSI ==="
untuk rekod dalam akaunAli.sejarah
    tunjuk "- " + rekod
selesai
