use melayu_core::interpreter::Interpreter;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;
use std::fs;

fn laksana_kod(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("{:?}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("{:?}", e))?;
    let mut interpreter = Interpreter::new();
    interpreter
        .interpret(&program)
        .map_err(|e| format!("{:?}", e))?;
    Ok("OK".to_string())
}

#[test]
fn test_akal_ml_operasi_matriks_native() {
    let code = r#"
ada A = [[1, 2], [3, 4]]
ada B = [[2, 0], [1, 2]]
ada C = darabMatriks(A, B)
# Hasil darab C = [[4, 4], [10, 8]]
semak C[0][0] == 4, "C[0][0] salah"
semak C[0][1] == 4, "C[0][1] salah"
semak C[1][0] == 10, "C[1][0] salah"
semak C[1][1] == 8, "C[1][1] salah"

ada At = transposisiMatriks(A)
semak At[0][1] == 3, "Transposisi At[0][1] salah"
semak At[1][0] == 2, "Transposisi At[1][0] salah"

ada M = [[4, 7], [2, 6]]
ada Minv = songsangMatriks(M)
ada I = darabMatriks(M, Minv)
# Nilai diagonal I mestilah hampir 1.0
semak bundar(I[0][0]) == 1, "Diagonal I[0][0] mestilah 1"
semak bundar(I[1][1]) == 1, "Diagonal I[1][1] mestilah 1"
semak bundar(I[0][1]) == 0, "Bukan diagonal I[0][1] mestilah 0"
"#;
    assert!(laksana_kod(code).is_ok());
}

#[test]
fn test_akal_ml_vektor_dan_metrik() {
    let code = r#"
ada v1 = [1, 2, 3]
ada v2 = [4, 5, 6]
ada dot = titikDarab(v1, v2)
semak dot == 32, "Titik darab mestilah 32"

ada p1 = [0, 0]
ada p2 = [3, 4]
ada dist = jarakEuclidean(p1, p2)
semak dist == 5, "Jarak Euclidean mestilah 5"

ada y_benar = [10, 20, 30]
ada y_ramal = [10, 20, 30]
semak skorMSE(y_benar, y_ramal) == 0, "MSE ramalan sempurna mestilah 0"
semak skorR2(y_benar, y_ramal) == 1, "R2 ramalan sempurna mestilah 1"

ada y_kat_benar = ["kucing", "anjing", "kucing"]
ada y_kat_ramal = ["kucing", "anjing", "anjing"]
# 2 daripada 3 betul = 0.6666...
semak bundar(skorAkurasi(y_kat_benar, y_kat_ramal) * 100) == 67, "Akurasi salah"

ada data = [2, 4, 4, 4, 5, 5, 7, 9]
semak purata(data) == 5, "Purata mestilah 5"
"#;
    assert!(laksana_kod(code).is_ok());
}

#[test]
fn test_akal_ml_regresi_linear_terbina_dalam() {
    let code = r#"
ambil "akal"

# Pola y = 2x + 3
ada X = [[1], [2], [3], [4]]
ada y = [5, 7, 9, 11]

ada model = RegresiLinear(betul)
model.latih(X, y)

# Ramal x = 5 dan x = 6 -> jangka y = 13 dan 15
ada X_baru = [[5], [6]]
ada ramalan = model.ramal(X_baru)

semak bundar(ramalan[0]) == 13, "Ramalan x=5 mestilah 13"
semak bundar(ramalan[1]) == 15, "Ramalan x=6 mestilah 15"
semak bundar(model.skor(X, y)) == 1, "Skor R2 mestilah 1.0"
"#;
    assert!(laksana_kod(code).is_ok());
}

#[test]
fn test_akal_ml_pengelas_knn_terbina_dalam() {
    let code = r#"
ambil "akal"

# Dua kelompok:
# Kelas A di sekitar [1, 1], Kelas B di sekitar [8, 8]
ada X = [
    [1.0, 1.0],
    [1.2, 0.8],
    [0.9, 1.1],
    [8.0, 8.0],
    [8.2, 7.9],
    [7.8, 8.1]
]
ada y = ["A", "A", "A", "B", "B", "B"]

ada knn = PengelasKNN(3)
knn.latih(X, y)

ada titik_ujian = [
    [1.1, 1.0], # hampir A
    [8.1, 8.0]  # hampir B
]

ada ramalan = knn.ramal(titik_ujian)
semak ramalan[0] == "A", "Titik pertama mestilah kelas A"
semak ramalan[1] == "B", "Titik kedua mestilah kelas B"
"#;
    assert!(laksana_kod(code).is_ok());
}

#[test]
fn test_akal_ml_csv_dan_bahagi_latih_uji() {
    let csv_path = "target/test_dataset_akal.csv";
    let csv_data = "keluasan,bilik,harga\n100,2,200\n150,3,300\n200,3,400\n250,4,500\n300,4,600\n";
    fs::write(csv_path, csv_data).unwrap();

    let code = r#"
ada dataset = bacaCSV("target/test_dataset_akal.csv", betul)
semak dataset["baris"] == 5, "Baris CSV mestilah 5"
semak dataset["bil_lajur"] == 3, "Lajur CSV mestilah 3"
semak dataset["lajur"][0] == "keluasan", "Nama lajur pertama salah"

# Bahagi data
ada X = []
ada y = []
untuk baris dalam dataset["data"]
    X.tambah([baris[0], baris[1]])
    y.tambah(baris[2])
selesai

ada bahagian = bahagiLatihUji(X, y, 0.4)
ada X_latih = bahagian[0]
ada X_uji = bahagian[1]
ada y_latih = bahagian[2]
ada y_uji = bahagian[3]
semak panjang(X_latih) == 3, "Panjang X_latih mestilah 3 (60%)"
semak panjang(X_uji) == 2, "Panjang X_uji mestilah 2 (40%)"
"#;
    let res = laksana_kod(code);
    let _ = fs::remove_file(csv_path);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_jaringan_saraf_xor() {
    let code = r#"
ambil "akal"

# Masalah Tak-Linear Klasik XOR
ada X = [
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0]
]
ada y = [
    [0.0],
    [1.0],
    [1.0],
    [0.0]
]

ada nn = JaringanSaraf([2, 4, 1], 3.2)
nn.latih(X, y, 2000, salah)

ada ramalan = nn.ramal(X)
semak ramalan[0] == 0, "XOR 0,0 mestilah 0"
semak ramalan[1] == 1, "XOR 0,1 mestilah 1"
semak ramalan[2] == 1, "XOR 1,0 mestilah 1"
semak ramalan[3] == 0, "XOR 1,1 mestilah 0"
"#;
    let res = laksana_kod(code);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_bayes_naif_dan_metrik() {
    let code = r#"
ambil "akal"

# Data diagnosis perubatan mudah
# [Suhu Badan, Kadar Nadi] -> "Sihat" atau "Demam"
ada X = [
    [36.5, 72.0],
    [36.6, 75.0],
    [36.8, 70.0],
    [38.5, 95.0],
    [39.0, 102.0],
    [38.8, 98.0]
]
ada y = ["Sihat", "Sihat", "Sihat", "Demam", "Demam", "Demam"]

ada gnb = PengelasBayesNaif()
gnb.latih(X, y)

ada pesakit_baru = [
    [36.7, 73.0], # Sihat
    [38.9, 100.0] # Demam
]
ada hasil = gnb.ramal(pesakit_baru)
semak hasil[0] == "Sihat", "Pesakit 1 mestilah Sihat"
semak hasil[1] == "Demam", "Pesakit 2 mestilah Demam"

# Uji Matriks Kekeliruan, Kejituan, Kepekaan, Skor F1
ada y_sebenar = ["Demam", "Demam", "Sihat", "Demam", "Sihat"]
ada y_ramal   = ["Demam", "Demam", "Demam", "Demam", "Sihat"] # 1 False Positive
ada cm = matriksKekeliruan(y_sebenar, y_ramal, "Demam")
semak cm["TP"] == 3.0, "TP mestilah 3"
semak cm["FP"] == 1.0, "FP mestilah 1"
semak cm["TN"] == 1.0, "TN mestilah 1"
semak cm["FN"] == 0.0, "FN mestilah 0"

ada p = kejituan(y_sebenar, y_ramal, "Demam")
ada r = kepekaan(y_sebenar, y_ramal, "Demam")
ada f1 = skorF1(y_sebenar, y_ramal, "Demam")

semak p == 0.75, "Kejituan (Precision) mestilah 0.75"
semak r == 1.0, "Kepekaan (Recall) mestilah 1.0"
semak bundar(f1 * 100) == 86, "Skor F1 mestilah ~0.857 (86%)"
"#;
    let res = laksana_kod(code);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_simpan_muat_dan_skala() {
    let code = r#"
ambil "akal"

# 1. Uji SkalaMinMaks
ada X = [
    [10.0, 100.0],
    [20.0, 200.0],
    [30.0, 300.0]
]
ada scaler = SkalaMinMaks()
ada X_skala = scaler.latihTransformasi(X)

semak X_skala[0][0] == 0.0, "Skala minimum mestilah 0.0"
semak X_skala[2][0] == 1.0, "Skala maksimum mestilah 1.0"
semak X_skala[1][0] == 0.5, "Skala tengah mestilah 0.5"

ada X_pulih = scaler.songsang(X_skala)
semak X_pulih[0][0] == 10.0, "Songsangan skala mestilah 10.0"
semak X_pulih[2][1] == 300.0, "Songsangan skala mestilah 300.0"

# 2. Uji Simpan dan Muat Model Regresi Linear
ada reg = RegresiLinear(betul)
reg.latih([[1.0], [2.0], [3.0]], [2.0, 4.0, 6.0])

reg.simpan("target/test_model_regresi.json")

ada reg_baru = RegresiLinear(betul)
reg_baru.muat("target/test_model_regresi.json")

ada ramal = reg_baru.ramal([[4.0]])
semak bundar(ramal[0]) == 8, "Model dimuatkan mesti ramal 8"
"#;
    let res = laksana_kod(code);
    let _ = fs::remove_file("target/test_model_regresi.json");
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_pokok_keputusan_dan_dataset_toy() {
    let code = r#"
ambil "akal"

# 1. Uji dataset bawaan muatIris
ada iris = muatIris()
semak iris["nama"] == "Iris", "Nama dataset mestilah Iris"
semak panjang(iris["ciri"]) == 30, "Bilangan sampel Iris mestilah 30"
semak panjang(iris["sasaran"]) == 30, "Bilangan sasaran Iris mestilah 30"

# 2. Uji dataset bawaan muatKredit
ada kredit = muatKredit()
semak kredit["nama"] == "KelayakanKredit", "Nama dataset mestilah KelayakanKredit"
semak panjang(kredit["ciri"]) == 16, "Bilangan sampel Kredit mestilah 16"

# 3. Uji janaDataSintetik
ada sintetik = janaDataSintetik(10, 3)
semak panjang(sintetik["ciri"]) == 10, "Sampel sintetik mestilah 10"
semak panjang(sintetik["ciri"][0]) == 3, "Ciri sintetik mestilah 3"

# 4. Uji Pengelas PokokKeputusan
ada dt = PokokKeputusan(4)
dt.latih(kredit["ciri"], kredit["sasaran"])

ada ramalan = dt.ramal([[20.0, 800.0, 15.0, 80.0]])
semak ramalan[0] == 1, "Profil kredit baik sepatutnya diramal Layak (1)"

dt.simpan("target/test_model_dt.json")
ada dt_muat = PokokKeputusan(4)
dt_muat.muat("target/test_model_dt.json")
ada ramalan_muat = dt_muat.ramal([[20.0, 800.0, 15.0, 80.0]])
semak ramalan_muat[0] == 1, "Model dimuatkan mesti ramal 1"
"#;
    let res = laksana_kod(code);
    let _ = fs::remove_file("target/test_model_dt.json");
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_hutan_rawak_jadual_dan_pengkod() {
    let code = r#"
ambil "akal"

# 1. Uji JadualData (DataFrame ala Pandas)
ada jadual = JadualData(
    ["umur", "gaji", "bandar", "status"],
    [
        [25.0, 5.0, "KL", "Tolak"],
        [40.0, 18.0, "Jakarta", "Lulus"],
        [22.0, kosong, "KL", "Tolak"],
        [38.0, 16.0, "Singapura", "Lulus"],
        [45.0, 22.0, "Jakarta", "Lulus"],
        [23.0, 4.5, "KL", "Tolak"]
    ]
)

semak jadual.bilanganBaris() == 6, "Bilangan baris mestilah 6"
semak jadual.bilanganLajur() == 4, "Bilangan lajur mestilah 4"

jadual.isiKosong("gaji", 4.0)
ada ringkas = jadual.ringkasan()
semak ringkas["lajur"]["gaji"]["min"] == 4.0, "Imputasi nilai kosong sepatutnya menjadikan min = 4.0"

ada X_jadual = jadual.pilihLajur(["umur", "gaji"]).keMatriks()
semak panjang(X_jadual) == 6, "Matriks mestilah mempunyai 6 baris"
semak X_jadual[2][1] == 4.0, "Nilai terimputasi mestilah 4.0"

# 2. Uji PengkodLabel & PengkodSatuPanas
ada pengkod = PengkodLabel()
ada y_kod = pengkod.latihTransformasi(jadual.dapatkanLajur("status"))
semak y_kod[0] == 0, "Tolak mestilah kod 0"
semak y_kod[1] == 1, "Lulus mestilah kod 1"
semak pengkod.songsang([1, 0])[0] == "Lulus", "Songsangan kod 1 mestilah Lulus"

ada ohe = PengkodSatuPanas()
ada bandar_ohe = ohe.latihTransformasi(jadual.dapatkanLajur("bandar"))
semak panjang(bandar_ohe[0]) == 3, "Terdapat 3 bandar unik jadi vektor One-Hot panjang 3"

# 3. Uji HutanRawak (Random Forest) + Simpan & Muat
ada hutan = HutanRawak(7, 3)
hutan.latih(X_jadual, y_kod)

ada ramal = hutan.ramal([[42.0, 20.0], [21.0, 4.0]])
semak ramal[0] == 1, "Profil gaji tinggi sepatutnya diramal Lulus (1)"
semak ramal[1] == 0, "Profil gaji rendah sepatutnya diramal Tolak (0)"

hutan.simpan("target/test_model_hutan.json")
ada hutan_muat = HutanRawak(7, 3)
hutan_muat.muat("target/test_model_hutan.json")
ada ramal_muat = hutan_muat.ramal([[42.0, 20.0]])
semak ramal_muat[0] == 1, "HutanRawak dimuatkan semula mesti mengekalkan ramalan tepat"
"#;
    let res = laksana_kod(code);
    let _ = fs::remove_file("target/test_model_hutan.json");
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_tensor_nlp_dan_pengesahan_silang() {
    let code = r#"
ambil "akal"

# 1. Uji Primitif Tensor Native & Operator @
ada A = tensor([[1.0, 2.0], [3.0, 4.0]])
ada B = tensor([[5.0, 6.0], [7.0, 8.0]])
ada C = A @ B
semak jenis(C) == "tensor", "Jenis hasil darab matriks mestilah tensor"
semak C.bentuk() == [2, 2], "Bentuk hasil mestilah [2, 2]"

ada c_list = C.keSenarai()
semak c_list[0][0] == 19.0, "Elemen [0][0] mestilah 19"
semak c_list[1][1] == 50.0, "Elemen [1][1] mestilah 50"

# Broadcasting 1D bias ke 2D tensor & ReLU
ada bias = tensor([1.0, -60.0])
ada Z = (C + bias).relu()
ada z_list = Z.keSenarai()
semak z_list[0][0] == 20.0, "20.0 selepas ReLU kekal 20.0"
semak z_list[1][1] == 0.0, "Nilai negatif selepas ReLU mestilah 0.0"

# LapisanLinear
ada lapisan = LapisanLinear(2, 3)
ada out = lapisan.maju(A)
semak out.bentuk() == [2, 3], "Keluaran LapisanLinear mestilah berbentuk [2, 3]"

# 2. Uji NLP PengiraTFIDF & Kesamaan Cosine
ada dokumen = [
    "bahasa melayu sangat hebat untuk pembelajaran mesin",
    "pembelajaran mesin dengan akalml laju dan pintar",
    "cuaca hari ini sangat cerah dan panas"
]
ada tfidf = PengiraTFIDF()
ada matriks_tfidf = tfidf.latihTransformasi(dokumen)
semak panjang(matriks_tfidf) == 3, "Mesti ada 3 vektor dokumen"

ada sim_ml = kesamaanCosine(matriks_tfidf[0], matriks_tfidf[1])
ada sim_cuaca = kesamaanCosine(matriks_tfidf[0], matriks_tfidf[2])
semak sim_ml > sim_cuaca, "Dokumen topik ML mesti lebih serupa antara satu sama lain berbanding dokumen cuaca"

# 3. Uji Softmax & Cross-Entropy Loss
ada logits = [[2.0, 1.0, 0.1]]
ada probs = matriksSoftmax(logits)
ada jumlah_prob = probs[0][0] + probs[0][1] + probs[0][2]
semak bundar(jumlah_prob * 100) == 100, "Jumlah kebarangkalian Softmax mestilah 1.0"
ada loss = ralatEntropiSilang([[1.0, 0.0, 0.0]], probs)
semak loss > 0.0 && loss < 1.0, "Cross-entropy loss mesti bernilai positif kecil"

# 4. Uji K-Fold Cross Validation
ada iris = muatIris()
kerja buatModelKNN()
    pulang PengelasKNN(3)
selesai
ada kfold = pengesahanSilang(buatModelKNN, iris["ciri"], iris["sasaran"], 3)
semak panjang(kfold["skor_lipatan"]) == 3, "Mesti ada 3 skor lipatan"
semak kfold["purata_skor"] >= 0.0, "Purata skor K-Fold mesti sah"
"#;
    let res = laksana_kod(code);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_akal_ml_matriks_selari_dan_info_perkakas() {
    let code = r#"
# 1. Uji infoPerkakas()
ada info = infoPerkakas()
semak info["teras_cpu"] >= 1, "Bilangan teras CPU mestilah sekurang-kurangnya 1"
semak info.adaKunci("enjin_matriks"), "Mesti ada kunci enjin_matriks"
semak info.adaKunci("gpu_cuda_dikesan"), "Mesti ada kunci gpu_cuda_dikesan"
semak info.adaKunci("gpu_rocm_dikesan"), "Mesti ada kunci gpu_rocm_dikesan"

# 2. Uji pendaraban matriks Tensor selari multi-teras (64x64 @ 64x64)
ada A = satuTensor([64, 64])
ada B = satuTensor([64, 64])
ada C = A @ B
semak C.bentuk() == [64, 64], "Bentuk hasil selari mestilah [64, 64]"
ada c_list = C.keSenarai()
semak c_list[0][0] == 64.0, "Elemen [0][0] mestilah 64.0"
semak c_list[63][63] == 64.0, "Elemen [63][63] mestilah 64.0"
semak C.jumlah() == 262144.0, "Jumlah semua elemen (64*64*64) mestilah 262144.0"

# 3. Uji pendaraban matriks Tensor besar (128x128 @ 128x128 - laluan GPU ROCm/CUDA jika tersedia)
ada G1 = satuTensor([128, 128])
ada G2 = satuTensor([128, 128])
ada G3 = G1 @ G2
semak G3.bentuk() == [128, 128], "Bentuk hasil 128x128 mestilah [128, 128]"
ada g_list = G3.keSenarai()
semak g_list[0][0] == 128.0, "Elemen [0][0] mestilah 128.0"
semak g_list[127][127] == 128.0, "Elemen [127][127] mestilah 128.0"
"#;
    let res = laksana_kod(code);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}

#[test]
fn test_jaringan_saraf_kecerunan_tidak_bergantung_pada_saiz_dataset() {
    let code = r#"
ambil "akal"

ada X = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]
ada Y = [[0.0], [1.0], [1.0], [0.0]]
ada X_besar = []
ada Y_besar = []
untuk i dari 1 hingga 25
    untuk j dari 0 hingga 3
        X_besar.tambah(X[j])
        Y_besar.tambah(Y[j])
    selesai
selesai

ada a = JaringanSaraf([2, 4, 1], 3.2)
ada b = JaringanSaraf([2, 4, 1], 3.2)
b.wajaran = [a.wajaran[0], a.wajaran[1]]
b.pincang = [a.pincang[0], a.pincang[1]]
a.latih(X, Y, 200, salah)
b.latih(X_besar, Y_besar, 200, salah)

ada pa = a.ramalKebarangkalian(X)
ada pb = b.ramalKebarangkalian(X)
untuk j dari 0 hingga 3
    semak mutlak(pa[j][0] - pb[j][0]) < 0.000001, "Latihan mestilah sama untuk data yang diulang"
selesai
"#;
    let res = laksana_kod(code);
    assert!(res.is_ok(), "Ralat: {:?}", res);
}
