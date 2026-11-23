# ==============================================================================
# ALGORITMA CARIAN BINARI (BINARY SEARCH) - MELAYU v0.3.0
# Kerumitan Masa: O(log n)
# ==============================================================================

# 1. Carian Binari Lelaran (Iterative)
kerja carianBinari(senarai, sasaran)
    ada kiri = 0
    ada kanan = senarai.panjang() tolak 1

    selagi kiri <= kanan
        ada tengah = lantai((kiri tambah kanan) / 2)
        ada nilaiTengah = senarai[tengah]

        jika nilaiTengah sama dengan sasaran
            pulang tengah
        selesai

        jika nilaiTengah < sasaran
            kiri = tengah tambah 1
        jika tidak
            kanan = tengah tolak 1
        selesai
    selesai

    pulang -1
selesai

tunjuk "=== UJIAN CARIAN BINARI ==="
ada dataTersusun = [3, 8, 15, 24, 38, 45, 59, 72, 81, 99]
tunjuk "Data Tersusun:", dataTersusun.cantum(", ")

ada indeksCari = carianBinari(dataTersusun, 45)
tunjuk "Cari 45 -> Jumpa di indeks:", indeksCari

ada indeksTiada = carianBinari(dataTersusun, 50)
tunjuk "Cari 50 -> Hasil:", indeksTiada, "(Tidak dijumpai)"
