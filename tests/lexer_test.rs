use melayu_core::lexer::Lexer;
use melayu_core::tokens::TokenKind;

#[test]
fn test_tokenisasi_asas() {
    let kod = "ada nama = \"Budi\"\ntetap umur = 17";
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().expect("Tokenisasi sepatutnya berjaya");

    assert_eq!(tokens[0].kind, TokenKind::Ada);
    assert_eq!(tokens[1].kind, TokenKind::Pengenal("nama".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::SamaDenganSatu);
    assert_eq!(tokens[3].kind, TokenKind::Teks("Budi".to_string()));
    assert_eq!(tokens[4].kind, TokenKind::BarisBaru);
    assert_eq!(tokens[5].kind, TokenKind::Tetap);
    assert_eq!(tokens[6].kind, TokenKind::Pengenal("umur".to_string()));
    assert_eq!(tokens[7].kind, TokenKind::SamaDenganSatu);
    assert_eq!(tokens[8].kind, TokenKind::Nombor(17.0));
}

#[test]
fn test_operator_perkataan_dan_simbol() {
    let kod = "10 tambah 5 sama dengan 15 dan bukan salah";
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().expect("Tokenisasi berjaya");

    assert_eq!(tokens[0].kind, TokenKind::Nombor(10.0));
    assert_eq!(tokens[1].kind, TokenKind::Tambah);
    assert_eq!(tokens[2].kind, TokenKind::Nombor(5.0));
    assert_eq!(tokens[3].kind, TokenKind::SamaDengan);
    assert_eq!(tokens[4].kind, TokenKind::Nombor(15.0));
    assert_eq!(tokens[5].kind, TokenKind::Dan);
    assert_eq!(tokens[6].kind, TokenKind::Bukan);
    assert_eq!(tokens[7].kind, TokenKind::Salah);
}

#[test]
fn test_kata_kunci_gabungan() {
    let kod = "jika tidak jika x lebih sama 10";
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().expect("Tokenisasi berjaya");

    assert_eq!(tokens[0].kind, TokenKind::JikaTidakJika);
    assert_eq!(tokens[1].kind, TokenKind::Pengenal("x".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::LebihSama);
    assert_eq!(tokens[3].kind, TokenKind::Nombor(10.0));
}

#[test]
fn test_pengesan_typo() {
    let kod = "tunjukkan \"Hai\"";
    let mut lexer = Lexer::new(kod);
    let err = lexer.tokenize().expect_err("Sepatutnya ralat typo");
    assert!(err.message.contains("bukan keyword Melayu"));
}

