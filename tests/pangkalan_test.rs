use melayu_core::analyzer::Analyzer;
use melayu_core::interpreter::Interpreter;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;
use std::fs;

fn laksana_kod(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("{:?}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("{:?}", e))?;
    let mut analyzer = Analyzer::new();
    analyzer.analyze(&program).map_err(|e| format!("{:?}", e))?;
    let mut interpreter = Interpreter::new();
    interpreter
        .interpret(&program)
        .map_err(|e| format!("{:?}", e))?;
    Ok("OK".to_string())
}

#[test]
fn test_pangkalan_data_crud_dan_kegigihan() {
    let db_file = "target/test_pangkalan_crud.json";
    let _ = fs::remove_file(db_file);

    let code = r#"
ambil "pangkalan"

ada db = Pangkalan("target/test_pangkalan_crud.json")
db.ciptaJadual("pelanggan")

ada r1 = db.sisip("pelanggan", {"nama": "Ali", "peranan": "admin", "skor": 90})
ada r2 = db.sisip("pelanggan", {"nama": "Budi", "peranan": "ahli", "skor": 75})
ada r3 = db.sisip("pelanggan", {"nama": "Citra", "peranan": "ahli", "skor": 88})

semak r1["_id"] == 1, "ID rekod pertama mestilah 1"
semak db.bilang("pelanggan") == 3, "Jumlah rekod mestilah 3"

ada ahli = db.cari("pelanggan", {"peranan": "ahli"})
semak panjang(ahli) == 2, "Sepatutnya ada 2 ahli"

ada admin = db.cariSatu("pelanggan", {"peranan": "admin"})
semak admin["nama"] == "Ali", "Admin mestilah Ali"

ada dikemaskini = db.kemaskini("pelanggan", {"nama": "Budi"}, {"skor": 95})
semak dikemaskini == 1, "Sepatutnya 1 rekod dikemaskini"

# Muat semula daripada fail cakera untuk menguji kegigihan (persistence)
ada db2 = Pangkalan("target/test_pangkalan_crud.json")
ada budi = db2.cariSatu("pelanggan", {"nama": "Budi"})
semak budi["skor"] == 95, "Skor Budi selepas dimuat semula mestilah 95"

ada dipadam = db2.padam("pelanggan", {"peranan": "ahli"})
semak dipadam == 2, "Sepatutnya 2 rekod dipadam"
semak db2.bilang("pelanggan") == 1, "Baki rekod mestilah 1"
"#;

    let res = laksana_kod(code);
    let _ = fs::remove_file(db_file);
    assert!(res.is_ok(), "Ralat pangkalan: {:?}", res);
}

