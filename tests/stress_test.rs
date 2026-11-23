use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_tekanan_senarai_besar() {
    let source = r#"
    ada senarai = []
    untuk i dari 1 hingga 2000
        senarai.tambah(i)
    selesai

    ada jumlah = 0
    untuk x dalam senarai
        jumlah = jumlah + x
    selesai
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    // Sum 1..2000 = 2000 * 2001 / 2 = 2,001,000
    assert_eq!(env.get("jumlah", pos).unwrap(), Value::Nombor(2_001_000.0));
}

#[test]
fn test_tekanan_instansiasi_banyak_objek() {
    let source = r#"
    kelas Item
        bina(id, nama)
            diri.id = id
            diri.nama = nama
        selesai

        kerja ambilId()
            pulang diri.id
        selesai
    selesai

    ada gudang = []
    untuk k dari 1 hingga 500
        gudang.tambah(Item(k, "Barang"))
    selesai

    ada idTerakhir = gudang[-1].ambilId()
    ada saizGudang = gudang.panjang()
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("saizGudang", pos).unwrap(), Value::Nombor(500.0));
    assert_eq!(env.get("idTerakhir", pos).unwrap(), Value::Nombor(500.0));
}
