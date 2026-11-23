use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_import_modul_dan_skop() {
    let source = r#"
    ambil kiraDiskaun, adakahGenap daripada "examples/matematik_modul.ml"

    ada diskaun1 = kiraDiskaun(200, 15)
    ada genap1 = adakahGenap(42)
    ada genap2 = adakahGenap(43)
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("diskaun1", pos).unwrap(), Value::Nombor(170.0));
    assert_eq!(env.get("genap1", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("genap2", pos).unwrap(), Value::Salah);
}
