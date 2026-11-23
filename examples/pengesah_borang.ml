# ==============================================================================
# PENGESAH BORANG DATA (FORM VALIDATOR OOP) - MELAYU v0.3.0
# ==============================================================================

kelas PengesahBorang
    bina()
        diri.ralat = []
    selesai

    kerja semakWajib(nilai, namaMedan)
        jika nilai.panjang() sama dengan 0
            diri.ralat.tambah("Medan '" + namaMedan + "' wajib diisi.")
        selesai
    selesai

    kerja semakPanjangMin(nilai, min, namaMedan)
        jika nilai.panjang() < min
            diri.ralat.tambah("Medan '" + namaMedan + "' mestilah sekurang-kurangnya " + teks(min) + " aksara.")
        selesai
    selesai

    kerja semakEmel(nilai)
        jika bukan nilai.mengandungi("@") atau bukan nilai.mengandungi(".")
            diri.ralat.tambah("Format alamat emel tidak sah.")
        selesai
    selesai

    kerja adakahSah()
        pulang diri.ralat.panjang() sama dengan 0
    selesai
selesai

tunjuk "=== PENGESAHAN BORANG PENDAFTARAN ==="
ada borang = PengesahBorang()
borang.semakWajib("Ali", "Nama")
borang.semakPanjangMin("123", 6, "Kata Laluan")
borang.semakEmel("ali-bukan-emel")

tunjuk "Status borang sah?", borang.adakahSah()
tunjuk "Senarai Ralat Dijumpai:"
untuk r dalam borang.ralat
    tunjuk "- " + r
selesai
