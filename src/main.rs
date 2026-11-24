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
        "mula" | "init" => {
            let nama = args.get(2).map(|s| s.as_str());
            mula_projek(nama);
        }
        "pasang" | "install" => {
            let sasaran = args.get(2).map(|s| s.as_str());
            pasang_pakej(sasaran);
        }
        "pakej" | "packages" | "senarai-pakej" => {
            senarai_pakej();
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
    println!("  melayu mula [nama_projek]          Cipta struktur projek Melayu baharu");
    println!("  melayu pasang [nama_atau_fail]     Pasang modul ke modul_melayu/");
    println!("  melayu pakej                       Senarai pakej dan framework tersedia");
    println!("  melayu bantuan                     Papar bantuan ini");
    println!("  melayu versi                       Papar versi semasa");
    println!();
    println!("CONTOH:");
    println!("  melayu mula aplikasi_saya");
    println!("  melayu pasang alat_kira.ml");
    println!("  melayu jalankan utama.ml");
}

fn mula_projek(nama_opt: Option<&str>) {
    let nama_projek = nama_opt.unwrap_or("projek_melayu");
    let sasaran_dir = std::path::Path::new(nama_projek);

    if nama_opt.is_some() && !sasaran_dir.exists() {
        if let Err(e) = fs::create_dir_all(sasaran_dir) {
            eprintln!("\x1b[1;31mAlamak! Gagal mencipta direktori '{}': {}\x1b[0m", nama_projek, e);
            process::exit(1);
        }
    }

    let pakej_path = sasaran_dir.join("pakej.json");
    let utama_path = sasaran_dir.join("utama.ml");
    let modul_dir = sasaran_dir.join("modul_melayu");
    let gitignore_path = sasaran_dir.join(".gitignore");
    let readme_path = sasaran_dir.join("README.md");

    let pakej_json_content = format!(
        r#"{{
  "nama": "{}",
  "versi": "0.1.0",
  "penerangan": "Projek dibina menggunakan Bahasa Melayu",
  "utama": "utama.ml",
  "kebergantungan": {{}}
}}
"#,
        nama_projek
    );

    let utama_ml_content = format!(
        r#"# Titik masuk utama untuk projek {}
tunjuk "========================================"
tunjuk "  Selamat Datang ke Projek: {}"
tunjuk "========================================"
tunjuk ""

tunjuk "Alhamdulillah, atur cara Bahasa Melayu berjaya dijalankan!"
"#,
        nama_projek, nama_projek
    );

    let _ = fs::write(&pakej_path, pakej_json_content);
    let _ = fs::write(&utama_path, utama_ml_content);
    let _ = fs::create_dir_all(&modul_dir);
    let _ = fs::write(modul_dir.join(".gitkeep"), "");
    let _ = fs::write(&gitignore_path, "target/\n*.tmp\n");
    let _ = fs::write(&readme_path, format!("# {}\n\nProjek Bahasa Melayu.\n\n## Cara Menjalankan\n```bash\nmelayu utama.ml\n```\n", nama_projek));

    println!("\x1b[1;32mProjek Melayu '{}' berjaya dicipta!\x1b[0m", nama_projek);
    println!("Struktur fail:");
    println!("  {}/", nama_projek);
    println!("  ├── pakej.json      (Manifest metadata & pakej)");
    println!("  ├── utama.ml        (Fail kod permulaan)");
    println!("  ├── modul_melayu/   (Folder pakej & kebergantungan)");
    println!("  └── README.md");
    println!();
    println!("Langkah seterusnya:");
    if nama_opt.is_some() {
        println!("  cd {}", nama_projek);
    }
    println!("  melayu utama.ml");
}

fn pasang_pakej(sasaran_opt: Option<&str>) {
    let modul_dir = std::path::Path::new("modul_melayu");
    if !modul_dir.exists() {
        let _ = fs::create_dir_all(modul_dir);
    }

    match sasaran_opt {
        Some(sasaran) => {
            let src_path = std::path::Path::new(sasaran);
            let fail_nama = if let Some(n) = src_path.file_name() {
                n.to_string_lossy().to_string()
            } else {
                format!("{}.ml", sasaran)
            };

            let dest_nama = if fail_nama.ends_with(".ml") {
                fail_nama.clone()
            } else {
                format!("{}.ml", fail_nama)
            };

            let dest_path = modul_dir.join(&dest_nama);
            let nama_pakej = dest_nama.trim_end_matches(".ml").to_string();

            if src_path.exists() && src_path.is_file() {
                if let Err(e) = fs::copy(src_path, &dest_path) {
                    eprintln!("\x1b[1;31mAlamak! Gagal menyalin fail modul: {}\x1b[0m", e);
                    process::exit(1);
                }
                println!("\x1b[1;32mPakej tempatan '{}' berjaya dipasang ke {}!\x1b[0m", nama_pakej, dest_path.display());
            } else {
                let template = format!(
                    r#"# Modul: {}
# Boleh diimport dengan: ambil "{}"

kelas {}
    bina()
    selesai

    kerja sapa()
        tunjuk "Hai daripada modul {}!"
    selesai
selesai
"#,
                    nama_pakej, nama_pakej, nama_pakej, nama_pakej
                );
                let _ = fs::write(&dest_path, template);
                println!("\x1b[1;32mModul baharu '{}' berjaya dicipta di {}!\x1b[0m", nama_pakej, dest_path.display());
            }

            let pakej_json_path = std::path::Path::new("pakej.json");
            if pakej_json_path.exists() {
                if let Ok(kandungan) = fs::read_to_string(pakej_json_path) {
                    let mut teks = kandungan;
                    if !teks.contains(&format!("\"{}\"", nama_pakej)) {
                        teks = teks.replace(
                            "\"kebergantungan\": {",
                            &format!("\"kebergantungan\": {{\n    \"{}\": \"^1.0.0\",", nama_pakej),
                        );
                        let _ = fs::write(pakej_json_path, teks);
                    }
                }
            }
        }
        None => {
            println!("\x1b[1;32mMenyemak kebergantungan modul_melayu/...\x1b[0m");
            println!("Semua kebergantungan pakej tempatan siap sedia!");
        }
    }
}

fn senarai_pakej() {
    println!("\x1b[1;32mPENGURUS PAKEJ MELAYU (SENARAI MODUL)\x1b[0m");
    println!("{}", "-".repeat(50));
    println!("Modul Terbina-Dalam (Standard Frameworks):");
    println!("  \x1b[1;36m* akal\x1b[0m     - Kerangka Kerja Machine Learning & Deep Learning");
    println!("  \x1b[1;36m* selasih\x1b[0m  - Kerangka Kerja Web Server & REST API");
    println!();

    let modul_dir = std::path::Path::new("modul_melayu");
    println!("Pakej Terpasang Tempatan (modul_melayu/):");
    if modul_dir.exists() {
        let mut wujud = false;
        if let Ok(entries) = fs::read_dir(modul_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name != ".gitkeep" {
                    println!("  - {}", name);
                    wujud = true;
                }
            }
        }
        if !wujud {
            println!("  (Tiada pakej luaran terpasang lagi. Guna 'melayu pasang <nama>' untuk menambah)");
        }
    } else {
        println!("  (Folder modul_melayu/ belum wujud)");
    }
    println!("{}", "-".repeat(50));
}
