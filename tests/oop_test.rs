use melayu_core::huraikan;
use melayu_core::interpreter::{Interpreter, Value};

#[test]
fn test_pewarisan_tiga_peringkat() {
    let source = r#"
    kelas Datuk
        bina(nama)
            diri.namaDatuk = nama
            diri.generasi = 1
        selesai

        kerja asalUsul()
            pulang "Keturunan " + diri.namaDatuk
        selesai
    selesai

    kelas Ayah warisi Datuk
        bina(namaDatuk, namaAyah)
            induk.bina(namaDatuk)
            diri.namaAyah = namaAyah
            diri.generasi = 2
        selesai

        kerja peranan()
            pulang "Bapa Keluarga"
        selesai
    selesai

    kelas Anak warisi Ayah
        bina(namaDatuk, namaAyah, namaAnak)
            induk.bina(namaDatuk, namaAyah)
            diri.namaAnak = namaAnak
            diri.generasi = 3
        selesai

        kerja peranan()
            pulang "Generasi Muda"
        selesai
    selesai

    ada budi = Anak("Tok Budi", "Pak Ali", "Budi Muda")
    ada gen = budi.generasi
    ada asal = budi.asalUsul()
    ada peranan_anak = budi.peranan()
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("gen", pos).unwrap(), Value::Nombor(3.0));
    assert_eq!(
        env.get("asal", pos).unwrap(),
        Value::Teks("Keturunan Tok Budi".to_string())
    );
    assert_eq!(
        env.get("peranan_anak", pos).unwrap(),
        Value::Teks("Generasi Muda".to_string())
    );
}

#[test]
fn test_polimorfisme_kaedah_dan_senarai_objek() {
    let source = r#"
    kelas Haiwan
        bina(nama)
            diri.nama = nama
        selesai

        kerja bersuara()
            pulang "Bunyi haiwan..."
        selesai
    selesai

    kelas Kucing warisi Haiwan
        bina(nama)
            induk.bina(nama)
        selesai

        kerja bersuara()
            pulang diri.nama + ": Meow!"
        selesai
    selesai

    kelas Anjing warisi Haiwan
        bina(nama)
            induk.bina(nama)
        selesai

        kerja bersuara()
            pulang diri.nama + ": Woof!"
        selesai
    selesai

    ada senaraiHaiwan = [Kucing("Oyen"), Anjing("Tom")]
    ada bunyi1 = senaraiHaiwan[0].bersuara()
    ada bunyi2 = senaraiHaiwan[1].bersuara()
    "#;

    let program = huraikan(source).expect("Hurai patut berjaya");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Eksekusi patut berjaya");

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(
        env.get("bunyi1", pos).unwrap(),
        Value::Teks("Oyen: Meow!".to_string())
    );
    assert_eq!(
        env.get("bunyi2", pos).unwrap(),
        Value::Teks("Tom: Woof!".to_string())
    );
}
