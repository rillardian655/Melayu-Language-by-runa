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

#[test]
fn test_manipulasi_teks_dan_koleksi_lanjutan() {
    let kod = r#"
    ada teksAsal = "epal,durian,pisang"
    ada buah = teksAsal.pecah(",")
    buah.isih()
    ada teksTersusun = buah.cantum(" - ")
    buah.balik()
    ada terbalik = buah.cantum(",")

    ada ayat = "Selamat Pagi Nusantara"
    ada diganti = ayat.ganti("Pagi", "Petang")
    ada mula = ayat.mulaDengan("Selamat")
    ada tamat = ayat.tamatDengan("Nusantara")
    ada ada_pagi = ayat.mengandungi("Pagi")
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(
        env.get("teksTersusun", pos).unwrap(),
        Value::Teks("durian - epal - pisang".to_string())
    );
    assert_eq!(
        env.get("terbalik", pos).unwrap(),
        Value::Teks("pisang,epal,durian".to_string())
    );
    assert_eq!(
        env.get("diganti", pos).unwrap(),
        Value::Teks("Selamat Petang Nusantara".to_string())
    );
    assert_eq!(env.get("mula", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("tamat", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("ada_pagi", pos).unwrap(), Value::Betul);
}

#[test]
fn test_fungsi_matematik_terbina_dalam() {
    let kod = r#"
    ada r = bundar(3.6)
    ada f = lantai(3.9)
    ada c = siling(3.1)
    ada m = mutlak(-42)
    ada mx = maksimum(10, 50, 30)
    ada mn = minimum(10, 50, 5)
    "#;
    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("r", pos).unwrap(), Value::Nombor(4.0));
    assert_eq!(env.get("f", pos).unwrap(), Value::Nombor(3.0));
    assert_eq!(env.get("c", pos).unwrap(), Value::Nombor(4.0));
    assert_eq!(env.get("m", pos).unwrap(), Value::Nombor(42.0));
    assert_eq!(env.get("mx", pos).unwrap(), Value::Nombor(50.0));
    assert_eq!(env.get("mn", pos).unwrap(), Value::Nombor(5.0));
}
