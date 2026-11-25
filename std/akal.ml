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

    kerja tambah(mLain)
        ada hasil = matriksTambah(diri.data, mLain.data)
        pulang Matriks(hasil)
    selesai

    kerja tolak(mLain)
        ada hasil = matriksTolak(diri.data, mLain.data)
        pulang Matriks(hasil)
    selesai

    kerja skala(faktor)
        ada hasil = matriksSkala(diri.data, faktor)
        pulang Matriks(hasil)
    selesai

    kerja hadamard(mLain)
        ada hasil = darabHadamard(diri.data, mLain.data)
        pulang Matriks(hasil)
    selesai

    kerja sigmoid()
        ada hasil = matriksSigmoid(diri.data)
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

    kerja simpan(laluan)
        ada data = {
            "tipe": "RegresiLinear",
            "denganPintas": diri.denganPintas,
            "pintas": diri.pintas,
            "pekali": diri.pekali
        }
        ada json_str = tulisJSON(data, betul)
        tulisFail(laluan, json_str)
    selesai

    kerja muat(laluan)
        ada kandungan = bacaFail(laluan)
        ada data = bacaJSON(kandungan)
        diri.denganPintas = data["denganPintas"]
        diri.pintas = data["pintas"]
        diri.pekali = data["pekali"]
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

# --- KELAS JARINGAN SARAF TIRUAN (Multi-Layer Perceptron / Deep Neural Network) ---
kelas JaringanSaraf
    bina(lapisan, kadarBelajar)
        diri.lapisan = lapisan
        diri.kadarBelajar = kadarBelajar
        diri.wajaran = []
        diri.pincang = []

        ada l = 0
        ada n_lapisan = panjang(lapisan)
        selagi l < n_lapisan - 1
            ada n_masukan = lapisan[l]
            ada n_keluaran = lapisan[l + 1]

            ada w = []
            ada i = 0
            selagi i < n_masukan
                ada baris_w = []
                ada j = 0
                selagi j < n_keluaran
                    ada r = (rawak() - 0.5) * 1.5
                    baris_w.tambah(r)
                    j = j + 1
                selesai
                w.tambah(baris_w)
                i = i + 1
            selesai
            diri.wajaran.tambah(w)

            ada b = [[]]
            ada j2 = 0
            selagi j2 < n_keluaran
                b[0].tambah(0.0)
                j2 = j2 + 1
            selesai
            diri.pincang.tambah(b)

            l = l + 1
        selesai
    selesai

    kerja ramalKebarangkalian(X)
        ada a = X
        ada l = 0
        ada n_tahap = panjang(diri.wajaran)
        selagi l < n_tahap
            ada z = darabMatriks(a, diri.wajaran[l])
            z = matriksTambah(z, diri.pincang[l])
            a = matriksSigmoid(z)
            l = l + 1
        selesai
        pulang a
    selesai

    kerja latih(X, Y, zaman, tunjukProgres)
        ada Y_mat = Y
        jika panjang(Y) > 0 dan bukan (jenis(Y[0]) == "senarai")
            Y_mat = []
            untuk y_item dalam Y
                Y_mat.tambah([y_item])
            selesai
        selesai

        ada n_tahap = panjang(diri.wajaran)
        ada p = 1
        selagi p <= zaman
            # 1. Suapan ke hadapan (Forward pass)
            ada pengaktifan = [X]
            ada a_kini = X
            ada l = 0
            selagi l < n_tahap
                ada z = darabMatriks(a_kini, diri.wajaran[l])
                z = matriksTambah(z, diri.pincang[l])
                a_kini = matriksSigmoid(z)
                pengaktifan.tambah(a_kini)
                l = l + 1
            selesai

            # 2. Perambatan ke belakang (Backpropagation)
            ada a_keluar = pengaktifan[n_tahap]
            ada ralat = matriksTolak(a_keluar, Y_mat)
            ada d_keluar = matriksDerivatifSigmoid(a_keluar)
            ada delta = darabHadamard(ralat, d_keluar)

            ada senarai_delta = [delta]
            ada l_rev = n_tahap - 1
            selagi l_rev > 0
                ada w_trans = transposisiMatriks(diri.wajaran[l_rev])
                ada ralat_tersembunyi = darabMatriks(senarai_delta[0], w_trans)
                ada d_act = matriksDerivatifSigmoid(pengaktifan[l_rev])
                ada delta_prev = darabHadamard(ralat_tersembunyi, d_act)

                ada delta_baru = [delta_prev]
                untuk d_item dalam senarai_delta
                    delta_baru.tambah(d_item)
                selesai
                senarai_delta = delta_baru

                l_rev = l_rev - 1
            selesai

            # 3. Kemas kini wajaran dan pincang (Gradient descent)
            ada u = 0
            selagi u < n_tahap
                ada a_prev_t = transposisiMatriks(pengaktifan[u])
                ada grad_w = darabMatriks(a_prev_t, senarai_delta[u])
                ada grad_b = matriksJumlahLajur(senarai_delta[u])

                ada delta_w = matriksSkala(grad_w, diri.kadarBelajar)
                ada delta_b = matriksSkala(grad_b, diri.kadarBelajar)

                diri.wajaran[u] = matriksTolak(diri.wajaran[u], delta_w)
                diri.pincang[u] = matriksTolak(diri.pincang[u], delta_b)
                u = u + 1
            selesai

            # 4. Paparan perkembangan latihan
            jika tunjukProgres
                ada saiz_selang = zaman / 5
                jika saiz_selang < 1
                    saiz_selang = 1
                selesai
                jika p == 1 atau p % saiz_selang == 0 atau p == zaman
                    ada jumlah_ralat = 0.0
                    ada n_baris = panjang(ralat)
                    ada n_lajur = panjang(ralat[0])
                    ada ri = 0
                    selagi ri < n_baris
                        ada rj = 0
                        selagi rj < n_lajur
                            ada err_val = ralat[ri][rj]
                            jumlah_ralat = jumlah_ralat + (err_val * err_val)
                            rj = rj + 1
                        selesai
                        ri = ri + 1
                    selesai
                    ada mse = jumlah_ralat / (n_baris * n_lajur)
                    tunjuk "Zaman " + teks(p) + "/" + teks(zaman) + " -> Ralat MSE: " + teks(mse)
                selesai
            selesai

            p = p + 1
        selesai
    selesai

    kerja ramal(X_baru)
        ada probs = diri.ramalKebarangkalian(X_baru)
        ada n_keluaran = diri.lapisan[panjang(diri.lapisan) - 1]
        ada hasil = []
        untuk baris dalam probs
            jika n_keluaran == 1
                jika baris[0] >= 0.5
                    hasil.tambah(1)
                jika tidak
                    hasil.tambah(0)
                selesai
            jika tidak
                ada idx_maks = 0
                ada val_maks = baris[0]
                ada c = 1
                selagi c < panjang(baris)
                    jika baris[c] > val_maks
                        val_maks = baris[c]
                        idx_maks = c
                    selesai
                    c = c + 1
                selesai
                hasil.tambah(idx_maks)
            selesai
        selesai
        pulang hasil
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        ada y_ramal_num = []
        untuk val dalam y_ramal
            y_ramal_num.tambah(nombor(val))
        selesai
        ada y_uji_num = []
        untuk val dalam y_uji
            y_uji_num.tambah(nombor(val))
        selesai
        pulang skorAkurasi(y_uji_num, y_ramal_num)
    selesai

    kerja simpan(laluan)
        ada data = {
            "tipe": "JaringanSaraf",
            "lapisan": diri.lapisan,
            "kadarBelajar": diri.kadarBelajar,
            "wajaran": diri.wajaran,
            "pincang": diri.pincang
        }
        ada json_str = tulisJSON(data, betul)
        tulisFail(laluan, json_str)
    selesai

    kerja muat(laluan)
        ada kandungan = bacaFail(laluan)
        ada data = bacaJSON(kandungan)
        diri.lapisan = data["lapisan"]
        diri.kadarBelajar = data["kadarBelajar"]
        diri.wajaran = data["wajaran"]
        diri.pincang = data["pincang"]
    selesai
