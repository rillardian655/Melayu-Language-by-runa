use melayu_core::analyzer::Analyzer;
use melayu_core::interpreter::Interpreter;
use melayu_core::lexer::Lexer;
use melayu_core::parser::Parser;
use melayu_core::repl::{cetak_ralat, run_repl};
use std::env;
use std::fs;
use std::process;

fn ambil_pilihan_had_langkah(args: &mut Vec<String>) {
    let Some(i) = args
        .iter()
        .position(|a| a == "--had-langkah" || a.starts_with("--had-langkah="))
    else {
        return;
    };
    let nilai = match args[i].strip_prefix("--had-langkah=") {
        Some(v) => v.to_string(),
        None if i + 1 < args.len() => args.remove(i + 1),
        None => {
            eprintln!("\x1b[1;31mAlamak! '--had-langkah' memerlukan nilai, contoh: --had-langkah 0\x1b[0m");
            process::exit(1);
        }
    };
    args.remove(i);
    if nilai != "0" && nilai != "tiada" && nilai.parse::<u64>().is_err() {
        eprintln!("\x1b[1;31mAlamak! Nilai '--had-langkah' mesti nombor bulat atau 0 (tiada had).\x1b[0m");
        process::exit(1);
    }
    env::set_var("MELAYU_HAD_LANGKAH", nilai);
}

const SAIZ_TINDANAN_UTAMA: usize = 256 * 1024 * 1024;

fn main() {
    let utas = std::thread::Builder::new()
        .name("melayu-utama".to_string())
        .stack_size(SAIZ_TINDANAN_UTAMA)
        .spawn(main_utama)
        .expect("gagal memulakan utas utama");
    if utas.join().is_err() {
        process::exit(101);
    }
}

fn main_utama() {
    let mut args: Vec<String> = env::args().collect();
    ambil_pilihan_had_langkah(&mut args);

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
            let mut nama = None;
            let mut is_mvc = false;
            for arg in args.iter().skip(2) {
                if arg == "--mvc" || arg == "--web" {
                    is_mvc = true;
                } else if nama.is_none() && !arg.starts_with('-') {
                    nama = Some(arg.as_str());
                }
            }
            mula_projek(nama, is_mvc);
        }
        "pasang" | "install" => {
            let sasaran = args.get(2).map(|s| s.as_str());
            pasang_pakej(sasaran);
        }
        "pakej" | "packages" | "senarai-pakej" => {
            senarai_pakej();
        }
        "taman" | "playground" => {
            let port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(7878);
            jalankan_taman_permainan(port);
        }
        "bantuan" | "--help" | "-h" => {
            cetak_bantuan();
        }
        "versi" | "--version" | "-v" => {
            println!("MELAYU v0.4.1");
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
    println!("MELAYU v0.4.1 — CLI");
    println!("\"Bahasa programming serumpun, untuk serumpun developer.\"");
    println!("\x1b[0m");
    println!("PENGGUNAAN:");
    println!("  melayu                             Buka REPL interaktif");
    println!("  melayu jalankan <fail.ml>          Jalankan fail atur cara Melayu");
    println!("  melayu <fail.ml>                   Jalankan fail secara terus");
    println!("  melayu semak <fail.ml>             Semak sintaks dan semantik fail");
    println!("  melayu uji <fail.ml>               Jalankan suite ujian unit");
    println!("  melayu tanda-aras <fail.ml> [n]    Jalankan penanda aras masa (benchmark)");
    println!("  melayu mula [nama_projek] [--mvc]  Cipta struktur projek Melayu (asas atau MVC)");
    println!("  melayu pasang [nama_atau_fail]     Pasang modul ke modul_melayu/");
    println!("  melayu pakej                       Senarai pakej dan framework tersedia");
    println!("  melayu taman [port]                Buka Web Playground interaktif (lalai: 7878)");
    println!("  melayu bantuan                     Papar bantuan ini");
    println!("  melayu versi                       Papar versi semasa");
    println!();
    println!("PILIHAN:");
    println!("  --had-langkah <n>                  Had langkah pelaksanaan (lalai: 10000000, 0 = tiada had)");
    println!("                                     Boleh juga guna pemboleh ubah persekitaran MELAYU_HAD_LANGKAH");
    println!();
    println!("CONTOH:");
    println!("  melayu mula aplikasi_saya");
    println!("  melayu mula web_kredit --mvc");
    println!("  melayu pasang alat_kira.ml");
    println!("  melayu jalankan utama.ml");
}

