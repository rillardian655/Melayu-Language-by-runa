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

