# matematik_modul.ml
# Modul utiliti matematik untuk contoh sistem import 'ambil'

tetap PI = 3.14159265

kerja kiraDiskaun(harga, peratus)
    pulang harga tolak (harga kali (peratus bahagi 100))
selesai

kerja adakahGenap(n)
    pulang (n modulo 2) sama dengan 0
selesai

kerja purata(senaraiNombor)
    ada jumlah = 0
    untuk x dalam senaraiNombor
        jumlah = jumlah tambah x
    selesai
    pulang jumlah bahagi panjang(senaraiNombor)
selesai