selesai

# --- KELAS PENGELAS BAYES NAIF (Gaussian Naive Bayes) ---
kelas PengelasBayesNaif
    bina()
        diri.kelas_unik = []
        diri.priors = {}
        diri.min_fitur = {}
        diri.sisihan_fitur = {}
    selesai

    kerja latih(X, y)
        ada n_sampel = panjang(X)
        jika n_sampel == 0
            baling "Dataset latihan kosong!"
        selesai
        ada n_fitur = panjang(X[0])

        diri.kelas_unik = []
        diri.priors = {}
        diri.min_fitur = {}
        diri.sisihan_fitur = {}

        # Kumpulkan sampel mengikut label kelas
        ada data_kelas = {}
        ada i = 0
        selagi i < n_sampel
            ada label = teks(y[i])
            jika bukan data_kelas.adaKunci(label)
                data_kelas[label] = []
                diri.kelas_unik.tambah(label)
            selesai
            data_kelas[label].tambah(X[i])
            i = i + 1
        selesai

        # Hitung kebarangkalian prior, purata (mean) dan sisihan piawai setiap ciri
        untuk label dalam diri.kelas_unik
            ada sampel_c = data_kelas[label]
            ada n_c = panjang(sampel_c)
            diri.priors[label] = n_c / n_sampel

            ada senarai_purata = []
            ada senarai_sisihan = []

            ada d = 0
            selagi d < n_fitur
                ada nilai_fitur = []
                untuk baris dalam sampel_c
                    nilai_fitur.tambah(baris[d])
                selesai

                ada m = purata(nilai_fitur)
                ada s = sisihanPiawai(nilai_fitur)
                jika s < 0.000001
                    s = 0.000001
                selesai

                senarai_purata.tambah(m)
                senarai_sisihan.tambah(s)
                d = d + 1
            selesai

            diri.min_fitur[label] = senarai_purata
            diri.sisihan_fitur[label] = senarai_sisihan
        selesai
    selesai

    kerja kebarangkalianGaussian(x, m, s)
        ada varians = s * s
        ada pemalar = 2.5066282746310002 * s
        ada beza = x - m
        ada eks = eksponen(0.0 - ((beza * beza) / (2.0 * varians)))
        pulang eks / pemalar
    selesai

    kerja ramal(X_baru)
        ada ramalan = []
        ada n_fitur = panjang(diri.min_fitur[diri.kelas_unik[0]])

        untuk baris dalam X_baru
            ada posterior_terbaik = -1000000000.0
            ada kelas_terbaik = kosong

            untuk label dalam diri.kelas_unik
                ada prior = diri.priors[label]
                ada log_posterior = logaritmaAsli(prior)

                ada d = 0
                selagi d < n_fitur
                    ada m = diri.min_fitur[label][d]
                    ada s = diri.sisihan_fitur[label][d]
                    ada prob = diri.kebarangkalianGaussian(baris[d], m, s)
                    jika prob < 0.000000000001
                        prob = 0.000000000001
                    selesai
                    log_posterior = log_posterior + logaritmaAsli(prob)
                    d = d + 1
                selesai

                jika log_posterior > posterior_terbaik
                    posterior_terbaik = log_posterior
                    kelas_terbaik = label
                selesai
            selesai

            ramalan.tambah(kelas_terbaik)
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

    kerja simpan(laluan)
        ada data = {
            "tipe": "PengelasBayesNaif",
            "kelas_unik": diri.kelas_unik,
            "priors": diri.priors,
            "min_fitur": diri.min_fitur,
            "sisihan_fitur": diri.sisihan_fitur
        }
        ada json_str = tulisJSON(data, betul)
        tulisFail(laluan, json_str)
    selesai

    kerja muat(laluan)
        ada kandungan = bacaFail(laluan)
        ada data = bacaJSON(kandungan)
        diri.kelas_unik = data["kelas_unik"]
        diri.priors = data["priors"]
        diri.min_fitur = data["min_fitur"]
        diri.sisihan_fitur = data["sisihan_fitur"]
    selesai
