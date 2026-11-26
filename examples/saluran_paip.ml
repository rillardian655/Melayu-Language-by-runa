# ==============================================================================
# CONTOH MELAYU v0.4.0: OPERATOR SALURAN PAIP (PIPELINE OPERATOR)
# ==============================================================================
# Operator saluran paip `|>` (atau kata kunci `alir`) membolehkan aliran data
# diproses secara berurutan dari kiri ke kanan dengan kemas, ekspresif,
# dan bebas dari panggilan fungsi bersarang yang bertindan.
# ==============================================================================

kerja ganda(x)
    pulang x * 2
selesai

kerja tambah_lima(x)
    pulang x + 5
selesai

kerja tolak_angka(a, b)
    pulang a - b
selesai

kerja format_ringgit(jumlah)
    pulang "RM " + jumlah
selesai

cetak("--- UJIAN SALURAN PAIP (|>) ---")

# Cara lama (bersarang / nested):
# format_ringgit(tolak_angka(ganda(tambah_lima(10)), 5))
# Cara baharu yang sangat bersih dan mudah dibaca:
ada nilai_akhir = 10
|> tambah_lima
|> ganda
|> tolak_angka(5)
|> format_ringgit

cetak("Hasil pemprosesan paip: " + nilai_akhir)

# Penggunaan kata kunci alternatif: 'alir'
ada hasil_alir = 20 alir ganda alir tambah_lima
cetak("Hasil menggunakan 'alir': " + hasil_alir)
