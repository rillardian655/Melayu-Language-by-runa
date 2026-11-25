use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};
use std::fs;

#[test]
fn test_sekatan_rekursi_berlebihan() {
    let source = r#"
kerja pusing(n)
    pulang pusing(n + 1)
selesai

pusing(1)
"#;
    let program = huraikan(source).expect("Hurai patut berjaya");
    // Tetapkan had rekursi rendah (contoh 25) untuk ujian pantas
    let mut interpreter = Interpreter::new().with_recursion_limit(25);
    let result = interpreter.interpret(&program);

    assert!(result.is_err(), "Rekursi melampau sepatutnya disekat");
    let err = result.unwrap_err();
    assert_eq!(err.title, "Alamak!");
    assert!(
        err.message.contains("Had rekursi maksimum"),
        "Mesej ralat sepatutnya menyebut had rekursi: {}",
        err.message
    );
}

#[test]
fn test_pencegahan_gelung_tak_terhingga_dos() {
    let source = r#"
ada x = 1
selagi betul
    x = x + 1
selesai
"#;
    let program = huraikan(source).expect("Hurai patut berjaya");
    // Tetapkan had langkah rendah (500 langkah) untuk ujian selamat
    let mut interpreter = Interpreter::new().with_step_limit(Some(500));
    let result = interpreter.interpret(&program);

    assert!(result.is_err(), "Gelung tak terhingga patut dihentikan");
    let err = result.unwrap_err();
    assert_eq!(err.title, "Woy!");
    assert!(
        err.message.contains("Had pelaksanaan selamat"),
        "Mesej ralat sepatutnya menyebut had langkah: {}",
        err.message
    );
}

#[test]
fn test_pengesahan_indeks_ketat_nan_pecahan_dan_julat() {
    // 1. Indeks pecahan
    let source_pecahan = r#"
ada s = [10, 20, 30]
ada ambik = s[1.5]
"#;
    let prog1 = huraikan(source_pecahan).expect("Hurai berjaya");
    let mut interp1 = Interpreter::new();
    let res1 = interp1.interpret(&prog1);
    assert!(res1.is_err());
    assert!(res1.unwrap_err().message.contains("nombor bulat"));

    // 2. Indeks negatif melebihi panjang (underflow check)
    let source_underflow = r#"
ada s = [10, 20, 30]
ada ambik = s[-5]
"#;
    let prog2 = huraikan(source_underflow).expect("Hurai berjaya");
    let mut interp2 = Interpreter::new();
    let res2 = interp2.interpret(&prog2);
    assert!(res2.is_err());
    assert!(res2.unwrap_err().message.contains("luar julat"));

    // 3. Indeks rentetan negatif yang sah
    let source_negatif_sah = r#"
ada ayat = "melayu"
ada aksara_akhir = ayat[-1]
"#;
    let prog3 = huraikan(source_negatif_sah).expect("Hurai berjaya");
    let mut interp3 = Interpreter::new();
    interp3
        .interpret(&prog3)
        .expect("Patut berjaya baca indeks -1");
    let val = interp3
        .env
        .borrow()
        .get(
            "aksara_akhir",
            melayu_core::ast::LineCol { line: 1, col: 1 },
        )
        .unwrap();
    assert_eq!(val, Value::Teks("u".to_string()));
}

