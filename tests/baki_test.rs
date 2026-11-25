mod bantuan;

use bantuan::*;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Stdio};
use std::time::{Duration, Instant};

fn port_bebas() -> u16 {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port()
}

fn mula(f: &Folder, kod: &str, port: u16) -> Child {
    f.tulis("s.ml", kod);
    let anak = arahan(&f.0, &["s.ml"]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    for _ in 0..100 {
        std::thread::sleep(Duration::from_millis(30));
        if TcpListener::bind(("127.0.0.1", port)).is_err() {
            return anak;
        }
    }
    panic!("pelayan tidak bermula");
}

fn minta(port: u16, mentah: &[u8]) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
    s.write_all(mentah).unwrap();
    let mut b = Vec::new();
    let _ = s.read_to_end(&mut b);
    String::from_utf8_lossy(&b).to_string()
}

fn get(port: u16, laluan: &str) -> String {
    minta(port, format!("GET {} HTTP/1.1\r\nHost: x\r\n\r\n", laluan).as_bytes())
}

const GEMA: &str = "kerja k(req)\n    pulang {\"status\": 200, \"jenisKandungan\": \"text/plain\", \"badan\": req[\"kaedah\"] + \"|\" + req[\"laluan\"] + \"|\" + req[\"badan\"]}\nselesai\n";

#[test]
fn pelayan_tidak_ranap_pada_permintaan_pelik() {
    let f = Folder::baru();
    let port = port_bebas();
    let mut anak = mula(&f, &format!("{}mulakanPelayanHTTP({}, k, 0)\n", GEMA, port), port);
    for n in 0..7 {
        let t = minta(port, format!("POST /a HTTP/1.1\r\nContent-Length: {}\r\n\r\n\u{e9}\u{e9}\u{e9}", n).as_bytes());
        assert!(t.starts_with("HTTP/1.1 200"), "{} -> {}", n, t);
    }
    assert!(minta(port, "POST /a HTTP/1.1\r\nContent-Length: 6\r\n\r\n\u{e9}\u{e9}\u{e9}".as_bytes()).ends_with("POST|/a|\u{e9}\u{e9}\u{e9}"));
    assert!(minta(port, b"POST /a HTTP/1.1\r\nContent-Length: 3\r\n\r\nabcdef").ends_with("POST|/a|abc"));
    let pelik: Vec<Vec<u8>> = vec![
        b"\r\n\r\n".to_vec(),
        b"GET".to_vec(),
        vec![0xff; 100],
        b"GET / HTTP/1.1\r\nContent-Length: -5\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nContent-Length: 99999999999999999999999\r\n\r\n".to_vec(),
        b"POST / HTTP/1.1\r\nContent-Length: 2000000\r\n\r\n".to_vec(),
        [b"GET / HTTP/1.1\r\nX: ".to_vec(), vec![b'a'; 40_000]].concat(),
        b"GET / HTTP/1.1\r\n: kosong\r\nTiadaTitikBertindih\r\n\r\n".to_vec(),
    ];
    for p in pelik {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let _ = s.write_all(&p);
        let _ = s.shutdown(std::net::Shutdown::Write);
        s.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
        let _ = s.read_to_end(&mut Vec::new());
    }
    assert!(get(port, "/hidup").ends_with("GET|/hidup|"));
    assert!(anak.try_wait().unwrap().is_none());
    let _ = anak.kill();
    let _ = anak.wait();
}

#[test]
fn klien_perlahan_tidak_menyekat_klien_lain() {
    let f = Folder::baru();
    let port = port_bebas();
    let mut anak = mula(&f, &format!("{}mulakanPelayanHTTP({}, k, 0)\n", GEMA, port), port);
    let perlahan: Vec<TcpStream> = (0..20)
        .map(|_| {
            let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
            s.write_all(b"GET / HTTP/1.1\r\n").unwrap();
            s
        })
        .collect();
    let t = Instant::now();
    for i in 0..10 {
        assert!(get(port, &format!("/{}", i)).ends_with(&format!("GET|/{}|", i)));
    }
    assert!(t.elapsed() < Duration::from_secs(2), "{:?}", t.elapsed());
    drop(perlahan);
    let _ = anak.kill();
    let _ = anak.wait();
}

