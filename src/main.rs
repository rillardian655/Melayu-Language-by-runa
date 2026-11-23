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
        "uji" | "test" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mAlamak! Sila nyatakan fail .ml untuk diuji.\x1b[0m");
                eprintln!("Contoh: melayu uji fail_ujian.ml");
                process::exit(1);
            }
            uji_fail(&args[2]);
        }
        "tanda-aras" | "benchmark" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mAlamak! Sila nyatakan fail .ml untuk ditanda aras.\x1b[0m");
                eprintln!("Contoh: melayu tanda-aras program.ml [jumlah_lelaran]");
                process::exit(1);
            }
            let lelaran: usize = if args.len() >= 4 {
                args[3].parse().unwrap_or(5)
            } else {
                5
            };
            tanda_aras_fail(&args[2], lelaran);
        }
        "bantuan" | "--help" | "-h" => {
            cetak_bantuan();
        }
        "versi" | "--version" | "-v" => {
            println!("MELAYU v0.4.0");
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

fn uji_fail(path: &str) {
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

    println!("\x1b[1;36m=== SUITE UJIAN MELAYU v0.4.0 ===\x1b[0m");
    println!("Menjalankan ujian untuk: \x1b[1;33m{}\x1b[0m", path);
    println!("{}", "-".repeat(50));

    let mut interpreter = Interpreter::new();
    let keputusan = match interpreter.jalankan_ujian(&program) {
        Ok(res) => res,
        Err(e) => {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
    };

    if keputusan.is_empty() {
        println!("\x1b[1;33mTiada blok 'uji' ditemui dalam fail ini.\x1b[0m");
        println!("Petua: Gunakan sintaks 'uji \"nama ujian\" {{ ... }}' untuk mentakrifkan ujian.");
        return;
    }

    let mut jumlah_lulus = 0;
    let mut jumlah_gagal = 0;
    let mut jumlah_masa = 0.0;

    for u in &keputusan {
        jumlah_masa += u.tempoh_ms;
        if u.lulus {
            jumlah_lulus += 1;
            println!("  \x1b[1;32m[LULUS]\x1b[0m {} \x1b[90m({:.2} ms)\x1b[0m", u.tajuk, u.tempoh_ms);
        } else {
            jumlah_gagal += 1;
            println!("  \x1b[1;31m[GAGAL]\x1b[0m {} \x1b[90m({:.2} ms)\x1b[0m", u.tajuk, u.tempoh_ms);
            if let Some(ref r) = u.ralat {
                println!("    \x1b[31m-> Ralat: {}\x1b[0m", r);
            }
        }
    }

    println!("{}", "-".repeat(50));
    if jumlah_gagal == 0 {
        println!(
            "\x1b[1;32mKEPUTUSAN: SEMUA UJIAN LULUS! ({}/{} ujian, {:.2} ms)\x1b[0m",
            jumlah_lulus,
            keputusan.len(),
            jumlah_masa
        );
    } else {
        println!(
            "\x1b[1;31mKEPUTUSAN: {} LULUS, {} GAGAL dari {} ujian ({:.2} ms)\x1b[0m",
            jumlah_lulus,
            jumlah_gagal,
            keputusan.len(),
            jumlah_masa
        );
        process::exit(1);
    }
}

fn tanda_aras_fail(path: &str, lelaran: usize) {
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

    println!("\x1b[1;36m=== PENANDA ARAS KINERJA MELAYU v0.4.0 ===\x1b[0m");
    println!("Fail: \x1b[1;33m{}\x1b[0m | Jumlah Lelaran: \x1b[1;32m{}\x1b[0m", path, lelaran);
    println!("{}", "-".repeat(50));

    let mut rekod_masa: Vec<f64> = Vec::with_capacity(lelaran);

    for i in 1..=lelaran {
        let mula = std::time::Instant::now();
        let mut interpreter = Interpreter::new();
        if let Err(e) = interpreter.interpret(&program) {
            cetak_ralat(&e, &source);
            process::exit(1);
        }
        let tempoh_ms = mula.elapsed().as_secs_f64() * 1000.0;
        rekod_masa.push(tempoh_ms);
        println!("  Lelaran {:>2}: {:>8.2} ms", i, tempoh_ms);
    }

    let min = rekod_masa.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = rekod_masa.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let purata = rekod_masa.iter().sum::<f64>() / (lelaran as f64);

    println!("{}", "-".repeat(50));
    println!("  Purata Masa     (Mean) : \x1b[1;32m{:.2} ms\x1b[0m", purata);
    println!("  Terkencang      (Min)  : \x1b[1;34m{:.2} ms\x1b[0m", min);
    println!("  Paling Perlahan (Max)  : \x1b[1;33m{:.2} ms\x1b[0m", max);
    println!("{}", "-".repeat(50));
}

fn cetak_bantuan() {
    println!("\x1b[1;32m");
    println!("MELAYU v0.4.0 — CLI");
    println!("\"Bahasa programming serumpun, untuk serumpun developer.\"");
    println!("\x1b[0m");
    println!("PENGGUNAAN:");
    println!("  melayu                             Buka REPL interaktif");
    println!("  melayu jalankan <fail.ml>          Jalankan fail atur cara Melayu");
    println!("  melayu <fail.ml>                   Jalankan fail secara terus");
    println!("  melayu semak <fail.ml>             Semak sintaks dan semantik fail");
    println!("  melayu uji <fail.ml>               Jalankan suite ujian unit");
    println!("  melayu tanda-aras <fail.ml> [n]    Jalankan penanda aras masa (benchmark)");
    println!("  melayu bantuan                     Papar bantuan ini");
    println!("  melayu versi                       Papar versi semasa");
    println!();
    println!("CONTOH:");
    println!("  melayu jalankan contoh/hello.ml");
    println!("  melayu uji contoh/suite_ujian.ml");
    println!("  melayu tanda-aras contoh/kiraan_matriks.ml 10");
}
