# ==============================================================================
# ALGORITMA ISIHAN KLASIK (BUBBLE SORT & QUICKSORT) - MELAYU v0.3.0
# ==============================================================================

# 1. Isihan Buih (Bubble Sort)
kerja isihBuih(senarai)
    ada n = senarai.panjang()
    ada salinan = []
    untuk item dalam senarai
        salinan.tambah(item)
    selesai

    untuk i dari 0 hingga n tolak 1
        untuk j dari 0 hingga n tolak i tolak 2
            jika salinan[j] > salinan[j tambah 1]
                # Tukar kedudukan
                ada sementara = salinan[j]
                salinan[j] = salinan[j tambah 1]
                salinan[j tambah 1] = sementara
            selesai
        selesai
    selesai
    pulang salinan
selesai

# 2. Isihan Pantas Rekursif (QuickSort)
kerja isihPantas(senarai)
    jika senarai.panjang() kurang sama 1
        pulang senarai
    selesai

    ada paksi = senarai[0]
    ada lebihKecil = []
    ada sama = []
    ada lebihBesar = []

    untuk x dalam senarai
        jika x < paksi
            lebihKecil.tambah(x)
        jika tidak jika x sama dengan paksi
            sama.tambah(x)
        jika tidak
            lebihBesar.tambah(x)
        selesai
    selesai

    ada kiriTersusun = isihPantas(lebihKecil)
    ada kananTersusun = isihPantas(lebihBesar)

    ada hasil = []
    untuk a dalam kiriTersusun
        hasil.tambah(a)
    selesai
    untuk b dalam sama
        hasil.tambah(b)
    selesai
    untuk c dalam kananTersusun
        hasil.tambah(c)
    selesai

    pulang hasil
selesai

tunjuk "=== UJIAN ALGORITMA ISIHAN ==="
ada dataRawak = [64, 34, 25, 12, 22, 11, 90, 5, 88]
tunjuk "Data Asal:", dataRawak.cantum(", ")

ada hasilBuih = isihBuih(dataRawak)
tunjuk "Hasil Isihan Buih:", hasilBuih.cantum(", ")

ada hasilPantas = isihPantas(dataRawak)
tunjuk "Hasil QuickSort:", hasilPantas.cantum(", ")