selesai

# --- KELAS POKOK KEPUTUSAN (Decision Tree Classifier) ---
kelas PokokKeputusan
    bina(kedalamanMaks)
        ada k = kedalamanMaks
        jika k == kosong
            k = 5
        selesai
        diri.kedalamanMaks = k
        diri.pokok = kosong
    selesai

    kerja kiraGini(y_label)
        ada n = panjang(y_label)
        jika n == 0
            pulang 0.0
        selesai
        ada kiraan = {}
        ada i = 0
        selagi i < n
            ada lbl = teks(y_label[i])
            jika kiraan.adaKunci(lbl)
                kiraan[lbl] = kiraan[lbl] + 1
            jika tidak
                kiraan[lbl] = 1
            selesai
            i = i + 1
        selesai

        ada jumlah_sq = 0.0
        ada kunci = kiraan.kunci()
        ada k_idx = 0
        selagi k_idx < panjang(kunci)
            ada c = kiraan[kunci[k_idx]]
            ada p = c / n
            jumlah_sq = jumlah_sq + (p * p)
            k_idx = k_idx + 1
        selesai
        pulang 1.0 - jumlah_sq
    selesai

    kerja dapatkanMajoriti(y_label)
        ada kiraan = {}
        ada i = 0
        selagi i < panjang(y_label)
            ada lbl = teks(y_label[i])
            jika kiraan.adaKunci(lbl)
                kiraan[lbl] = kiraan[lbl] + 1
            jika tidak
                kiraan[lbl] = 1
            selesai
            i = i + 1
        selesai

        ada max_c = -1
        ada maj = kosong
        ada kunci = kiraan.kunci()
        ada k_idx = 0
        selagi k_idx < panjang(kunci)
            ada k = kunci[k_idx]
            jika kiraan[k] > max_c
                max_c = kiraan[k]
                maj = k
            selesai
            k_idx = k_idx + 1
        selesai
        pulang nombor(maj)
    selesai

    kerja binaNod(X, y, kedalaman)
        ada n_sampel = panjang(X)
        jika n_sampel == 0
            pulang {"jenis": "daun", "kelas": 0}
        selesai

        ada maj = diri.dapatkanMajoriti(y)
        ada gini_asal = diri.kiraGini(y)

        # Hentikan jika sampel tulen atau capai had kedalaman
        jika gini_asal == 0.0 || kedalaman >= diri.kedalamanMaks || n_sampel <= 2
            pulang {"jenis": "daun", "kelas": maj}
        selesai

        ada n_ciri = panjang(X[0])
        ada skor_terbaik = 1000.0
        ada ciri_terbaik = kosong
        ada ambang_terbaik = kosong
        ada X_kiri_t = []
        ada y_kiri_t = []
        ada X_kanan_t = []
        ada y_kanan_t = []

        ada f = 0
        selagi f < n_ciri
            ada s_idx = 0
            selagi s_idx < n_sampel
                ada ambang = X[s_idx][f]
                ada X_kiri = []
                ada y_kiri = []
                ada X_kanan = []
                ada y_kanan = []

                ada i = 0
                selagi i < n_sampel
                    jika X[i][f] <= ambang
                        X_kiri.tambah(X[i])
                        y_kiri.tambah(y[i])
                    jika tidak
                        X_kanan.tambah(X[i])
                        y_kanan.tambah(y[i])
                    selesai
                    i = i + 1
                selesai

                jika panjang(X_kiri) > 0 && panjang(X_kanan) > 0
                    ada gini_kiri = diri.kiraGini(y_kiri)
                    ada gini_kanan = diri.kiraGini(y_kanan)
                    ada skor_pembahagian = (panjang(X_kiri) / n_sampel * gini_kiri) + (panjang(X_kanan) / n_sampel * gini_kanan)
                    jika skor_pembahagian < skor_terbaik
                        skor_terbaik = skor_pembahagian
                        ciri_terbaik = f
                        ambang_terbaik = ambang
                        X_kiri_t = X_kiri
                        y_kiri_t = y_kiri
                        X_kanan_t = X_kanan
                        y_kanan_t = y_kanan
                    selesai
                selesai
                s_idx = s_idx + 1
            selesai
            f = f + 1
        selesai

        jika ciri_terbaik == kosong
            pulang {"jenis": "daun", "kelas": maj}
        selesai

        ada cabang_kiri = diri.binaNod(X_kiri_t, y_kiri_t, kedalaman + 1)
        ada cabang_kanan = diri.binaNod(X_kanan_t, y_kanan_t, kedalaman + 1)

        pulang {
            "jenis": "nod",
            "ciri": ciri_terbaik,
            "ambang": ambang_terbaik,
            "kiri": cabang_kiri,
            "kanan": cabang_kanan
        }
    selesai

    kerja latih(X, y)
        jika panjang(X) == 0
            baling "Dataset latih PokokKeputusan kosong!"
        selesai
        diri.pokok = diri.binaNod(X, y, 0)
    selesai

    kerja ramalSatu(titik, nod)
        jika nod["jenis"] == "daun"
            pulang nod["kelas"]
        selesai
        ada f = nod["ciri"]
        ada ambang = nod["ambang"]
        jika titik[f] <= ambang
            pulang diri.ramalSatu(titik, nod["kiri"])
        jika tidak
            pulang diri.ramalSatu(titik, nod["kanan"])
        selesai
    selesai

    kerja ramal(X_baru)
        jika diri.pokok == kosong
            baling "PokokKeputusan belum dilatih!"
        selesai
        ada hasil = []
        ada i = 0
        selagi i < panjang(X_baru)
            hasil.tambah(diri.ramalSatu(X_baru[i], diri.pokok))
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        pulang skorAkurasi(y_uji, y_ramal)
    selesai

    kerja simpan(laluan)
        ada data = {
            "tipe": "PokokKeputusan",
            "kedalamanMaks": diri.kedalamanMaks,
            "pokok": diri.pokok
        }
        ada json_str = tulisJSON(data, betul)
        tulisFail(laluan, json_str)
    selesai

    kerja muat(laluan)
        ada kandungan = bacaFail(laluan)
        ada data = bacaJSON(kandungan)
        diri.kedalamanMaks = data["kedalamanMaks"]
        diri.pokok = data["pokok"]
    selesai
