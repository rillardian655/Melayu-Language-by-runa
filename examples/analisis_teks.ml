# ==============================================================================
# ANALISIS KEKERAPAN TEKS & KOSA KATA - MELAYU v0.3.0
# ==============================================================================

kerja analisisKekerapanPerkataan(perenggan)
    ada perkataan = perenggan.hurufKecil().pecah(" ")
    ada kiraan = kamus {}

    untuk p dalam perkataan
        jika kiraan.adaKunci(p)
            kiraan[p] = kiraan[p] + 1
        jika tidak
            kiraan[p] = 1
        selesai
    selesai

    pulang kiraan
selesai

tunjuk "=== ANALISIS KEKERAPAN TEKS ==="
ada teksContoh = "bahasa melayu bahasa serumpun bahasa nusantara membina teknologi melayu"
ada hasilKiraan = analisisKekerapanPerkataan(teksContoh)

tunjuk "Teks Analisis:", teksContoh
tunjuk "Hasil Kekerapan JSON:"
tunjuk tulisJSON(hasilKiraan, betul)
