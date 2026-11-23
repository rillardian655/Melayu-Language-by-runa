# ==============================================================================
# PENJANA KATA LALUAN RAWAK (PASSWORD GENERATOR) - MELAYU v0.3.0
# ==============================================================================

ada aksaraKecil = "abcdefghijklmnopqrstuvwxyz"
ada aksaraBesar = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
ada angka = "0123456789"
ada simbol = "!@#$%^&*"

ada semuaAksara = aksaraKecil + aksaraBesar + angka + simbol

kerja janaKataLaluan(panjangSasaran)
    jika panjangSasaran < 4
        pulang "Panjang minimum mestilah 4 aksara."
    selesai

    ada hasil = ""
    ada jumlahAksara = semuaAksara.panjang()

    untuk i dari 1 hingga panjangSasaran
        ada indeksRawak = rawak(0, jumlahAksara tolak 1)
        ada pilih = semuaAksara[indeksRawak]
        hasil = hasil + pilih
    selesai

    pulang hasil
selesai

tunjuk "=== PENJANA KATA LALUAN RAWAK SELAMAT ==="
tunjuk "Kata Laluan 1 (8 aksara):", janaKataLaluan(8)
tunjuk "Kata Laluan 2 (12 aksara):", janaKataLaluan(12)
tunjuk "Kata Laluan 3 (16 aksara):", janaKataLaluan(16)