selesai

# --- KELAS SKALA MIN-MAKS (MinMaxScaler Preprocessing) ---
kelas SkalaMinMaks
    bina()
        diri.min_nilai = []
        diri.maks_nilai = []
    selesai

    kerja latih(X)
        ada n_baris = panjang(X)
        jika n_baris == 0
            pulang diri
        selesai
        ada n_lajur = panjang(X[0])
        diri.min_nilai = []
        diri.maks_nilai = []

        ada j = 0
        selagi j < n_lajur
            ada min_c = X[0][j]
            ada maks_c = X[0][j]
            ada i = 1
            selagi i < n_baris
                ada v = X[i][j]
                jika v < min_c
                    min_c = v
                selesai
                jika v > maks_c
                    maks_c = v
                selesai
                i = i + 1
            selesai
            diri.min_nilai.tambah(min_c)
            diri.maks_nilai.tambah(maks_c)
            j = j + 1
        selesai
    selesai

    kerja transformasi(X)
        ada n_baris = panjang(X)
        ada n_lajur = panjang(diri.min_nilai)
        ada hasil = []
        ada i = 0
        selagi i < n_baris
            ada baris_baru = []
            ada j = 0
            selagi j < n_lajur
                ada rentang = diri.maks_nilai[j] - diri.min_nilai[j]
                jika rentang == 0
                    baris_baru.tambah(0.0)
                jika tidak
                    baris_baru.tambah((X[i][j] - diri.min_nilai[j]) / rentang)
                selesai
                j = j + 1
            selesai
            hasil.tambah(baris_baru)
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja latihTransformasi(X)
        diri.latih(X)
        pulang diri.transformasi(X)
    selesai

    kerja songsang(X)
        ada n_baris = panjang(X)
        ada n_lajur = panjang(diri.min_nilai)
        ada hasil = []
        ada i = 0
        selagi i < n_baris
            ada baris_baru = []
            ada j = 0
            selagi j < n_lajur
                ada rentang = diri.maks_nilai[j] - diri.min_nilai[j]
                baris_baru.tambah((X[i][j] * rentang) + diri.min_nilai[j])
                j = j + 1
            selesai
            hasil.tambah(baris_baru)
            i = i + 1
        selesai
        pulang hasil
    selesai
