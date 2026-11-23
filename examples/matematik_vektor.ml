# ==============================================================================
# KELAS VEKTOR 2D & FIZIK MATEMATIK - MELAYU v0.3.0
# ==============================================================================

kelas Vektor2D
    bina(x, y)
        diri.x = x
        diri.y = y
    selesai

    kerja campur(vLain)
        pulang Vektor2D(diri.x + vLain.x, diri.y + vLain.y)
    selesai

    kerja bezakan(vLain)
        pulang Vektor2D(diri.x - vLain.x, diri.y - vLain.y)
    selesai

    kerja skala(faktor)
        pulang Vektor2D(diri.x * faktor, diri.y * faktor)
    selesai

    kerja titikDarab(vLain)
        pulang (diri.x * vLain.x) + (diri.y * vLain.y)
    selesai

    kerja magnitud()
        ada hasilKuasa = (diri.x * diri.x) + (diri.y * diri.y)
        pulang puncaKuasaDua(hasilKuasa)
    selesai

    kerja teksInfo()
        pulang "(" + teks(diri.x) + ", " + teks(diri.y) + ")"
    selesai
selesai

tunjuk "=== 1. OPERASI VEKTOR 2D ==="
ada v1 = Vektor2D(3, 4)
ada v2 = Vektor2D(1, 2)

tunjuk "Vektor 1:", v1.teksInfo()
tunjuk "Magnitud Vektor 1 (hipotenus 3-4-5):", v1.magnitud()

ada vCampur = v1.campur(v2)
tunjuk "v1 campur v2:", vCampur.teksInfo()

ada vBeza = v1.bezakan(v2)
tunjuk "v1 bezakan v2:", vBeza.teksInfo()

ada vGandaTiga = v1.skala(3)
tunjuk "v1 skala 3:", vGandaTiga.teksInfo()

ada dot = v1.titikDarab(v2)
tunjuk "Dot Product (v1 . v2):", dot
