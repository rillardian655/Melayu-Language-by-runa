# ==============================================================================
# SUITE UJIAN UNIT TERBINA-DALAM MELAYU v0.4.0
# ==============================================================================
# Jalankan suite ini menggunakan arahan CLI rasmi:
#   melayu uji contoh/suite_ujian.ml
# ==============================================================================

kerja faktorial(n)
    semak n >= 0, "Faktorial memerlukan nombor positif atau sifar"
    jika n <= 1
        pulang 1
    selesai
    pulang n * faktorial(n - 1)
selesai

kerja adakah_palindrom(kata)
    ada p = kata.panjang()
    untuk i dari 0 hingga p / 2
        jika kata[i] != kata[p - 1 - i]
            pulang salah
        selesai
    selesai
    pulang betul
selesai

# -----------------------------------------------------------------------------
# BLOK-BLOK UJIAN UNIT
# -----------------------------------------------------------------------------

uji "Kiraan faktorial asas"
    semak faktorial(0) == 1, "0! mestilah 1"
    semak faktorial(1) == 1, "1! mestilah 1"
    semak faktorial(5) == 120, "5! mestilah 120"
selesai

uji "Pemeriksaan palindrom"
    semak adakah_palindrom("katak") == betul, "'katak' adalah palindrom"
    semak adakah_palindrom("malam") == betul, "'malam' adalah palindrom"
    semak adakah_palindrom("kereta") == salah, "'kereta' bukan palindrom"
selesai

uji "Pengendalian nilai selamat"
    ada data = kosong
    ada nilai = data ?? "Selamat"
    semak nilai == "Selamat", "Null coalescing gagal"
selesai