selesai

# --- KELAS PENYELARASAN PIAWAI (StandardScaler / Z-Score) ---
kelas PenyelarasanPiawai
    bina()
        diri.purata_nilai = []
        diri.sisihan_nilai = []
    selesai

    kerja latih(X)
        ada n_baris = panjang(X)
        jika n_baris == 0
            pulang diri
        selesai
        ada n_lajur = panjang(X[0])
        diri.purata_nilai = []
        diri.sisihan_nilai = []

        ada j = 0
        selagi j < n_lajur
            ada lajur_vals = []
            ada i = 0
            selagi i < n_baris
                lajur_vals.tambah(X[i][j])
                i = i + 1
            selesai
            ada m = purata(lajur_vals)
            ada s = sisihanPiawai(lajur_vals)
            jika s < 0.000001
                s = 0.000001
            selesai
            diri.purata_nilai.tambah(m)
            diri.sisihan_nilai.tambah(s)
            j = j + 1
        selesai
    selesai

    kerja transformasi(X)
        ada n_baris = panjang(X)
        ada n_lajur = panjang(diri.purata_nilai)
        ada hasil = []
        ada i = 0
        selagi i < n_baris
            ada baris_baru = []
            ada j = 0
            selagi j < n_lajur
                baris_baru.tambah((X[i][j] - diri.purata_nilai[j]) / diri.sisihan_nilai[j])
                j = j + 1
            selesai
            hasil.tambah(baris_baru)
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja latihTransformasi(X)
        diri.latih(X)
        pulang diri.transformasi(X)
    selesai

    kerja songsang(X)
        ada n_baris = panjang(X)
        ada n_lajur = panjang(diri.purata_nilai)
        ada hasil = []
        ada i = 0
        selagi i < n_baris
            ada baris_baru = []
            ada j = 0
            selagi j < n_lajur
                baris_baru.tambah((X[i][j] * diri.sisihan_nilai[j]) + diri.purata_nilai[j])
                j = j + 1
            selesai
            hasil.tambah(baris_baru)
            i = i + 1
        selesai
        pulang hasil
    selesai
selesai

# ==============================================================================
# DATASET CONTOH TERBINA-DALAM (TOY DATASETS & DATA GENERATOR)
# ==============================================================================

fungsi muatIris()
    # Dataset Bunga Iris Fisher (Sampel Terpilih 30 Rekod: Setosa, Versicolor, Virginica)
    # Ciri: [Panjang Sepal, Lebar Sepal, Panjang Petal, Lebar Petal] (cm)
    ada X = [
        [5.1, 3.5, 1.4, 0.2], [4.9, 3.0, 1.4, 0.2], [4.7, 3.2, 1.3, 0.2], [4.6, 3.1, 1.5, 0.2],
        [5.0, 3.6, 1.4, 0.2], [5.4, 3.9, 1.7, 0.4], [4.6, 3.4, 1.4, 0.3], [5.0, 3.4, 1.5, 0.2],
        [4.4, 2.9, 1.4, 0.2], [4.9, 3.1, 1.5, 0.1],
        [7.0, 3.2, 4.7, 1.4], [6.4, 3.2, 4.5, 1.5], [6.9, 3.1, 4.9, 1.5], [5.5, 2.3, 4.0, 1.3],
        [6.5, 2.8, 4.6, 1.5], [5.7, 2.8, 4.5, 1.3], [6.3, 3.3, 4.7, 1.6], [4.9, 2.4, 3.3, 1.0],
        [6.6, 2.9, 4.6, 1.3], [5.2, 2.7, 3.9, 1.4],
        [6.3, 3.3, 6.0, 2.5], [5.8, 2.7, 5.1, 1.9], [7.1, 3.0, 5.9, 2.1], [6.3, 2.9, 5.6, 1.8],
        [6.5, 3.0, 5.8, 2.2], [7.6, 3.0, 6.6, 2.1], [4.9, 2.5, 4.5, 1.7], [7.3, 2.9, 6.3, 1.8],
        [6.7, 2.5, 5.8, 1.8], [7.2, 3.6, 6.1, 2.5]
    ]
    # Sasaran: 0 = Setosa, 1 = Versicolor, 2 = Virginica
    ada y = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2
    ]
    pulang {
        "nama": "Iris",
        "ciri": X,
        "sasaran": y,
        "nama_ciri": ["panjang_sepal", "lebar_sepal", "panjang_petal", "lebar_petal"],
        "nama_kelas": ["Setosa", "Versicolor", "Virginica"]
    }
