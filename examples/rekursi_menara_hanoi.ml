# ==============================================================================
# PENYELESAIAN TEKA-TEKI MENARA HANOI SECARA REKURSIF - MELAYU v0.3.0
# ==============================================================================

ada bilanganLangkah = 0

kerja selesaikanHanoi(cakera, tiangAsal, tiangTujuan, tiangBantuan)
    jika cakera sama dengan 1
        bilanganLangkah = bilanganLangkah tambah 1
        tunjuk "Langkah " + teks(bilanganLangkah) + ": Pindahkan cakera 1 dari Tiang " + tiangAsal + " -> Tiang " + tiangTujuan
        pulang kosong
    selesai

    # 1. Pindahkan n-1 cakera dari Asal ke Bantuan
    selesaikanHanoi(cakera tolak 1, tiangAsal, tiangBantuan, tiangTujuan)

    # 2. Pindahkan cakera ke-n dari Asal ke Tujuan
    bilanganLangkah = bilanganLangkah tambah 1
    tunjuk "Langkah " + teks(bilanganLangkah) + ": Pindahkan cakera " + teks(cakera) + " dari Tiang " + tiangAsal + " -> Tiang " + tiangTujuan

    # 3. Pindahkan n-1 cakera dari Bantuan ke Tujuan
    selesaikanHanoi(cakera tolak 1, tiangBantuan, tiangTujuan, tiangAsal)
selesai

tunjuk "=== SIMULASI MENARA HANOI (3 CAKERA) ==="
tunjuk "Tiang A: Asal | Tiang B: Bantuan | Tiang C: Tujuan"
tunjuk ""

selesaikanHanoi(3, "A", "C", "B")

tunjuk ""
tunjuk "Selesai dalam jumlah langkah:", bilanganLangkah
tunjuk "Formula minimum 2^n - 1 untuk n=3:", (2 * 2 * 2) tolak 1
