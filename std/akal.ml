# ==============================================================================
# KERANGKA KERJA AKALML (MACHINE LEARNING BAWAAN BAHASA MELAYU)
# Versi: 1.0.0
# ==============================================================================

# --- KELAS MATRIKS ---
kelas Matriks
    bina(data)
        diri.data = data
        diri.baris = panjang(data)
        jika diri.baris > 0
            diri.lajur = panjang(data[0])
        jika tidak
            diri.lajur = 0
        selesai
    selesai

    kerja darab(mLain)
        ada hasil = darabMatriks(diri.data, mLain.data)
        pulang Matriks(hasil)
    selesai

    kerja transposisi()
        ada hasil = transposisiMatriks(diri.data)
        pulang Matriks(hasil)
    selesai

    kerja songsang()
        ada hasil = songsangMatriks(diri.data)
        pulang Matriks(hasil)
    selesai

    kerja nilai()
        pulang diri.data
    selesai

    kerja paparkan()
        tunjuk "Matriks (" + teks(diri.baris) + " x " + teks(diri.lajur) + "):"
        untuk baris dalam diri.data
            ada barisTeks = "["
            ada pertama = betul
            untuk el dalam baris
                jika bukan pertama
                    barisTeks = barisTeks + ", "
                selesai
                barisTeks = barisTeks + teks(el)
                pertama = salah
            selesai
            barisTeks = barisTeks + "]"
            tunjuk barisTeks
        selesai
    selesai
selesai

# --- KELAS REGRESI LINEAR (Ordinary Least Squares) ---
kelas RegresiLinear
    bina(denganPintas)
        diri.denganPintas = denganPintas
        diri.pekali = kosong
        diri.pintas = 0
    selesai

    kerja latih(X, y)
        ada n_sampel = panjang(X)
        jika n_sampel == 0
            baling "Dataset latih kosong!"
        selesai

        ada X_mat = []
        jika diri.denganPintas
            untuk baris dalam X
                ada barisBaru = [1.0]
                untuk c dalam baris
                    barisBaru.tambah(c)
                selesai
                X_mat.tambah(barisBaru)
            selesai
        jika tidak
            X_mat = X
        selesai

        ada y_mat = []
        untuk val dalam y
            y_mat.tambah([val])
        selesai

        ada Xt = transposisiMatriks(X_mat)
        ada XtX = darabMatriks(Xt, X_mat)
        ada invXtX = songsangMatriks(XtX)
        ada Xty = darabMatriks(Xt, y_mat)
        ada beta = darabMatriks(invXtX, Xty)

        ada hasil_beta = []
        untuk b dalam beta
            hasil_beta.tambah(b[0])
        selesai

        jika diri.denganPintas
            diri.pintas = hasil_beta[0]
            ada pekali_tanpa_pintas = []
            ada idx = 1
            selagi idx < panjang(hasil_beta)
                pekali_tanpa_pintas.tambah(hasil_beta[idx])
                idx = idx + 1
            selesai
            diri.pekali = pekali_tanpa_pintas
        jika tidak
            diri.pintas = 0
            diri.pekali = hasil_beta
        selesai
    selesai

    kerja ramal(X_baru)
        jika diri.pekali == kosong
            baling "Model belum dilatih! Sila panggil .latih(X, y) dahulu."
        selesai

        ada ramalan = []
        untuk baris dalam X_baru
            ada y_topi = diri.pintas
            ada i = 0
            selagi i < panjang(diri.pekali)
                y_topi = y_topi + (baris[i] * diri.pekali[i])
                i = i + 1
            selesai
            ramalan.tambah(y_topi)
        selesai
        pulang ramalan
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        pulang skorR2(y_uji, y_ramal)
    selesai
selesai

