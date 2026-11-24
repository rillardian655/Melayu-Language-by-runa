use melayu_core::interpreter::Interpreter;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;
use std::thread;
use std::time::Duration;

fn laksana_kod(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("{:?}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("{:?}", e))?;
    let mut interpreter = Interpreter::new();
    interpreter
        .interpret(&program)
        .map_err(|e| format!("{:?}", e))?;
    Ok("OK".to_string())
}

#[test]
fn test_selasih_web_server_dan_klien() {
    let server_code = r#"
ambil "selasih"

ada app = PelayanWeb(18080)

kerja handleHalo(req)
    pulang balasJSON({"mesej": "Selamat datang ke Selasih Web!"}, 200)
selesai

kerja handleEcho(req)
    ada data = dariJSON(req["badan"])
    pulang balasJSON({"diterima": data["teks"]}, 201)
selesai

app.dapatkan("/halo", handleHalo)
app.hantar("/echo", handleEcho)

# Layan 2 permintaan kemudian berhenti
app.dengar(2)
"#;

    let client_code = r#"
# Uji permintaan GET
ada res1 = hantarPermintaanHTTP("GET", "http://127.0.0.1:18080/halo")
semak res1["status"] == 200, "Status GET mestilah 200"
ada data1 = dariJSON(res1["badan"])
semak data1["mesej"] == "Selamat datang ke Selasih Web!", "Kandungan GET salah"

# Uji permintaan POST
ada payload = tulisJSON({"teks": "Bahasa Melayu Mantap!"})
ada res2 = hantarPermintaanHTTP("POST", "http://127.0.0.1:18080/echo", payload)
semak res2["status"] == 201, "Status POST mestilah 201"
ada data2 = dariJSON(res2["badan"])
semak data2["diterima"] == "Bahasa Melayu Mantap!", "Kandungan POST salah"
"#;

    let handle = thread::spawn(move || {
        let res = laksana_kod(server_code);
        assert!(res.is_ok(), "Ralat pelayan: {:?}", res);
    });

    thread::sleep(Duration::from_millis(200));

    let client_res = laksana_kod(client_code);
    assert!(client_res.is_ok(), "Ralat klien: {:?}", client_res);

    handle.join().unwrap();
}
