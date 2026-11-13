use melayu_core::analyzer::Analyzer;
use melayu_core::interpreter::Interpreter;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;
use melayu_core::repl::{cetak_ralat, run_repl};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        // Tanpa argumen: Buka REPL
        run_repl();
        return;
    }

    match args[1].as_str() {
        "jalankan" | "run" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mAlamak! Sila nyatakan fail .ml yang ingin dijalankan.\x1b[0m");
                eprintln!("Contoh: melayu jalankan program.ml");
                process::exit(1);
            }
            jalankan_fail(&args[2]);
        }
        "semak" | "check" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mAlamak! Sila nyatakan fail .ml yang ingin disemak.\x1b[0m");
                eprintln!("Contoh: melayu semak program.ml");
                process::exit(1);
            }
            semak_fail(&args[2]);
        }
        "bantuan" | "--help" | "-h" => {
            cetak_bantuan();
        }
        "versi" | "--version" | "-v" => {
            println!("MELAYU v0.1.0");
            println!("Bahasa programming serumpun, untuk serumpun developer.");
        }
        fail if fail.ends_with(".ml") => {
            jalankan_fail(fail);
        }
        arahan_lain => {
            eprintln!(
                "\x1b[1;31mEh! Arahan '{}' tidak dikenali.\x1b[0m",
                arahan_lain
            );
            eprintln!("Gunakan 'melayu bantuan' untuk melihat senarai arahan.");
            process::exit(1);
        }
    }
}

fn jalankan_fail(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[1;31mWoy! Gagal membaca fail '{}': {}\x1b[0m", path, e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
    };

    let mut analyzer = Analyzer::new();
    if let Err(e) = analyzer.analyze(&program) {
        cetak_ralat(&e, &source);
        process::exit(1);
    }

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.interpret(&program) {
        cetak_ralat(&e, &source);
        process::exit(1);
    }
}

fn semak_fail(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[1;31mWoy! Gagal membaca fail '{}': {}\x1b[0m", path, e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
    };

    let mut analyzer = Analyzer::new();
    if let Err(e) = analyzer.analyze(&program) {
        cetak_ralat(&e, &source);
        process::exit(1);
    }

    println!("\x1b[1;32mTahniah! Fail '{}' sah dan bebas dari sebarang ralat sintaks atau semantik.\x1b[0m", path);
}

fn cetak_bantuan() {
    println!("\x1b[1;32m");
    println!("MELAYU v0.1.0 — CLI");
    println!("\"Bahasa programming serumpun, untuk serumpun developer.\"");
    println!("\x1b[0m");
    println!("PENGGUNAAN:");
    println!("  melayu                       Buka REPL interaktif");
    println!("  melayu jalankan <fail.ml>    Jalankan fail atur cara Melayu");
    println!("  melayu <fail.ml>             Jalankan fail secara terus");
    println!("  melayu semak <fail.ml>       Semak sintaks dan semantik fail");
    println!("  melayu bantuan               Papar bantuan ini");
    println!("  melayu versi                 Papar versi semasa");
    println!();
    println!("CONTOH:");
    println!("  melayu jalankan contoh/hello.ml");
    println!("  melayu contoh/fizzbuzz.ml");
}