# --- KELAS PENGELAS KNN (K-Nearest Neighbors) ---
kelas PengelasKNN
    bina(k)
        diri.k = k
        diri.X_latih = []
        diri.y_latih = []
    selesai

    kerja latih(X, y)
        diri.X_latih = X
        diri.y_latih = y
    selesai

    kerja ramal(X_baru)
        jika panjang(diri.X_latih) == 0
            baling "Model KNN belum dilatih! Sila panggil .latih(X, y) dahulu."
        selesai

        ada ramalan = []
        untuk titik dalam X_baru
            ada jarak_senarai = []
            ada i = 0
            selagi i < panjang(diri.X_latih)
                ada d = jarakEuclidean(titik, diri.X_latih[i])
                jarak_senarai.tambah([d, diri.y_latih[i]])
                i = i + 1
            selesai

            ada n = panjang(jarak_senarai)
            ada a = 0
            selagi a < n
                ada min_idx = a
                ada b = a + 1
                selagi b < n
                    jika jarak_senarai[b][0] < jarak_senarai[min_idx][0]
                        min_idx = b
                    selesai
                    b = b + 1
                selesai
                ada temp = jarak_senarai[a]
                jarak_senarai[a] = jarak_senarai[min_idx]
                jarak_senarai[min_idx] = temp
                a = a + 1
            selesai

            ada undian = {}
            ada c = 0
            ada had_k = diri.k
            jika had_k > n
                had_k = n
            selesai

            selagi c < had_k
                ada label = teks(jarak_senarai[c][1])
                jika undian.adaKunci(label)
                    undian[label] = undian[label] + 1
                jika tidak
                    undian[label] = 1
                selesai
                c = c + 1
            selesai

            ada label_terbaik = kosong
            ada undian_maks = -1
            untuk kunci dalam undian.kunci()
                jika undian[kunci] > undian_maks
                    undian_maks = undian[kunci]
                    label_terbaik = kunci
                selesai
            selesai

            ramalan.tambah(label_terbaik)
        selesai
        pulang ramalan
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        ada y_uji_teks = []
        untuk val dalam y_uji
            y_uji_teks.tambah(teks(val))
        selesai
        pulang skorAkurasi(y_uji_teks, y_ramal)
    selesai
selesai

# --- KELAS PENGELOMPOKAN K-MEANS (Unsupervised Clustering) ---
kelas PengelompokanKMeans
    bina(k, lelaranMaksimum)
        diri.k = k
        diri.lelaranMaksimum = lelaranMaksimum
        diri.pusat = []
    selesai

    kerja latih(X)
        ada n_sampel = panjang(X)
        jika n_sampel < diri.k
            baling "Bilangan sampel mestilah lebih besar atau sama dengan k!"
        selesai

        diri.pusat = []
        ada i = 0
        selagi i < diri.k
            diri.pusat.tambah(X[i])
            i = i + 1
        selesai

        ada n_dim = panjang(X[0])
        ada pusingan = 0

        selagi pusingan < diri.lelaranMaksimum
            ada kelompok = []
            ada idx = 0
            selagi idx < diri.k
                kelompok.tambah([])
                idx = idx + 1
            selesai

            untuk titik dalam X
                ada jarak_min = 1000000000.0
                ada pusat_terpilih = 0
                ada c = 0
                selagi c < diri.k
                    ada dist = jarakEuclidean(titik, diri.pusat[c])
                    jika dist < jarak_min
                        jarak_min = dist
                        pusat_terpilih = c
                    selesai
                    c = c + 1
                selesai
                kelompok[pusat_terpilih].tambah(titik)
            selesai

            ada pusat_baharu = []
            ada c = 0
            selagi c < diri.k
                ada anggota = kelompok[c]
                jika panjang(anggota) == 0
                    pusat_baharu.tambah(diri.pusat[c])
                jika tidak
                    ada pusat_c = []
                    ada d = 0
                    selagi d < n_dim
                        ada jumlah = 0.0
                        untuk pt dalam anggota
                            jumlah = jumlah + pt[d]
                        selesai
                        pusat_c.tambah(jumlah / panjang(anggota))
                        d = d + 1
                    selesai
                    pusat_baharu.tambah(pusat_c)
                selesai
                c = c + 1
            selesai

            diri.pusat = pusat_baharu
            pusingan = pusingan + 1
        selesai
    selesai

    kerja ramal(X_baru)
        ada ramalan = []
        untuk titik dalam X_baru
            ada jarak_min = 1000000000.0
            ada pusat_terpilih = 0
            ada c = 0
            selagi c < diri.k
                ada dist = jarakEuclidean(titik, diri.pusat[c])
                jika dist < jarak_min
                    jarak_min = dist
                    pusat_terpilih = c
                selesai
                c = c + 1
            selesai
            ramalan.tambah(pusat_terpilih)
        selesai
        pulang ramalan
    selesai
selesai
