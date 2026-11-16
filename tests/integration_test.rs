use melayu_core::jalankan;

#[test]
fn test_program_penuh_fizzbuzz() {
    let kod = r#"
    ada jumlah = 0
    untuk i dari 1 hingga 15
        jika i modulo 15 sama dengan 0
            jumlah = jumlah tambah 15
        jika tidak jika i modulo 3 sama dengan 0
            jumlah = jumlah tambah 3
        jika tidak jika i modulo 5 sama dengan 0
            jumlah = jumlah tambah 5
        selesai
    selesai
    "#;
    let res = jalankan(kod);
    assert!(res.is_ok());
}

#[test]
fn test_ralat_semantik_pembolehubah_tak_wujud() {
    let kod = r#"
    tunjuk nama
    "#;
    let err = jalankan(kod).expect_err("Sepatutnya ralat");
    assert!(err.message.contains("belum diisytihar"));
}

#[test]
fn test_ralat_ubah_tetap() {
    let kod = r#"
    tetap PI = 3.14
    PI = 3.14159
    "#;
    let err = jalankan(kod).expect_err("Sepatutnya ralat");
    assert!(err.message.contains("pembolehubah tetap"));
}

#[test]
fn test_sistem_modul_ambil() {
    let module_filename = "modul_ujian_temp.ml";
    let module_code = r#"
kerja ganda(x)
    pulang x kali 2
selesai

ada nomborVersi = 100
"#;
    std::fs::write(module_filename, module_code).expect("Gagal tulis modul ujian");

    // 1. Ambil terpilih (selective import)
    let kod_selective = format!(
        r#"
ambil ganda daripada "{}"
ada hasil = ganda(21)
"#,
        module_filename
    );
    let res1 = melayu_core::huraikan(&kod_selective).unwrap();
    let mut interp1 = melayu_core::interpreter::Interpreter::new();
    interp1.interpret(&res1).unwrap();
    assert_eq!(
        interp1
            .env
            .borrow()
            .get("hasil", melayu_core::ast::LineCol::new(1, 1))
            .unwrap(),
        melayu_core::interpreter::Value::Nombor(42.0)
    );

    // 2. Ambil sebagai kamus modul (aliased import)
    let kod_alias = format!(
        r#"
ambil "{}" sebagai mod
ada hasil_versi = mod["nomborVersi"]
"#,
        module_filename
    );
    let res2 = melayu_core::huraikan(&kod_alias).unwrap();
    let mut interp2 = melayu_core::interpreter::Interpreter::new();
    interp2.interpret(&res2).unwrap();
    assert_eq!(
        interp2
            .env
            .borrow()
            .get("hasil_versi", melayu_core::ast::LineCol::new(1, 1))
            .unwrap(),
        melayu_core::interpreter::Value::Nombor(100.0)
    );

    // 3. Ambil semua (wildcard import)
    let kod_wildcard = format!(
        r#"
ambil "{}"
ada hasil_w = ganda(5)
"#,
        module_filename
    );
    let res3 = melayu_core::huraikan(&kod_wildcard).unwrap();
    let mut interp3 = melayu_core::interpreter::Interpreter::new();
    interp3.interpret(&res3).unwrap();
    assert_eq!(
        interp3
            .env
            .borrow()
            .get("hasil_w", melayu_core::ast::LineCol::new(1, 1))
            .unwrap(),
        melayu_core::interpreter::Value::Nombor(10.0)
    );

    // Bersihkan fail ujian
    let _ = std::fs::remove_file(module_filename);
}
