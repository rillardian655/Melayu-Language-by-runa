# ==============================================================================
# KERANGKA KERJA WEB SELASIH (BAHASA MELAYU .ML)
# Versi: 1.0.0
# ==============================================================================

fungsi balasJSON(data, status)
    ada st = status
    jika st == kosong
        st = 200
    selesai
    pulang {
        "status": st,
        "jenisKandungan": "application/json",
        "badan": tulisJSON(data)
    }
selesai

fungsi balasTeks(teksMsg, status)
    ada st = status
    jika st == kosong
        st = 200
    selesai
    pulang {
        "status": st,
        "jenisKandungan": "text/plain",
        "badan": teks(teksMsg)
    }
selesai

fungsi balasHTML(htmlMsg, status)
    ada st = status
    jika st == kosong
        st = 200
    selesai
    pulang {
        "status": st,
        "jenisKandungan": "text/html; charset=utf-8",
        "badan": teks(htmlMsg)
    }
selesai

kelas PelayanWeb
    bina(port)
        diri.port = port
        diri.laluan_get = {}
        diri.laluan_post = {}
        diri.laluan_put = {}
        diri.laluan_delete = {}
    selesai

    kerja dapatkan(laluan, handler)
        diri.laluan_get[laluan] = handler
    selesai

    kerja hantar(laluan, handler)
        diri.laluan_post[laluan] = handler
    selesai

    kerja kemaskini(laluan, handler)
        diri.laluan_put[laluan] = handler
    selesai

    kerja padam(laluan, handler)
        diri.laluan_delete[laluan] = handler
    selesai

    kerja kendaliPermintaan(req)
        ada kaedah = req["kaedah"]
        ada laluan = req["laluan"]
        ada handler = kosong

        jika kaedah == "GET" dan diri.laluan_get.adaKunci(laluan)
            handler = diri.laluan_get[laluan]
        jika tidak jika kaedah == "POST" dan diri.laluan_post.adaKunci(laluan)
            handler = diri.laluan_post[laluan]
        jika tidak jika kaedah == "PUT" dan diri.laluan_put.adaKunci(laluan)
            handler = diri.laluan_put[laluan]
        jika tidak jika kaedah == "DELETE" dan diri.laluan_delete.adaKunci(laluan)
            handler = diri.laluan_delete[laluan]
        selesai

        jika bukan (handler == kosong)
            pulang handler(req)
        jika tidak
            pulang balasJSON({
                "ralat": "Laluan '" + laluan + "' tidak dijumpai pada pelayan.",
                "status": 404
            }, 404)
        selesai
    selesai

    kerja dengar(hadPermintaan)
        ada had = hadPermintaan
        jika had == kosong
            had = 0
        selesai

        pulang mulakanPelayanHTTP(diri.port, diri, had)
    selesai
selesai
