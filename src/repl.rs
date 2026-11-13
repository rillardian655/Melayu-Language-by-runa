use crate::analyzer::Analyzer;
use crate::ast::Stmt;
use crate::errors::MelayuError;
use crate::interpreter::{Interpreter, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

pub fn run_repl() {
    println!("\x1b[1;32m");
    println!("  __  __ _____ _        _ __   ___   _ ");
    println!(" |  \\/  | ____| |      / \\\\ \\ / / | | |");
    println!(" | |\\/| |  _| | |     / _ \\\\ V /| | | |");
    println!(" | |  | | |___| |___ / ___ \\| | | |_| |");
    println!(" |_|  |_|_____|_____/_/   \\_\\_|  \\___/ ");
    println!("\x1b[0m");
    println!("MELAYU v0.1.0 — REPL Interaktif");
    println!("\"Bahasa programming serumpun, untuk serumpun developer.\"");
    println!("Taip kod kau di bawah. Taip \"keluar\" atau tekan Ctrl+C untuk berhenti.\n");

    let mut interpreter = Interpreter::new();
    let mut buffer = String::new();

    loop {
        if buffer.is_empty() {
            print!("\x1b[1;34mmelayu>\x1b[0m ");
        } else {
            print!("\x1b[1;33m  ... >\x1b[0m ");
        }
        let _ = io::stdout().flush();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break, // EOF / Ctrl+D
            Ok(_) => {}
            Err(err) => {
                eprintln!("Alamak! Ralat membaca input: {}", err);
                break;
            }
        }

        let trimmed = line.trim();
        if buffer.is_empty() && (trimmed == "keluar" || trimmed == "exit" || trimmed == "quit") {
            println!("Jumpa lagi! Selamat bergotong-royong!");
            break;
        }

        buffer.push_str(&line);

        // Cuba huraikan kod
        let mut lexer = Lexer::new(&buffer);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                // Jika error adalah rentetan belum ditutup atau komentar belum ditutup, teruskan membaca input
                if e.message.contains("tidak ditutup") {
                    continue;
                }
                cetak_ralat(&e, &buffer);
                buffer.clear();
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                // Jika error adalah blok belum ditutup, teruskan membaca multi-baris
                if e.message.contains("belum ditutup") {
                    continue;
                }
                cetak_ralat(&e, &buffer);
                buffer.clear();
                continue;
            }
        };

        // Semakan Analyzer (hanya untuk pengesyoran dan amaran cepat)
        let mut analyzer = Analyzer::new();
        if let Err(e) = analyzer.analyze(&program) {
            // Dalam REPL, jangan sekat jika pembolehubah sudah wujud dari baris sebelumnya
            if !e.message.contains("dah wujud") {
                cetak_ralat(&e, &buffer);
                buffer.clear();
                continue;
            }
        }

        // Jalankan program di REPL
        for stmt in &program.pernyataan {
            match stmt {
                Stmt::Ungkapan(expr, _) => {
                    match interpreter.evaluate_expr(expr, &interpreter.env.clone()) {
                        Ok(val) => {
                            if val != Value::Kosong {
                                println!("\x1b[36m=>\x1b[0m {}", val);
                            }
                        }
                        Err(e) => cetak_ralat(&e, &buffer),
                    }
                }
                _ => {
                    if let Err(e) = interpreter.execute_stmt(stmt, &interpreter.env.clone()) {
                        cetak_ralat(&e, &buffer);
                    }
                }
            }
        }

        buffer.clear();
    }
}

pub fn cetak_ralat(e: &MelayuError, source: &str) {
    let lines: Vec<&str> = source.lines().collect();
    let src_line = if e.line > 0 && e.line <= lines.len() {
        Some(lines[e.line - 1].to_string())
    } else {
        None
    };

    let mut err = e.clone();
    if let Some(src) = src_line {
        err = err.with_source(&src);
    }

    eprintln!("\x1b[1;31m{}\x1b[0m", err);
}
