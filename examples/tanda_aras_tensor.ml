# ==============================================================================
# TANDA ARAS KOMPUTASI TENSOR SELARI MULTI-TERAS & INFO PERKAKAS
# ==============================================================================

ada info = infoPerkakas()
tunjuk "=========================================================="
tunjuk "   INFO PERKAKAS & ENJIN KOMPUTASI AKALML"
tunjuk "=========================================================="
tunjuk "Teras CPU Dikesan  : " + teks(info["teras_cpu"]) + " teras"
tunjuk "Enjin Matriks      : " + info["enjin_matriks"]
tunjuk "GPU CUDA Dikesan   : " + teks(info["gpu_cuda_dikesan"])
tunjuk "GPU ROCm Dikesan   : " + teks(info["gpu_rocm_dikesan"])
tunjuk "Status GPU         : " + info["status_gpu"]
tunjuk ""

tunjuk "Menjalankan pendaraban Tensor 128x128 @ 128x128 secara selari..."
ada mula = capMasa()
ada A = rawakTensor([128, 128])
ada B = rawakTensor([128, 128])
ada C = A @ B
ada tamat = capMasa()

tunjuk "Bentuk hasil       : " + teks(C.bentuk())
tunjuk "Purata elemen      : " + teks(C.purata())
tunjuk "Masa pelaksanaan   : " + teks(tamat - mula) + " ms"