fn mula_projek(nama_opt: Option<&str>, is_mvc: bool) {
    let nama_projek = nama_opt.unwrap_or(if is_mvc { "aplikasi_web_melayu" } else { "projek_melayu" });
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

    let _ = fs::create_dir_all(&modul_dir);
    let _ = fs::write(modul_dir.join(".gitkeep"), "");
    let _ = fs::write(&gitignore_path, "target/\n*.tmp\n*.log\n");

    if is_mvc {
        let app_dir = sasaran_dir.join("app");
        let kawalan_dir = app_dir.join("kawalan");
        let model_dir = app_dir.join("model");
        let servis_dir = app_dir.join("perkhidmatan");
        let laluan_dir = sasaran_dir.join("laluan");
        let data_dir = sasaran_dir.join("data");

        let _ = fs::create_dir_all(&kawalan_dir);
        let _ = fs::create_dir_all(&model_dir);
        let _ = fs::create_dir_all(&servis_dir);
        let _ = fs::create_dir_all(&laluan_dir);
        let _ = fs::create_dir_all(&data_dir);

        let config_path = sasaran_dir.join("konfigurasi.json");
        let _ = fs::write(
            &config_path,
            format!(
                r#"{{
  "nama_aplikasi": "{}",
  "versi": "1.0.0",
  "port": 8080,
  "persekitaran": "pembangunan"
}}
"#,
                nama_projek
            ),
        );

        let data_csv_path = data_dir.join("sampel.csv");
        let _ = fs::write(
            &data_csv_path,
            "id,nama,kategori,nilai\n1,Projek Alfa,Teknologi,95.5\n2,Projek Beta,Kewangan,88.0\n3,Projek Gamma,Pendidikan,91.2\n",
        );

        let model_path = model_dir.join("model_item.ml");
        let _ = fs::write(
            &model_path,
            r#"# ==========================================================
# MODEL ITEM (DATA ENTITY)
# ==========================================================

kelas ModelItem
    bina(id, nama, status)
        diri.id = id
        diri.nama = nama
        diri.status = status
    selesai

    kerja keKamus()
        pulang {
            "id": diri.id,
            "nama": diri.nama,
            "status": diri.status
        }
    selesai
selesai
"#,
        );

        let servis_path = servis_dir.join("servis_item.ml");
        let _ = fs::write(
            &servis_path,
            r#"# ==========================================================
# SERVIS LOGIK PERNIAGAAN
# ==========================================================

kerja dapatkanSenaraiItem()
    pulang [
        {"id": 1, "nama": "Sistem Utama", "status": "aktif"},
        {"id": 2, "nama": "Enjin AI", "status": "sedia"},
        {"id": 3, "nama": "Pengkalan Data", "status": "beroperasi"}
    ]
selesai

kerja kiraStatistik()
    pulang {
        "jumlah_item": 3,
        "kecekapan": "99.8%",
        "status_sistem": "sihat"
    }
selesai
"#,
        );

        let kawalan_path = kawalan_dir.join("kawalan_utama.ml");
        let _ = fs::write(
            &kawalan_path,
            format!(
                r#"# ==========================================================
# PENGENDALI KAWALAN (CONTROLLER)
# ==========================================================
ambil "selasih"
ambil "app/perkhidmatan/servis_item.ml" sebagai Servis

kerja kendaliLamanUtama(permintaan)
    ada html = "<html><head><title>{}</title></head><body>" +
               "<h1>Selamat Datang ke {} (MVC Framework)</h1>" +
               "<p>Pelayan Web Selasih sedia berkhidmat!</p>" +
               "<ul><li>GET /api/items - Senarai data</li>" +
               "<li>GET /api/statistik - Status sistem</li></ul>" +
               "</body></html>"
    pulang balasHTML(html, 200)
selesai

kerja kendaliSenaraiItem(permintaan)
    ada senarai = Servis.dapatkanSenaraiItem()
    pulang balasJSON({{"kejayaan": betul, "data": senarai}}, 200)
selesai

kerja kendaliStatistik(permintaan)
    ada stat = Servis.kiraStatistik()
    pulang balasJSON({{"kejayaan": betul, "statistik": stat}}, 200)
selesai
"#,
                nama_projek, nama_projek
            ),
        );

        let laluan_path = laluan_dir.join("api.ml");
        let _ = fs::write(
            &laluan_path,
            r#"# ==========================================================
# PENDAFTARAN LALUAN (ROUTES)
# ==========================================================
ambil "app/kawalan/kawalan_utama.ml" sebagai Kawalan

kerja daftarLaluan(app)
    app.dapatkan("/", Kawalan.kendaliLamanUtama)
    app.dapatkan("/api/items", Kawalan.kendaliSenaraiItem)
    app.dapatkan("/api/statistik", Kawalan.kendaliStatistik)
selesai
"#,
        );

        let _ = fs::write(
            &utama_path,
            format!(
                r#"# ==========================================================
# TITIK MASUK UTAMA APLIKASI MVC ({})
# ==========================================================
ambil "selasih"
ambil "laluan/api.ml" sebagai Laluan

ada aplikasi = PelayanWeb(8080)
Laluan.daftarLaluan(aplikasi)

tunjuk "=========================================================="
tunjuk "  Aplikasi Web MVC '{}' sedia beroperasi!"
tunjuk "  Laman: http://localhost:8080"
tunjuk "  API  : http://localhost:8080/api/items"
tunjuk "=========================================================="
tunjuk "Tekan Ctrl+C untuk menghentikan pelayan."
aplikasi.dengar(0)
"#,
                nama_projek, nama_projek
            ),
        );

        let pakej_json_content = format!(
            r#"{{
  "nama": "{}",
  "versi": "1.0.0",
  "penerangan": "Aplikasi Web & API Berstruktur MVC Bahasa Melayu",
  "utama": "utama.ml",
  "corak": "MVC",
  "kebergantungan": {{
    "selasih": "terbina"
  }}
}}
"#,
            nama_projek
        );
        let _ = fs::write(&pakej_path, pakej_json_content);

        let _ = fs::write(
            &readme_path,
            format!(
                r#"# {} (Aplikasi MVC Bahasa Melayu)

Aplikasi web berstruktur MVC (Model-View-Controller) menggunakan kerangka kerja Selasih.

## Struktur Direktori
* `app/kawalan/` - Pengendali rute dan logik HTTP (Controllers)
* `app/model/` - Struktur data dan entiti aplikasi (Models)
* `app/perkhidmatan/` - Logik perniagaan dan pemprosesan data (Services)
* `laluan/` - Pendaftaran URL penghalaan (Routes)
* `data/` - Dataset dan fail storan tempatan
* `modul_melayu/` - Pakej luaran

## Cara Menjalankan
```bash
melayu utama.ml
```
"#,
                nama_projek
            ),
        );

        println!("\x1b[1;32mProjek Melayu MVC '{}' berjaya dicipta!\x1b[0m", nama_projek);
        println!("Struktur fail:");
        println!("  {}/", nama_projek);
        println!("  ├── pakej.json                  (Manifest metadata)");
        println!("  ├── konfigurasi.json            (Tetapan port & app)");
        println!("  ├── utama.ml                    (Titik masuk web server)");
        println!("  ├── app/");
        println!("  │   ├── kawalan/kawalan_utama.ml (Controller)");
        println!("  │   ├── model/model_item.ml     (Model)");
        println!("  │   └── perkhidmatan/servis_item.ml (Service)");
        println!("  ├── laluan/api.ml               (Router)");
        println!("  ├── data/sampel.csv             (Dataset)");
        println!("  ├── modul_melayu/               (Folder pakej)");
        println!("  └── README.md");
    } else {
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
        let _ = fs::write(
            &readme_path,
            format!("# {}\n\nProjek Bahasa Melayu.\n\n## Cara Menjalankan\n```bash\nmelayu utama.ml\n```\n", nama_projek),
        );

        println!("\x1b[1;32mProjek Melayu '{}' berjaya dicipta!\x1b[0m", nama_projek);
        println!("Struktur fail:");
        println!("  {}/", nama_projek);
        println!("  ├── pakej.json      (Manifest metadata & pakej)");
        println!("  ├── utama.ml        (Fail kod permulaan)");
        println!("  ├── modul_melayu/   (Folder pakej & kebergantungan)");
        println!("  └── README.md");
    }

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
            if sasaran.trim().is_empty() || sasaran == "." || sasaran == ".." || sasaran.contains('\0') {
                eprintln!("\x1b[1;31mAlamak! Nama modul '{}' tidak sah.\x1b[0m", sasaran);
                process::exit(1);
            }
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