#[test]
fn pelayan_mengira_keadaan_dan_berhenti_pada_had() {
    let f = Folder::baru();
    let port = port_bebas();
    let kod = format!(
        "ada n = 0\nkerja k(req)\n    n = n + 1\n    pulang teks(n)\nselesai\nada jumlah = mulakanPelayanHTTP({p}, k, 40)\ntunjuk jumlah\ntunjuk n\nada lagi = mulakanPelayanHTTP({p}, k, 1)\ntunjuk n\n",
        p = port
    );
    let anak = mula(&f, &kod, port);
    let utas: Vec<_> = (0..40).map(|_| std::thread::spawn(move || get(port, "/"))).collect();
    let mut nilai: Vec<u32> = utas
        .into_iter()
        .map(|h| h.join().unwrap().rsplit("\r\n\r\n").next().unwrap().parse().unwrap())
        .collect();
    nilai.sort();
    assert_eq!(nilai, (1..=40).collect::<Vec<u32>>());
    for _ in 0..100 {
        if let Ok(mut s) = TcpStream::connect(("127.0.0.1", port)) {
            let _ = s.write_all(b"GET / HTTP/1.1\r\n\r\n");
            let mut t = String::new();
            let _ = s.read_to_string(&mut t);
            if t.ends_with("41") {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let o = anak.wait_with_output().unwrap();
    assert_eq!(keluaran(&o).trim(), "40\n40\n41");
}

#[test]
fn ralat_pengendali_memberi_500_dan_mesej_sebenar() {
    let f = Folder::baru();
    let port = port_bebas();
    let kod = format!("ambil \"selasih\"\nada app = PelayanWeb({})\nkerja rosak(req)\n    pulang 1 / 0\nselesai\napp.dapatkan(\"/r\", rosak)\napp.dengar(2)\n", port);
    let anak = mula(&f, &kod, port);
    assert!(get(port, "/r").starts_with("HTTP/1.1 500"));
    assert!(get(port, "/tiada").starts_with("HTTP/1.1 404"));
    let o = anak.wait_with_output().unwrap();
    assert!(keluaran(&o).contains("Pembahagian dengan kosong"), "{}", keluaran(&o));
}

#[test]
fn port_tidak_sah_memberi_ralat() {
    for p in ["70000", "-1", "80.5"] {
        ralat_bersih(&format!("{}mulakanPelayanHTTP({}, k, 1)\n", GEMA, p));
    }
}

#[test]
fn nilai_dikongsi_berulang_kali_memberi_ralat_bukan_tergantung() {
    let bina = "ada a = [1]\nuntuk i dari 1 hingga 60\n    a = [a, a]\nselesai\nada k = {\"x\": 1}\nuntuk i dari 1 hingga 60\n    k = {\"a\": k, \"b\": k}\nselesai\n";
    for guna in ["tunjuk a", "tunjuk teks(a)", "tunjuk \"x\" + a", "tunjuk [a].cantum(\",\")", "tunjuk tulisJSON(a)", "tunjuk tulisJSON(a, betul)", "tunjuk k", "tunjuk tulisJSON(k)", "baling a", "ada z = {}\nz[a] = 1", "tunjuk \"abc\".mengandungi(a)"] {
        let t = Instant::now();
        let (k, r) = jalankan_terhad(&format!("{}{}\n", bina, guna));
        assert_eq!(k, Some(1), "{} -> {}", guna, ringkas(&r));
        assert!(r.contains("Had penggunaan memori"), "{} -> {}", guna, ringkas(&r));
        assert!(t.elapsed() < Duration::from_secs(20), "{} {:?}", guna, t.elapsed());
    }
    assert_eq!(jaya(&format!("{}ada b = [1]\nuntuk i dari 1 hingga 60\n    b = [b, b]\nselesai\ntunjuk a == b\n", bina)), "betul");
    assert_eq!(jaya("ada a = []\nuntuk i dari 1 hingga 300000\n    a.tambah(\"baris teks yang agak panjang\")\nselesai\ntunjuk panjang(teks(a)) > 9000000\ntunjuk panjang(tulisJSON(a)) > 9000000\n"), "betul\nbetul");
}

#[cfg(unix)]
#[test]
fn perlumbaan_pautan_tidak_terlepas_dari_sandbox() {
    use std::os::unix::fs::symlink;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let f = Folder::baru();
    let luar = f.0.join("luar");
    let proj = f.0.join("proj");
    fs::create_dir_all(&luar).unwrap();
    fs::create_dir_all(proj.join("nyata")).unwrap();
    fs::write(luar.join("r.txt"), "RAHSIA").unwrap();
    fs::write(luar.join("m.ml"), "kerja k()\n    pulang \"RAHSIA\"\nselesai\n").unwrap();
    fs::write(proj.join("nyata/r.txt"), "biasa").unwrap();
    fs::write(proj.join("nyata/m.ml"), "kerja k()\n    pulang \"biasa\"\nselesai\n").unwrap();
    fs::write(
        proj.join("u.ml"),
        "ada bocor = 0\nada i = 0\nselagi i < 6000\n    cuba\n        jika bacaFail(\"d/r.txt\") == \"RAHSIA\"\n            bocor = bocor + 1\n        selesai\n    tangkap e\n        ada q = 1\n    selesai\n    cuba\n        tulisFail(\"d/w.txt\", \"x\")\n    tangkap e\n        ada q = 1\n    selesai\n    cuba\n        padamFail(\"d/p.txt\")\n    tangkap e\n        ada q = 1\n    selesai\n    i = i + 1\nselesai\ntunjuk bocor\n",
    )
    .unwrap();
    let henti = Arc::new(AtomicBool::new(false));
    let (h2, p2, l2) = (henti.clone(), proj.clone(), luar.clone());
    let penukar = std::thread::spawn(move || {
        let d = p2.join("d");
        while !h2.load(Ordering::Relaxed) {
            let _ = fs::rename(p2.join("nyata"), &d);
            let _ = fs::rename(&d, p2.join("nyata"));
            let _ = symlink(&l2, &d);
            let _ = fs::write(l2.join("p.txt"), "jangan padam");
            let _ = fs::remove_file(&d);
        }
    });
    let o = arahan(&proj, &["--had-langkah", "0", "u.ml"]).output().unwrap();
    henti.store(true, Ordering::Relaxed);
    penukar.join().unwrap();
    assert_eq!(keluaran(&o).trim(), "0", "bacaan terlepas");
    assert!(!luar.join("w.txt").exists(), "tulisan terlepas");
}

#[cfg(unix)]
#[test]
fn fail_bukan_biasa_tidak_menggantung() {
    let f = Folder::baru();
    assert!(std::process::Command::new("mkfifo").arg(f.0.join("paip")).status().unwrap().success());
    f.tulis("biasa.txt", "lama yang panjang");
    f.tulis("u.ml", "cuba\n    bacaFail(\"paip\")\ntangkap e\n    tunjuk \"1\"\nselesai\ncuba\n    tulisFail(\"paip\", \"x\")\ntangkap e\n    tunjuk \"2\"\nselesai\ncuba\n    bacaCSV(\"paip\")\ntangkap e\n    tunjuk \"3\"\nselesai\ncuba\n    ambil \"paip\" sebagai P\ntangkap e\n    tunjuk \"4\"\nselesai\ntulisFail(\"biasa.txt\", \"baru\")\ntunjuk bacaFail(\"biasa.txt\")\npadamFail(\"biasa.txt\")\ntunjuk wujudFail(\"biasa.txt\")\ntulisFail(\"baharu.txt\", \"ok\")\ntunjuk bacaFail(\"baharu.txt\")\n");
    let mut anak = arahan(&f.0, &["u.ml"]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let t = Instant::now();
    while anak.try_wait().unwrap().is_none() {
        if t.elapsed() > Duration::from_secs(10) {
            let _ = anak.kill();
            panic!("tergantung pada FIFO");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    assert_eq!(keluaran(&anak.wait_with_output().unwrap()).trim(), "1\n2\n3\n4\nbaru\nsalah\nok");
}

#[test]
fn skop_dan_penutupan_kekal_betul() {
    let kod = "ada x = 1\njika betul\n    x = 2\n    ada y = 10\n    jika betul\n        ada y = 20\n        x = x + y\n    selesai\n    x = x + y\nselesai\ntunjuk x\nada fs = []\nuntuk i dari 1 hingga 3\n    kerja f()\n        pulang i\n    selesai\n    fs.tambah(f)\nselesai\ntunjuk fs[0]() + fs[1]() * 10 + fs[2]() * 100\nkerja luar(a)\n    ada jumlah = a\n    selagi a > 0\n        ada sementara = a\n        jumlah = jumlah + sementara\n        a = a - 1\n    selesai\n    pulang jumlah\nselesai\ntunjuk luar(4)\nada b1 = 1\nada b2 = 2\nada b3 = 3\nada b4 = 4\nada b5 = 5\nada b6 = 6\nada b7 = 7\nada b8 = 8\nada b9 = 9\nada b10 = 10\nkerja banyak()\n    ada c1 = 1\n    ada c2 = 2\n    ada c3 = 3\n    ada c4 = 4\n    ada c5 = 5\n    ada c6 = 6\n    ada c7 = 7\n    ada c8 = 8\n    ada c9 = 9\n    ada c10 = 10\n    c10 = c10 + b10\n    c1 = 100\n    pulang c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8 + c9 + c10\nselesai\ntunjuk banyak()\ncuba\n    b1 = tak_wujud\ntangkap e\n    tunjuk \"e\"\nselesai\n";
    let kod = kod.replace("cuba\n    b1 = tak_wujud\ntangkap e\n    tunjuk \"e\"\nselesai\n", "");
    assert_eq!(jaya(&kod), "32\n321\n14\n164");
}

#[test]
fn fungsi_dalam_sifat_boleh_mengubah_objeknya() {
    let kod = "kelas A\n    bina()\n        diri.x = 1\n        diri.f = kosong\n    selesai\n    kerja m()\n        diri.x = diri.x + 1\n        pulang diri.f()\n    selesai\nselesai\nada a = A()\nkerja g()\n    a.x = a.x + 40\n    pulang a.x\nselesai\na.f = g\ntunjuk a.f()\ntunjuk a.m()\nada k = {}\nkerja h()\n    k[\"x\"] = 42\n    k.buang(\"f\")\n    pulang k[\"x\"]\nselesai\nk[\"f\"] = h\ntunjuk k.f()\nada s = [1, 2, 3]\nkerja p(x)\n    s.tambah(x)\n    pulang x\nselesai\ntunjuk s.petakan(p)\ntunjuk panjang(s)\n";
    assert_eq!(jaya(kod), "41\n82\n42\n[1, 2, 3]\n6");
}

#[test]
fn aliran_gelung_dan_pulang_bersarang() {
    let kod = "kerja f(n)\n    ada i = 0\n    selagi betul\n        i = i + 1\n        jika i == 2\n            teruskan\n        selesai\n        untuk j dari 1 hingga 5\n            jika j == 3\n                hentikan\n            selesai\n            jika i * j == n\n                pulang [i, j]\n            selesai\n        selesai\n        jika i > 10\n            hentikan\n        jika tidak jika i == 7\n            ada z = i\n            i = z + 1\n        jika tidak\n            i = i\n        selesai\n    selesai\n    pulang \"tiada\"\nselesai\ntunjuk f(6)\ntunjuk f(4)\ntunjuk f(100)\nkerja kosongan()\n    jika betul\n        pulang\n    selesai\nselesai\ntunjuk kosongan()\n";
    assert_eq!(jaya(kod), "[3, 2]\n[4, 1]\ntiada\nkosong");
}
