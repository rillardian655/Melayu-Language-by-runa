# ==============================================================================
# KRIPTOGRAFI KLASIK: PENYULITAN CAESAR (CAESAR CIPHER) - MELAYU v0.3.0
# ==============================================================================

ada huruf = "abcdefghijklmnopqrstuvwxyz"

kerja indeksAksara(ch)
    ada aksaraKecil = ch.hurufKecil()
    untuk i dari 0 hingga huruf.panjang() tolak 1
        jika huruf[i] sama dengan aksaraKecil
            pulang i
        selesai
    selesai
    pulang -1
selesai

kerja sulitCaesar(mesej, anjakan)
    ada hasil = ""
    untuk k dari 0 hingga mesej.panjang() tolak 1
        ada ch = mesej[k]
        ada pos = indeksAksara(ch)
        jika pos sama dengan -1
            hasil = hasil + ch
        jika tidak
            ada posBaru = (pos tambah anjakan) modulo 26
            hasil = hasil + huruf[posBaru]
        selesai
    selesai
    pulang hasil
selesai

kerja nyahSulitCaesar(teksSulit, anjakan)
    ada anjakanBalik = (26 tolak (anjakan modulo 26)) modulo 26
    pulang sulitCaesar(teksSulit, anjakanBalik)
selesai

tunjuk "=== DEMO PENYULITAN CAESAR ==="
ada teksAsal = "bahasa melayu serumpun"
ada anjakan = 5

ada teksRahsia = sulitCaesar(teksAsal, anjakan)
tunjuk "Teks Asal:", teksAsal
tunjuk "Disulitkan (+5):", teksRahsia

ada teksDinyah = nyahSulitCaesar(teksRahsia, anjakan)
tunjuk "Dinyahsulitkan:", teksDinyah