fn jalankan_taman_permainan(port: u16) {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    const HTML_PLAYGROUND: &str = include_str!("../playground/index.html");
    let addr = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("\x1b[1;31mAlamak! Gagal membuka Taman Permainan pada {}: {}\x1b[0m", addr, e);
            process::exit(1);
        }
    };

    // Jana token CSRF rawak untuk sesi ini bagi menghalang serangan CSRF localhost
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let csrf_token = format!("{:x}-{:x}", nanos ^ 0x5a5a_f0f0_1234_9876, process::id());
    let html_with_token = HTML_PLAYGROUND.replace("__MELAYU_CSRF_TOKEN__", &csrf_token);

    println!("\x1b[1;36m==========================================================\x1b[0m");
    println!("\x1b[1;32m  Taman Permainan Web MELAYU & AkalML sedia!\x1b[0m");
    println!("  Buka di pelayar web: \x1b[1;33mhttp://{}\x1b[0m", addr);
    println!("\x1b[1;36m==========================================================\x1b[0m");
    println!("Tekan Ctrl+C untuk menutup pelayan.");

    for stream_res in listener.incoming() {
        let mut stream = match stream_res {
            Ok(s) => s,
            Err(_) => continue,
        };
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(5)));

        let mut buf = vec![0u8; 65536];
        let n = match stream.read(&mut buf) {
            Ok(n) if n > 0 => n,
            _ => continue,
        };
        let raw = String::from_utf8_lossy(&buf[..n]);
        let mut parts = raw.splitn(2, "\r\n\r\n");
        let headers = parts.next().unwrap_or("");
        let body = parts.next().unwrap_or("");
        let first_line = headers.lines().next().unwrap_or("");
        let mut tokens = first_line.split_whitespace();
        let method = tokens.next().unwrap_or("GET");
        let path = tokens.next().unwrap_or("/");

        if method == "GET" && path == "/" {
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{}",
                html_with_token.len(),
                html_with_token
            );
            let _ = stream.write_all(resp.as_bytes());
        } else if method == "POST" && path == "/api/jalankan" {
            // Sahkan token CSRF & pengepala Host bagi menghalang RCE & DNS Rebinding
            let mut token_sah = false;
            let mut host_sah = false;
            let expected_host1 = format!("127.0.0.1:{}", port);
            let expected_host2 = format!("localhost:{}", port);
            for line in headers.lines().skip(1) {
                if let Some((k, v)) = line.split_once(':') {
                    let k_trim = k.trim();
                    let v_trim = v.trim();
                    if k_trim.eq_ignore_ascii_case("x-melayu-token") && v_trim == csrf_token {
                        token_sah = true;
                    } else if k_trim.eq_ignore_ascii_case("host")
                        && (v_trim == expected_host1
                            || v_trim == expected_host2
                            || v_trim == "127.0.0.1"
                            || v_trim == "localhost")
                    {
                        host_sah = true;
                    }
                }
            }
            if !token_sah || !host_sah {
                let msg = "{\"kejayaan\":false,\"ralat\":\"Akses ditolak: Token keselamatan CSRF atau hos tidak sah.\"}";
                let resp = format!(
                    "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    msg.len(),
                    msg
                );
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.flush();
                continue;
            }

            let dummy_pos = melayu_core::ast::LineCol::new(1, 1);
            let kod_src = match melayu_core::builtins::builtin_baca_json(
                &[melayu_core::interpreter::Value::Teks(body.to_string())],
                dummy_pos,
            ) {
                Ok(melayu_core::interpreter::Value::Kamus(map)) => {
                    match map.borrow().get("kod") {
                        Some(melayu_core::interpreter::Value::Teks(s)) => s.clone(),
                        _ => String::new(),
                    }
                }
                _ => String::new(),
            };

            let tmp_dir = std::env::temp_dir().join("melayu_playground_sandbox");
            let _ = fs::create_dir_all(&tmp_dir);
            let tmp_path = tmp_dir.join(format!("melayu_pg_{}.ml", process::id()));
            let _ = fs::write(&tmp_path, &kod_src);

            let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("melayu"));
            // Jalankan dalam direktori sandbox dengan had masa maksimum 10 saat
            let (ok, text) = match process::Command::new(exe)
                .env_remove("MELAYU_HAD_LANGKAH")
                .current_dir(&tmp_dir)
                .arg(&tmp_path)
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    let start = std::time::Instant::now();
                    let mut timed_out = false;
                    loop {
                        match child.try_wait() {
                            Ok(Some(_)) => break,
                            Ok(None) => {
                                if start.elapsed() > std::time::Duration::from_secs(10) {
                                    let _ = child.kill();
                                    timed_out = true;
                                    break;
                                }
                                std::thread::sleep(std::time::Duration::from_millis(25));
                            }
                            Err(_) => break,
                        }
                    }
                    let _ = fs::remove_file(&tmp_path);
                    if timed_out {
                        (false, "Alamak! Pelaksanaan melebihi had masa keselamatan (10 saat) dan dihentikan.".to_string())
                    } else {
                        match child.wait_with_output() {
                            Ok(o) if o.status.success() => {
                                (true, String::from_utf8_lossy(&o.stdout).to_string())
                            }
                            Ok(o) => {
                                let err_txt = String::from_utf8_lossy(&o.stderr).to_string();
                                let out_txt = String::from_utf8_lossy(&o.stdout).to_string();
                                (false, format!("{}{}", out_txt, err_txt))
                            }
                            Err(e) => (false, format!("Gagal membaca keluaran proses: {}", e)),
                        }
                    }
                }
                Err(e) => {
                    let _ = fs::remove_file(&tmp_path);
                    (false, format!("Gagal menjalankan proses: {}", e))
                }
            };

            let mut resp_map = melayu_core::interpreter::PetaNilai::new();
            resp_map.insert(
                "kejayaan".to_string(),
                if ok {
                    melayu_core::interpreter::Value::Betul
                } else {
                    melayu_core::interpreter::Value::Salah
                },
            );
            resp_map.insert(
                if ok { "keluaran" } else { "ralat" }.to_string(),
                melayu_core::interpreter::Value::Teks(text),
            );
            let json_res = melayu_core::builtins::nilai_ke_json_string(
                &melayu_core::interpreter::Value::Kamus(std::rc::Rc::new(std::cell::RefCell::new(resp_map))),
                false,
                0,
                dummy_pos,
            )
            .unwrap_or_else(|_| "{\"kejayaan\":false,\"ralat\":\"Ralat JSON\"}".to_string());

            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                json_res.len(),
                json_res
            );
            let _ = stream.write_all(resp.as_bytes());
        } else {
            let msg = "404 Tidak Dijumpai";
            let resp = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                msg.len(),
                msg
            );
            let _ = stream.write_all(resp.as_bytes());
        }
        let _ = stream.flush();
    }
}
