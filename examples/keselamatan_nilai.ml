# ==============================================================================
# CONTOH MELAYU v0.4.0: KESELAMATAN NILAI & NULL HANDLING
# ==============================================================================
# Mengelakkan kemalangan 'null pointer' dengan:
# 1. Null-Coalescing: `??` untuk menetapkan nilai lalai sekiranya kosong
# 2. Akses Rantai Selamat: `?.` untuk akses sifat & kaedah tanpa ralat
# ==============================================================================

kelas Profil
    bina(nama, emel)
        diri.nama = nama
        diri.emel = emel
    selesai

    kerja info()
        pulang diri.nama + " <" + diri.emel + ">"
    selesai
selesai

# 1. Null-coalescing asas:
ada input_pengguna = kosong
ada nama_tetamu = input_pengguna ?? "Tetamu Tanpa Nama"
cetak("Selamat datang, " + nama_tetamu + "!")

# 2. Rantai selamat dengan objek:
ada pengguna_sah = Profil("Siti Nurhaliza", "siti@melayu.dev")
ada pengguna_tiada = kosong

cetak("Nama Pengguna Sah   : " + (pengguna_sah?.nama ?? "Tidak diketahui"))
cetak("Nama Pengguna Tiada : " + (pengguna_tiada?.nama ?? "Anonim"))

# 3. Panggilan kaedah selamat:
cetak("Info Pengguna Sah   : " + (pengguna_sah?.info() ?? "Tiada data"))
cetak("Info Pengguna Tiada : " + (pengguna_tiada?.info() ?? "Tiada data"))
