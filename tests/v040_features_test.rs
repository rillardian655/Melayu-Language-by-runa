use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_operator_saluran_paip() {
    let source = r#"
kerja tambah_satu(n)
    pulang n + 1
selesai

kerja darab_dua(n)
    pulang n * 2
selesai

kerja beza(a, b)
    pulang a - b
selesai

ada hasil1 = 5 |> tambah_satu |> darab_dua
ada hasil2 = 10 |> beza(3)
ada hasil3 = 4 alir tambah_satu alir darab_dua
"#;
    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "Pelaksanaan saluran paip berjaya");

    let env = interpreter.env.borrow();
    assert_eq!(env.get("hasil1", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Nombor(12.0));
    assert_eq!(env.get("hasil2", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Nombor(7.0));
    assert_eq!(env.get("hasil3", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Nombor(10.0));
}

#[test]
fn test_padanan_corak_ekspresif() {
    let source = r#"
kerja klasifikasi_skor(skor)
    pulang padan skor
        bila 90 hingga 100 => "Cemerlang"
        bila 80 hingga 89 => "Kepujian"
        bila 50 hingga 79 => "Lulus"
        bila 0 hingga 49 => "Gagal"
        lain => "Tidak Sah"
    selesai
selesai

kerja periksa_nombor(n)
    pulang padan n
        bila 0 => "Sifar"
        bila x jika n > 0 => "Positif"
        lain => "Negatif"
    selesai
selesai

ada g1 = klasifikasi_skor(95)
ada g2 = klasifikasi_skor(82)
ada g3 = klasifikasi_skor(65)
ada g4 = klasifikasi_skor(30)
ada g5 = klasifikasi_skor(150)

ada p1 = periksa_nombor(0)
ada p2 = periksa_nombor(42)
ada p3 = periksa_nombor(-7)
"#;
    let program = huraikan(source).expect("Hurai padan patut berjaya");
    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "Pelaksanaan padan berjaya");

    let env = interpreter.env.borrow();
    assert_eq!(env.get("g1", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Cemerlang".to_string()));
    assert_eq!(env.get("g2", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Kepujian".to_string()));
    assert_eq!(env.get("g3", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Lulus".to_string()));
    assert_eq!(env.get("g4", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Gagal".to_string()));
    assert_eq!(env.get("g5", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Tidak Sah".to_string()));

    assert_eq!(env.get("p1", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Sifar".to_string()));
    assert_eq!(env.get("p2", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Positif".to_string()));
    assert_eq!(env.get("p3", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Negatif".to_string()));
}

#[test]
fn test_null_coalescing_dan_akses_selamat() {
    let source = r#"
ada data_kosong = kosong
ada nilai_lalai = data_kosong ?? "Nilai Pengganti"
ada nilai_ada = 123 ?? 456

kelas Pengguna
    bina(nama)
        diri.nama = nama
    selesai
    kerja sapa()
        pulang "Halo, " + diri.nama
    selesai
selesai

ada p = Pengguna("Ahmad")
ada p_kosong = kosong

ada nama_ada = p?.nama ?? "Anonim"
ada nama_kosong = p_kosong?.nama ?? "Anonim"

ada sapa_ada = p?.sapa() ?? "Tiada Sapaan"
ada sapa_kosong = p_kosong?.sapa() ?? "Tiada Sapaan"
"#;
    let program = huraikan(source).expect("Hurai selamat patut berjaya");
    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "Pelaksanaan null safe berjaya");

    let env = interpreter.env.borrow();
    assert_eq!(env.get("nilai_lalai", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Nilai Pengganti".to_string()));
    assert_eq!(env.get("nilai_ada", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Nombor(123.0));
    assert_eq!(env.get("nama_ada", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Ahmad".to_string()));
    assert_eq!(env.get("nama_kosong", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Anonim".to_string()));
    assert_eq!(env.get("sapa_ada", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Halo, Ahmad".to_string()));
    assert_eq!(env.get("sapa_kosong", melayu_core::ast::LineCol::new(1, 1)).unwrap(), Value::Teks("Tiada Sapaan".to_string()));
}

#[test]
fn test_suite_ujian_dan_semakan() {
    let source = r#"
kerja kira_diskaun(harga, peratus)
    semak harga >= 0, "Harga tidak boleh negatif"
    semak peratus >= 0 dan peratus <= 100, "Peratus mestilah antara 0 dan 100"
    pulang harga - (harga * peratus / 100)
selesai

uji "Kiraan diskaun normal"
    ada hasil = kira_diskaun(100, 20)
    semak hasil == 80, "Sepatutnya 80"
selesai

uji "Kiraan harga sifar"
    ada hasil = kira_diskaun(0, 50)
    semak hasil == 0
selesai

uji "Ujian ralat sengaja gagal"
    kira_diskaun(-10, 10)
selesai
"#;
    let program = huraikan(source).expect("Hurai suite ujian patut berjaya");
    let mut interpreter = Interpreter::new();
    let keputusan = interpreter.jalankan_ujian(&program).expect("Ujian patut selesai dinilai");

    assert_eq!(keputusan.len(), 3);
    assert_eq!(keputusan[0].tajuk, "Kiraan diskaun normal");
    assert!(keputusan[0].lulus, "Ujian 1 sepatutnya lulus");

    assert_eq!(keputusan[1].tajuk, "Kiraan harga sifar");
    assert!(keputusan[1].lulus, "Ujian 2 sepatutnya lulus");

    assert_eq!(keputusan[2].tajuk, "Ujian ralat sengaja gagal");
    assert!(!keputusan[2].lulus, "Ujian 3 sengaja gagal");
    assert!(keputusan[2].ralat.as_ref().unwrap().contains("Harga tidak boleh negatif"));
}
