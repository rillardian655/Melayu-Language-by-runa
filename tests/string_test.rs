use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_kaedah_manipulasi_rentetan_teks() {
    let source = r#"
    ada teksAsal = "nusantara"
    ada besarAwal = teksAsal.hurufAwalBesar()
    ada ulangTeks = "ml".ulang(3)
    ada selit1 = "42".selitDepan(5, "0")
    ada selit2 = "RM".selitBelakang(6, ".")
    ada adakahMula = teksAsal.mulaDengan("nusan")
    ada adakahTamat = teksAsal.tamatDengan("tara")
    ada adakahKandung = teksAsal.mengandungi("santan")
    ada adakahKandungBenar = teksAsal.mengandungi("santa")
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(
        env.get("besarAwal", pos).unwrap(),
        Value::Teks("Nusantara".to_string())
    );
    assert_eq!(
        env.get("ulangTeks", pos).unwrap(),
        Value::Teks("mlmlml".to_string())
    );
    assert_eq!(
        env.get("selit1", pos).unwrap(),
        Value::Teks("00042".to_string())
    );
    assert_eq!(
        env.get("selit2", pos).unwrap(),
        Value::Teks("RM....".to_string())
    );
    assert_eq!(env.get("adakahMula", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("adakahTamat", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("adakahKandung", pos).unwrap(), Value::Salah);
    assert_eq!(env.get("adakahKandungBenar", pos).unwrap(), Value::Betul);
}