selesai

fungsi muatKredit()
    # Dataset Penilaian Kelayakan Kredit Bank
    # Ciri: [Pendapatan (Juta), Skor Kredit (300-850), Nisbah Hutang (%), Simpanan (Juta)]
    ada X = [
        [15.0, 780.0, 15.0, 50.0],
        [4.5,  520.0, 65.0,  5.0],
        [22.0, 810.0, 10.0, 90.0],
        [5.0,  580.0, 45.0,  8.0],
        [12.5, 720.0, 25.0, 35.0],
        [3.8,  490.0, 70.0,  2.0],
        [18.0, 790.0, 18.0, 65.0],
        [6.2,  560.0, 55.0, 10.0],
        [14.0, 740.0, 22.0, 42.0],
        [4.0,  510.0, 60.0,  4.0],
        [25.0, 830.0, 12.0, 120.0],
        [5.5,  540.0, 50.0,  9.0],
        [11.0, 710.0, 28.0, 30.0],
        [3.5,  480.0, 75.0,  1.5],
        [16.5, 760.0, 19.0, 55.0],
        [4.8,  530.0, 58.0,  6.0]
    ]
    # Sasaran: 1 = Layak / Lulus, 0 = Ditolak
    ada y = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
    pulang {
        "nama": "KelayakanKredit",
        "ciri": X,
        "sasaran": y,
        "nama_ciri": ["pendapatan", "skor_kredit", "nisbah_hutang", "simpanan"],
        "nama_kelas": ["Ditolak", "Layak"]
    }
selesai

fungsi janaDataSintetik(bilangan_sampel, bilangan_ciri)
    ada X = []
    ada y = []
    ada i = 0
    selagi i < bilangan_sampel
        ada baris = []
        ada j = 0
        ada jumlah = 0.0
        selagi j < bilangan_ciri
            ada v = rawak(1, 100) / 10.0
            baris.tambah(v)
            jumlah = jumlah + v
            j = j + 1
        selesai
        X.tambah(baris)
        jika (jumlah / bilangan_ciri) >= 5.0
            y.tambah(1)
        jika tidak
            y.tambah(0)
        selesai
        i = i + 1
    selesai
    pulang {
        "ciri": X,
        "sasaran": y
    }
selesai

# ==============================================================================
# ENSEMBLE LEARNING: HUTAN RAWAK (Random Forest Classifier)
# ==============================================================================

kelas HutanRawak
    bina(bilanganPokok, kedalamanMaks)
        ada bp = bilanganPokok
        jika bp == kosong
            bp = 5
        selesai
        ada km = kedalamanMaks
        jika km == kosong
            km = 4
        selesai
        diri.bilanganPokok = bp
        diri.kedalamanMaks = km
        diri.hutan = []
    selesai

    kerja latih(X, y)
        ada n_sampel = panjang(X)
        jika n_sampel == 0
            baling "Dataset latih HutanRawak kosong!"
        selesai

        diri.hutan = []
        ada p = 0
        selagi p < diri.bilanganPokok
            # Pensampelan Bootstrap (Bagging)
            ada X_boot = []
            ada y_boot = []
            ada i = 0
            selagi i < n_sampel
                ada idx = rawak(0, n_sampel - 1)
                X_boot.tambah(X[idx])
                y_boot.tambah(y[idx])
                i = i + 1
            selesai

            ada pokok = PokokKeputusan(diri.kedalamanMaks)
            pokok.latih(X_boot, y_boot)
            diri.hutan.tambah(pokok)
            p = p + 1
        selesai
    selesai

    kerja ramal(X_baru)
        jika panjang(diri.hutan) == 0
            baling "Model HutanRawak belum dilatih! Sila panggil .latih(X, y) dahulu."
        selesai

        ada hasil = []
        ada i = 0
        selagi i < panjang(X_baru)
            ada titik = X_baru[i]
            ada undi = {}

            ada p = 0
            selagi p < panjang(diri.hutan)
                ada pokok = diri.hutan[p]
                ada ramal_pokok = pokok.ramalSatu(titik, pokok.pokok)
                ada kunci_str = teks(ramal_pokok)
                jika undi.adaKunci(kunci_str)
                    undi[kunci_str] = undi[kunci_str] + 1
                jika tidak
                    undi[kunci_str] = 1
                selesai
                p = p + 1
            selesai

            # Undian majoriti
            ada undi_maks = -1
            ada pemenang = kosong
            ada senarai_kunci = undi.kunci()
            ada k_idx = 0
            selagi k_idx < panjang(senarai_kunci)
                ada k = senarai_kunci[k_idx]
                jika undi[k] > undi_maks
                    undi_maks = undi[k]
                    pemenang = k
                selesai
                k_idx = k_idx + 1
            selesai

            hasil.tambah(nombor(pemenang))
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        pulang skorAkurasi(y_uji, y_ramal)
    selesai

    kerja simpan(laluan)
        ada pokok_senarai = []
        ada p = 0
        selagi p < panjang(diri.hutan)
            pokok_senarai.tambah(diri.hutan[p].pokok)
            p = p + 1
        selesai
        ada data = {
            "tipe": "HutanRawak",
            "bilanganPokok": diri.bilanganPokok,
            "kedalamanMaks": diri.kedalamanMaks,
            "pokok_senarai": pokok_senarai
        }
        ada json_str = tulisJSON(data, betul)
        tulisFail(laluan, json_str)
    selesai

    kerja muat(laluan)
        ada kandungan = bacaFail(laluan)
        ada data = bacaJSON(kandungan)
        diri.bilanganPokok = data["bilanganPokok"]
        diri.kedalamanMaks = data["kedalamanMaks"]
        diri.hutan = []
        ada senarai_nod = data["pokok_senarai"]
        ada p = 0
        selagi p < panjang(senarai_nod)
            ada pokok = PokokKeputusan(diri.kedalamanMaks)
            pokok.pokok = senarai_nod[p]
            diri.hutan.tambah(pokok)
            p = p + 1
        selesai
    selesai
