# ==============================================================================
# POLA REKA BENTUK: KILANG (FACTORY PATTERN) - MELAYU v0.3.0
# ==============================================================================

kelas NotifikasiEmel
    kerja hantar(mesej)
        pulang "Menghantar EMEL: " + mesej
    selesai
selesai

kelas NotifikasiSMS
    kerja hantar(mesej)
        pulang "Menghantar SMS: " + mesej
    selesai
selesai

kelas NotifikasiWhatsApp
    kerja hantar(mesej)
        pulang "Menghantar WHATSAPP: " + mesej
    selesai
selesai

kelas KilangNotifikasi
    kerja binaSaluran(jenisSaluran)
        jika jenisSaluran.hurufKecil() sama dengan "emel"
            pulang NotifikasiEmel()
        jika tidak jika jenisSaluran.hurufKecil() sama dengan "sms"
            pulang NotifikasiSMS()
        jika tidak jika jenisSaluran.hurufKecil() sama dengan "whatsapp"
            pulang NotifikasiWhatsApp()
        selesai
        pulang kosong
    selesai
selesai

tunjuk "=== POLA REKA BENTUK KILANG (FACTORY) ==="
ada kilang = KilangNotifikasi()

ada n1 = kilang.binaSaluran("emel")
ada n2 = kilang.binaSaluran("whatsapp")

tunjuk n1.hantar("Kod pengesahan anda adalah 982134")
tunjuk n2.hantar("Pesanan anda telah berjaya dihantar!")
