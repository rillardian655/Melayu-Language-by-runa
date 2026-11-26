# Panduan Rujukan AkalML (`ambil "akal"`)

**AkalML** ialah kerangka kerja Machine Learning dan Deep Learning terbina dalam Bahasa Melayu, dikuasakan oleh enjin `Tensor` selari multi-teras serta sokongan GPU dinamik (AMD ROCm / NVIDIA CUDA).

## 1. Primitif `Tensor` Native & Operator `@`

```melayu
ada A = tensor([[1.0, 2.0], [3.0, 4.0]])
ada W = rawakTensor([2, 3])
ada b = sifarTensor([3])

# Pendaraban matriks (@) + broadcasting bias + ReLU
ada Z = ((A @ W) + b).relu()
tunjuk Z.bentuk(), Z.purata()
```

Pilihan GPU & Had Langkah:
* `MELAYU_GPU=fp32` untuk pendaraban matriks FP32 pada GPU (auto-fallback ke CPU FP64 jika perlu).
* `--had-langkah 0` atau `MELAYU_HAD_LANGKAH=0` untuk latihan dataset besar.

## 2. Model Pembelajaran Mesin Sedia Guna

| Kelas | Kegunaan | Kaedah Utama |
| :--- | :--- | :--- |
| `RegresiLinear(denganPintas)` | Ramalan nilai berterusan (OLS) | `.latih(X, y)`, `.ramal(X)`, `.skor(X, y)`, `.simpan()`, `.muat()` |
| `PengelasKNN(k)` | Pengelasan jiran terdekat | `.latih(X, y)`, `.ramal(X)`, `.skor(X, y)` |
| `PengelompokanKMeans(k, lelaran)` | Pengklusteran tanpa seliaan | `.latih(X)`, `.ramal(X)` |
| `PengelasBayesNaif()` | Pengelas Gaussian Naive Bayes | `.latih(X, y)`, `.ramal(X)`, `.skor(X, y)`, `.simpan()`, `.muat()` |
| `PokokKeputusan(kedalamanMaks)` | Pokok keputusan Ketulenan Gini | `.latih(X, y)`, `.ramal(X)`, `.skor(X, y)`, `.simpan()`, `.muat()` |
| `HutanRawak(pokok, kedalaman)` | Ensemble Random Forest (Bagging) | `.latih(X, y)`, `.ramal(X)`, `.skor(X, y)`, `.simpan()`, `.muat()` |
| `JaringanSaraf(lapisan, lr)` | Deep MLP (Sigmoid + MSE) | `.latih(X, Y, zaman, progres)`, `.ramal(X)`, `.simpan()`, `.muat()` |
| `RangkaianAdam(lapisan, lr)` | Deep MLP Moden (ReLU + Softmax/BCE + Adam) | `.latih(X, Y, zaman, progres)`, `.ramal(X)`, `.simpan()`, `.muat()` |

## 3. Pengolahan Data, NLP & Penilaian

* **JadualData (DataFrame)**: `bacaJadual("data.csv")`, `.pilihLajur()`, `.tapis()`, `.isiKosong()`, `.ringkasan()`, `.keMatriks()`.
* **Penyekala & Pengkod**: `SkalaMinMaks`, `PenyelarasanPiawai`, `PengkodLabel`, `PengkodSatuPanas`.
* **NLP**: `PengiraTFIDF` & `kesamaanCosine(v1, v2)`.
* **Dataset Terbina & Validasi**: `muatIris()` (150 rekod), `muatKredit()`, `janaDataSintetik()`, `benihRawak(n)`, `bahagiLatihUji()`, dan `pengesahanSilang()`.
