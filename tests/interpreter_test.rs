use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_aritmetik_dan_logik() {
    let kod = r#"
    ada a = 10
    ada b = 20
    ada c = a tambah b kali 2
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let c_val = interp
        .env
        .borrow()
        .get("c", melayu_core::ast::LineCol::new(1, 1))
        .unwrap();
    assert_eq!(c_val, Value::Nombor(50.0));
}

#[test]
fn test_fungsi_dan_rekursi() {
    let kod = r#"
    kerja faktorial(n)
        jika n kurang sama 1
            pulang 1
        selesai
        pulang n kali faktorial(n tolak 1)
    selesai

    ada hasil = faktorial(5)
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let hasil = interp
        .env
        .borrow()
        .get("hasil", melayu_core::ast::LineCol::new(1, 1))
        .unwrap();
    assert_eq!(hasil, Value::Nombor(120.0));
}

#[test]
fn test_senarai_dan_kaedah() {
    let kod = r#"
    ada s = [1, 2, 3]
    s.tambah(4)
    ada saiz = panjang(s)
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let saiz = interp
        .env
        .borrow()
        .get("saiz", melayu_core::ast::LineCol::new(1, 1))
        .unwrap();
    assert_eq!(saiz, Value::Nombor(4.0));
}

#[test]
fn test_cuba_tangkap() {
    let kod = r#"
    ada ralat_dikesan = salah
    cuba
        ada x = 10 bahagi 0
    tangkap e
        ralat_dikesan = betul
    selesai
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let status = interp
        .env
        .borrow()
        .get("ralat_dikesan", melayu_core::ast::LineCol::new(1, 1))
        .unwrap();
    assert_eq!(status, Value::Betul);
}
