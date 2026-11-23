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

#[test]
fn test_operasi_fungsian_senarai() {
    let kod = r#"
    ada nombor = [1, 2, 3, 4, 5]

    kerja gandaDua(x)
        pulang x kali 2
    selesai

    kerja genap(x)
        pulang (x % 2) == 0
    selesai

    kerja tambahSemua(terkumpul, x)
        pulang terkumpul tambah x
    selesai

    kerja lebihBesarDariTiga(x)
        pulang x lebih 3
    selesai

    ada hasilPetakan = nombor.petakan(gandaDua)
    ada hasilTapis = nombor.tapis(genap)
    ada hasilKira = nombor.kiraSemua(tambahSemua, 0)
    ada hasilCari = nombor.cari(lebihBesarDariTiga)
    ada idxAda = nombor.indeks(3)
    ada idxTiada = nombor.indeks(99)

    ada jumlahSetiap = 0
    kerja campur(item)
        jumlahSetiap = jumlahSetiap tambah item
    selesai
    nombor.setiapSatu(campur)
    "#;

    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    // petakan -> [2, 4, 6, 8, 10]
    let petakan_val = env.get("hasilPetakan", pos).unwrap();
    if let Value::Senarai(l) = petakan_val {
        let b = l.borrow();
        assert_eq!(b.len(), 5);
        assert_eq!(b[0], Value::Nombor(2.0));
        assert_eq!(b[4], Value::Nombor(10.0));
    } else {
        panic!("hasilPetakan sepatutnya senarai");
    }

    // tapis -> [2, 4]
    let tapis_val = env.get("hasilTapis", pos).unwrap();
    if let Value::Senarai(l) = tapis_val {
        let b = l.borrow();
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], Value::Nombor(2.0));
        assert_eq!(b[1], Value::Nombor(4.0));
    } else {
        panic!("hasilTapis sepatutnya senarai");
    }

    // kiraSemua -> 1+2+3+4+5 = 15
    assert_eq!(env.get("hasilKira", pos).unwrap(), Value::Nombor(15.0));

    // cari -> 4
    assert_eq!(env.get("hasilCari", pos).unwrap(), Value::Nombor(4.0));

    // indeks
    assert_eq!(env.get("idxAda", pos).unwrap(), Value::Nombor(2.0));
    assert_eq!(env.get("idxTiada", pos).unwrap(), Value::Nombor(-1.0));

    // setiapSatu
    assert_eq!(env.get("jumlahSetiap", pos).unwrap(), Value::Nombor(15.0));
}

#[test]
fn test_kamus_kaedah_lanjutan() {
    let kod = r#"
    ada data = {
        "nama": "Runa",
        "umur": 20,
        "bahasa": "MELAYU"
    }

    ada adaNama = data.adaKunci("nama")
    ada adaGaji = data.adaKunci("gaji")

    ada umurDibuang = data.buang("umur")
    ada umurMasihAda = data.adaKunci("umur")
    ada saizKini = data.panjang()
    "#;

    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("adaNama", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("adaGaji", pos).unwrap(), Value::Salah);
    assert_eq!(env.get("umurDibuang", pos).unwrap(), Value::Nombor(20.0));
    assert_eq!(env.get("umurMasihAda", pos).unwrap(), Value::Salah);
    assert_eq!(env.get("saizKini", pos).unwrap(), Value::Nombor(2.0));
}

#[test]
fn test_format_teks_lanjutan() {
    let kod = r#"
    ada bintang = "*".ulang(5)
    ada nomborPad = "42".selitDepan(5, "0")
    ada tajukPad = "Bab 1".selitBelakang(10, ".")
    ada nama = "melayu nusantara".hurufAwalBesar()
    "#;

    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(
        env.get("bintang", pos).unwrap(),
        Value::Teks("*****".to_string())
    );
    assert_eq!(
        env.get("nomborPad", pos).unwrap(),
        Value::Teks("00042".to_string())
    );
    assert_eq!(
        env.get("tajukPad", pos).unwrap(),
        Value::Teks("Bab 1.....".to_string())
    );
    assert_eq!(
        env.get("nama", pos).unwrap(),
        Value::Teks("Melayu nusantara".to_string())
    );
}

#[test]
fn test_parser_dan_serializer_json() {
    let kod = r#"
    ada teksAsal = "{\"id\": 101, \"nama\": \"Nusantara\", \"aktif\": betul, \"skor\": [95, 88]}"
    # Nota: teks JSON standard menggunakan true, tetapi parser kita menyokong true/false/null
    ada jsonBaku = "{\"id\": 101, \"nama\": \"Nusantara\", \"aktif\": true, \"skor\": [95, 88], \"kosong\": null}"
    ada parsed = bacaJSON(jsonBaku)

    ada id = parsed["id"]
    ada nama = parsed["nama"]
    ada aktif = parsed["aktif"]
    ada skorSatu = parsed["skor"][0]
    ada k = parsed["kosong"]

    ada ditukarSemula = tulisJSON(parsed)
    ada parsedSemula = bacaJSON(ditukarSemula)
    ada idSemula = parsedSemula["id"]
    "#;

    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    assert_eq!(env.get("id", pos).unwrap(), Value::Nombor(101.0));
    assert_eq!(
        env.get("nama", pos).unwrap(),
        Value::Teks("Nusantara".to_string())
    );
    assert_eq!(env.get("aktif", pos).unwrap(), Value::Betul);
    assert_eq!(env.get("skorSatu", pos).unwrap(), Value::Nombor(95.0));
    assert_eq!(env.get("k", pos).unwrap(), Value::Kosong);
    assert_eq!(env.get("idSemula", pos).unwrap(), Value::Nombor(101.0));
}

#[test]
fn test_utiliti_masa_dan_tarikh() {
    let kod = r#"
    ada ts = capMasa()
    ada masa = masaKini()
    ada tarikh = tarikhKini()
    "#;

    let program = huraikan(kod).unwrap();
    let mut interp = Interpreter::new();
    interp.interpret(&program).unwrap();

    let env = interp.env.borrow();
    let pos = melayu_core::ast::LineCol::new(1, 1);

    let ts_val = env.get("ts", pos).unwrap();
    if let Value::Nombor(n) = ts_val {
        assert!(n > 1_700_000_000.0, "Timestamp patut lebih dari tahun 2023");
    } else {
        panic!("capMasa patut memulangkan nombor");
    }

    let masa_val = env.get("masa", pos).unwrap();
    if let Value::Teks(m) = masa_val {
        assert_eq!(m.len(), 8);
        assert_eq!(&m[2..3], ":");
        assert_eq!(&m[5..6], ":");
    } else {
        panic!("masaKini patut memulangkan teks format HH:MM:SS");
    }

    let tarikh_val = env.get("tarikh", pos).unwrap();
    if let Value::Teks(t) = tarikh_val {
        assert_eq!(t.len(), 10);
        assert_eq!(&t[4..5], "-");
        assert_eq!(&t[7..8], "-");
    } else {
        panic!("tarikhKini patut memulangkan teks format YYYY-MM-DD");
    }
}
