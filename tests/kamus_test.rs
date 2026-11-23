use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_operasi_kamus_menyeluruh() {
    let source = r#"
    ada m = kamus {
        "nama": "Melayu",
        "tahun": 2026,
        "aktif": betul
    }

    ada ada_nama = m.adaKunci("nama")
    ada ada_tiada = m.adaKunci("bukanKunci")

    m["versi"] = "0.3.0"
    ada nilai_versi = m["versi"]

    ada buang_tahun = m.buang("tahun")
    ada tahun_selepas = m["tahun"]
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("ada_nama", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("ada_tiada", pos).unwrap(), Value::Salah);
    assert_eq!(
        env.get("nilai_versi", pos).unwrap(),
        Value::Teks("0.3.0".to_string())
    );
    assert_eq!(env.get("buang_tahun", pos).unwrap(), Value::Nombor(2026.0));
    assert_eq!(env.get("tahun_selepas", pos).unwrap(), Value::Kosong);
}