selesai

# ==============================================================================
# PENGKODAN KATEGORI (LabelEncoder & OneHotEncoder)
# ==============================================================================

kelas PengkodLabel
    bina()
        diri.peta_ke_nombor = {}
        diri.peta_ke_teks = {}
        diri.kelas_unik = []
    selesai

    kerja latih(senarai_label)
        diri.peta_ke_nombor = {}
        diri.peta_ke_teks = {}
        diri.kelas_unik = []
        ada kaunter = 0
        ada i = 0
        selagi i < panjang(senarai_label)
            ada lbl = teks(senarai_label[i])
            jika bukan diri.peta_ke_nombor.adaKunci(lbl)
                diri.peta_ke_nombor[lbl] = kaunter
                diri.peta_ke_teks[teks(kaunter)] = lbl
                diri.kelas_unik.tambah(lbl)
                kaunter = kaunter + 1
            selesai
            i = i + 1
        selesai
    selesai

    kerja transformasi(senarai_label)
        ada hasil = []
        ada i = 0
        selagi i < panjang(senarai_label)
            ada lbl = teks(senarai_label[i])
            jika bukan diri.peta_ke_nombor.adaKunci(lbl)
                baling "Label tidak dikenali dalam PengkodLabel: " + lbl
            selesai
            hasil.tambah(diri.peta_ke_nombor[lbl])
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja latihTransformasi(senarai_label)
        diri.latih(senarai_label)
        pulang diri.transformasi(senarai_label)
    selesai

    kerja songsang(senarai_nombor)
        ada hasil = []
        ada i = 0
        selagi i < panjang(senarai_nombor)
            ada k = teks(senarai_nombor[i])
            jika bukan diri.peta_ke_teks.adaKunci(k)
                baling "Indeks kod tidak wujud dalam PengkodLabel: " + k
            selesai
            hasil.tambah(diri.peta_ke_teks[k])
            i = i + 1
        selesai
        pulang hasil
    selesai
selesai

kelas PengkodSatuPanas
    bina()
        diri.kelas_unik = []
        diri.peta_indeks = {}
    selesai

    kerja latih(senarai_label)
        diri.kelas_unik = []
        diri.peta_indeks = {}
        ada kaunter = 0
        ada i = 0
        selagi i < panjang(senarai_label)
            ada lbl = teks(senarai_label[i])
            jika bukan diri.peta_indeks.adaKunci(lbl)
                diri.peta_indeks[lbl] = kaunter
                diri.kelas_unik.tambah(lbl)
                kaunter = kaunter + 1
            selesai
            i = i + 1
        selesai
    selesai

    kerja transformasi(senarai_label)
        ada n_kelas = panjang(diri.kelas_unik)
        ada hasil = []
        ada i = 0
        selagi i < panjang(senarai_label)
            ada lbl = teks(senarai_label[i])
            jika bukan diri.peta_indeks.adaKunci(lbl)
                baling "Kategori tidak dikenali dalam PengkodSatuPanas: " + lbl
            selesai
            ada idx_aktif = diri.peta_indeks[lbl]
            ada vektor = []
            ada j = 0
            selagi j < n_kelas
                jika j == idx_aktif
                    vektor.tambah(1.0)
                jika tidak
                    vektor.tambah(0.0)
                selesai
                j = j + 1
            selesai
            hasil.tambah(vektor)
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja latihTransformasi(senarai_label)
        diri.latih(senarai_label)
        pulang diri.transformasi(senarai_label)
    selesai