#[test]
fn test_fail_io_sandbox_selamat() {
    let fail_ujian = "ujian_keselamatan_temp.txt";

    // Pastikan fail bersih jika wujud sebelumnya
    let _ = fs::remove_file(fail_ujian);

    let script = format!(
        r#"
ada fail = "{}"
ada tulis_ok = tulisFail(fail, "Salam Serumpun Melayu!")
ada ada_fail = wujudFail(fail)
ada kandungan = bacaFail(fail)
ada padam_ok = padamFail(fail)
ada selepas_padam = wujudFail(fail)
"#,
        fail_ujian
    );

    let prog = huraikan(&script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    interp
        .interpret(&prog)
        .expect("Operasi fail sah dalam sandbox patut berjaya");

    let kand = interp
        .env
        .borrow()
        .get("kandungan", melayu_core::ast::LineCol { line: 1, col: 1 })
        .unwrap();
    assert_eq!(kand, Value::Teks("Salam Serumpun Melayu!".to_string()));

    let selepas = interp
        .env
        .borrow()
        .get(
            "selepas_padam",
            melayu_core::ast::LineCol { line: 1, col: 1 },
        )
        .unwrap();
    assert_eq!(selepas, Value::Salah);
}

#[test]
fn test_sekatan_path_traversal() {
    let script = r#"
bacaFail("../../etc/passwd")
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);

    assert!(
        res.is_err(),
        "Path traversal ke luar sandbox mestilah disekat!"
    );
    let err = res.unwrap_err();
    assert!(
        err.message.contains("dilarang") || err.message.contains("disekat"),
        "Mesej ralat traversal: {}",
        err.message
    );
}

#[test]
fn test_fungsi_rawak_terbina_dalam() {
    let script = r#"
ada r1 = rawak()
ada r2 = rawak(10, 20)
ada sah = (r1 >= 0.0) dan (r1 <= 1.0) dan (r2 >= 10) dan (r2 <= 20)
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    interp
        .interpret(&prog)
        .expect("rawak() patut berjaya dijalankan");

    let sah = interp
        .env
        .borrow()
        .get("sah", melayu_core::ast::LineCol { line: 1, col: 1 })
        .unwrap();
    assert_eq!(sah, Value::Betul);
}

#[test]
fn test_sekatan_fail_sensitif_git_dan_env() {
    for target in &[".git/config", ".env", ".env.production", ".ssh/id_rsa"] {
        let script = format!("bacaFail(\"{}\")", target);
        let prog = huraikan(&script).expect("Hurai berjaya");
        let mut interp = Interpreter::new();
        let res = interp.interpret(&prog);
        assert!(res.is_err(), "Akses ke '{}' sepatutnya disekat!", target);
    }
}

#[test]
fn test_sekatan_suntikan_crlf_http() {
    let script = r#"
hantarPermintaanHTTP("GET", "http://127.0.0.1:8080/test\r\nEvil-Header: 1")
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);
    assert!(res.is_err(), "Suntikan CRLF pada URL HTTP mestilah disekat!");
}

#[test]
fn test_sekatan_kedalaman_json_dos() {
    let deep_json = format!("{}{}", "[".repeat(510), "]".repeat(510));
    let script = format!("bacaJSON(\"{}\")", deep_json);
    let prog = huraikan(&script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);
    assert!(res.is_err(), "JSON melebihi had kedalaman mestilah disekat!");
}

#[test]
fn test_penapis_xss_selit_html_selasih() {
    let script = r#"
ambil "selasih"
ada bahaya = "<script>alert('xss')</script>"
ada selamat = selitHTML(bahaya)
semak bukan selamat.mengandungi("<script>"), "Tag skrip mentah mestilah ditukar!"
semak selamat.mengandungi("&lt;script&gt;"), "Tag mesti dinyahkod ke entiti HTML"
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);
    assert!(res.is_ok(), "Penapis XSS selitHTML gagal: {:?}", res);
}

#[test]
fn test_sekatan_oom_tensor() {
    let script = r#"
sifarTensor([100000, 100000])
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);
    assert!(res.is_err(), "Alokasi tensor raksasa mestilah disekat daripada OOM!");
}

#[test]
fn test_sekatan_ssrf_metadata_http() {
    let script = r#"
hantarPermintaanHTTP("GET", "http://169.254.169.254/latest/meta-data/")
"#;
    let prog = huraikan(script).expect("Hurai berjaya");
    let mut interp = Interpreter::new();
    let res = interp.interpret(&prog);
    assert!(res.is_err(), "Akses SSRF ke metadata awan mestilah disekat!");
}
