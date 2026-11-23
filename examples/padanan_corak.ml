# ==============================================================================
# CONTOH MELAYU v0.4.0: PADANAN CORAK EKSPRESIF (PATTERN MATCHING)
# ==============================================================================
# Padanan corak `padan ... bila ... => ... lain => ... selesai` membolehkan
# perbandingan nilai, julat angka (hingga), pengawal syarat (jika), dan
# pengikatan pembolehubah tempatan secara sangat elegan.
# ==============================================================================

kerja nilai_gred(markah)
    pulang padan markah
        bila 90 hingga 100 => "A+ (Cemerlang Tertinggi)"
        bila 80 hingga 89  => "A  (Cemerlang)"
        bila 70 hingga 79  => "B  (Kepujian)"
        bila 50 hingga 69  => "C  (Lulus)"
        bila 0 hingga 49   => "F  (Gagal)"
        lain               => "Markah tidak sah!"
    selesai
selesai

kerja analisis_status_http(kod)
    pulang padan kod
        bila 200 => "200 OK: Permintaan berjaya."
        bila 201 => "201 Dicipta: Sumber berjaya dicipta."
        bila 400 => "400 Permintaan Buruk: Sintaks klien salah."
        bila 401 => "401 Tidak Sah: Perlu pengesahan identiti."
        bila 404 => "404 Tidak Dijumpai: URL tidak wujud."
        bila 500 => "500 Ralat Pelayan: Gangguan pada sistem dalaman."
        bila k jika kod >= 500 dan kod <= 599 => "Ralat Pelayan Kategori 5xx: Kod " + k
        lain => "Status tidak diketahui: " + kod
    selesai
selesai

cetak("--- ANALISIS GRED PEPERIKSAAN ---")
cetak("Markah 95 : " + nilai_gred(95))
cetak("Markah 83 : " + nilai_gred(83))
cetak("Markah 62 : " + nilai_gred(62))
cetak("Markah 40 : " + nilai_gred(40))
cetak("Markah -5 : " + nilai_gred(-5))

cetak("")
cetak("--- RESPON STATUS HTTP ---")
cetak(analisis_status_http(200))
cetak(analisis_status_http(404))
cetak(analisis_status_http(503))
