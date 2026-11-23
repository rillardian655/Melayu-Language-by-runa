# ==============================================================================
# KELAS MATRIKS 2D (OPERASI MATRIKS 2x2) - MELAYU v0.3.0
# ==============================================================================

kelas Matriks2x2
    bina(a, b, c, d)
        diri.a = a
        diri.b = b
        diri.c = c
        diri.d = d
    selesai

    kerja campur(mLain)
        pulang Matriks2x2(diri.a + mLain.a, diri.b + mLain.b, diri.c + mLain.c, diri.d + mLain.d)
    selesai

    kerja darabSkala(k)
        pulang Matriks2x2(diri.a * k, diri.b * k, diri.c * k, diri.d * k)
    selesai

    kerja penentu()
        pulang (diri.a * diri.d) - (diri.b * diri.c)
    selesai

    kerja transposisi()
        pulang Matriks2x2(diri.a, diri.c, diri.b, diri.d)
    selesai

    kerja paparkan()
        pulang "[" + teks(diri.a) + ", " + teks(diri.b) + " | " + teks(diri.c) + ", " + teks(diri.d) + "]"
    selesai
selesai

tunjuk "=== OPERASI MATRIKS 2x2 ==="
ada m1 = Matriks2x2(1, 2, 3, 4)
ada m2 = Matriks2x2(5, 6, 7, 8)

tunjuk "Matriks 1:", m1.paparkan()
tunjuk "Matriks 2:", m2.paparkan()

ada mJumlah = m1.campur(m2)
tunjuk "m1 + m2:", mJumlah.paparkan()

tunjuk "Penentu m1 (det):", m1.penentu()

ada mTrans = m1.transposisi()
tunjuk "Transposisi m1:", mTrans.paparkan()