selesai

# ==============================================================================
# PENGOLAHAN DATA TABULAR: JADUAL DATA (DataFrame ala Pandas)
# ==============================================================================

kelas JadualData
    bina(tajuk, baris)
        diri.tajuk = tajuk
        diri.baris = baris
    selesai

    kerja bilanganBaris()
        pulang panjang(diri.baris)
    selesai

    kerja bilanganLajur()
        pulang panjang(diri.tajuk)
    selesai

    kerja cariIndeksLajur(nama_lajur)
        ada i = 0
        selagi i < panjang(diri.tajuk)
            jika diri.tajuk[i] == nama_lajur
                pulang i
            selesai
            i = i + 1
        selesai
        baling "Lajur '" + nama_lajur + "' tidak wujud dalam JadualData!"
    selesai

    kerja dapatkanLajur(nama_lajur)
        ada idx = diri.cariIndeksLajur(nama_lajur)
        ada hasil = []
        ada i = 0
        selagi i < panjang(diri.baris)
            hasil.tambah(diri.baris[i][idx])
            i = i + 1
        selesai
        pulang hasil
    selesai

    kerja pilihLajur(senarai_nama)
        ada indeks_pilihan = []
        ada tajuk_baharu = []
        ada j = 0
        selagi j < panjang(senarai_nama)
            ada nama = senarai_nama[j]
            indeks_pilihan.tambah(diri.cariIndeksLajur(nama))
            tajuk_baharu.tambah(nama)
            j = j + 1
        selesai

        ada baris_baharu = []
        ada i = 0
        selagi i < panjang(diri.baris)
            ada b = []
            ada k = 0
            selagi k < panjang(indeks_pilihan)
                b.tambah(diri.baris[i][indeks_pilihan[k]])
                k = k + 1
            selesai
            baris_baharu.tambah(b)
            i = i + 1
        selesai

        pulang JadualData(tajuk_baharu, baris_baharu)
    selesai

    kerja tapis(nama_lajur, nilai_padan)
        ada idx = diri.cariIndeksLajur(nama_lajur)
        ada baris_baharu = []
        ada i = 0
        selagi i < panjang(diri.baris)
            jika diri.baris[i][idx] == nilai_padan
                baris_baharu.tambah(diri.baris[i])
            selesai
            i = i + 1
        selesai
        pulang JadualData(diri.tajuk, baris_baharu)
    selesai

    kerja isiKosong(nama_lajur, nilai_lalai)
        ada idx = diri.cariIndeksLajur(nama_lajur)
        ada i = 0
        selagi i < panjang(diri.baris)
            ada v = diri.baris[i][idx]
            jika v == kosong || v == ""
                diri.baris[i][idx] = nilai_lalai
            selesai
            i = i + 1
        selesai
        pulang diri
    selesai

    kerja keMatriks()
        ada matriks = []
        ada i = 0
        selagi i < panjang(diri.baris)
            ada baris_num = []
            ada j = 0
            selagi j < panjang(diri.baris[i])
                ada v = diri.baris[i][j]
                jika jenis(v) == "nombor"
                    baris_num.tambah(v)
                jika tidak
                    baris_num.tambah(nombor(v))
                selesai
                j = j + 1
            selesai
            matriks.tambah(baris_num)
            i = i + 1
        selesai
        pulang matriks
    selesai

    kerja ringkasan()
        ada statistik = {}
        ada j = 0
        selagi j < panjang(diri.tajuk)
            ada nama = diri.tajuk[j]
            ada nilai_num = []
            ada i = 0
            selagi i < panjang(diri.baris)
                ada v = diri.baris[i][j]
                jika jenis(v) == "nombor"
                    nilai_num.tambah(v)
                selesai
                i = i + 1
            selesai

            jika panjang(nilai_num) > 0
                ada min_v = nilai_num[0]
                ada maks_v = nilai_num[0]
                ada jumlah = 0.0
                ada k = 0
                selagi k < panjang(nilai_num)
                    ada val = nilai_num[k]
                    jika val < min_v
                        min_v = val
                    selesai
                    jika val > maks_v
                        maks_v = val
                    selesai
                    jumlah = jumlah + val
                    k = k + 1
                selesai
                statistik[nama] = {
                    "bilangan": panjang(nilai_num),
                    "min": min_v,
                    "maks": maks_v,
                    "purata": jumlah / panjang(nilai_num)
                }
            selesai
            j = j + 1
        selesai
        pulang {
            "bilangan_baris": panjang(diri.baris),
            "bilangan_lajur": panjang(diri.tajuk),
            "lajur": statistik
        }
    selesai
selesai

fungsi bacaJadual(laluan)
    ada csv = bacaCSV(laluan, betul)
    pulang JadualData(csv["lajur"], csv["data"])
selesai
