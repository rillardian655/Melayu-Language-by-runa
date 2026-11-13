use melayu_core::ast::Stmt;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;

#[test]
fn test_hurai_deklarasi_pembolehubah() {
    let kod = "ada x = 5\ntetap PI = 3.14";
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.pernyataan.len(), 2);
    match &program.pernyataan[0] {
        Stmt::IsytiharPembolehubah { nama, tetap, .. } => {
            assert_eq!(nama, "x");
            assert_eq!(*tetap, false);
        }
        _ => panic!("Bukan IsytiharPembolehubah"),
    }
    match &program.pernyataan[1] {
        Stmt::IsytiharPembolehubah { nama, tetap, .. } => {
            assert_eq!(nama, "PI");
            assert_eq!(*tetap, true);
        }
        _ => panic!("Bukan IsytiharPembolehubah"),
    }
}

#[test]
fn test_hurai_jika_selesai() {
    let kod = r#"
    jika x lebih 10
        tunjuk "Besar"
    jika tidak
        tunjuk "Kecil"
    selesai
    "#;
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.pernyataan.len(), 1);
    match &program.pernyataan[0] {
        Stmt::Jika {
            maka, jika_tidak, ..
        } => {
            assert_eq!(maka.len(), 1);
            assert!(jika_tidak.is_some());
        }
        _ => panic!("Bukan Stmt::Jika"),
    }
}

#[test]
fn test_hurai_fungsi() {
    let kod = r#"
    kerja darab(a, b)
        pulang a kali b
    selesai
    "#;
    let mut lexer = Lexer::new(kod);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.pernyataan.len(), 1);
    match &program.pernyataan[0] {
        Stmt::DeklarasiFungsi {
            nama,
            parameter,
            badan,
            ..
        } => {
            assert_eq!(nama, "darab");
            assert_eq!(parameter.len(), 2);
            assert_eq!(parameter[0].0, "a");
            assert_eq!(parameter[1].0, "b");
            assert_eq!(badan.len(), 1);
        }
        _ => panic!("Bukan DeklarasiFungsi"),
    }
}
