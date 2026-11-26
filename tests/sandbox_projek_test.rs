mod bantuan;

use bantuan::*;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

#[test]
fn akar_sandbox_ialah_folder_projek() {
    let f = Folder::baru();
    f.tulis("rahsia.txt", "RAHSIA");
    f.tulis("p/pakej.json", "{}");
    f.tulis("p/data/x.txt", "DATA");
    f.tulis("p/mm.ml", "kerja k()\n    pulang \"mm\"\nselesai\n");
    f.tulis("p/modul_melayu/salam.ml", "kerja s()\n    pulang \"projek\"\nselesai\n");
    f.tulis("modul_melayu/salam.ml", "kerja s()\n    pulang \"TERTANAM\"\nselesai\n");
    f.tulis("mm.ml", "kerja k()\n    pulang \"TERTANAM\"\nselesai\n");
    let kod = "ambil \"mm.ml\"\nambil \"salam\"\ntunjuk k() + s() + bacaFail(\"data/x.txt\")\ntulisFail(\"baru.txt\", \"1\")\ncuba\n    tunjuk bacaFail(\"rahsia.txt\")\ntangkap e\n    tunjuk \"sekat\"\nselesai\n";
    f.tulis("p/u.ml", kod);
    f.tulis("p/src/u.ml", kod);
    for (cwd, skrip) in [("p", "u.ml"), (".", "p/u.ml"), ("p/src", "u.ml"), ("p/src", "../u.ml"), (".", "p/src/u.ml")] {
        for arah in [vec![skrip], vec!["jalankan", skrip]] {
            let o = arahan(&f.0.join(cwd), &arah).output().unwrap();
            assert_eq!(keluaran(&o).trim(), "mmprojekDATA\nsekat", "{} {:?}", cwd, arah);
        }
        let o = arahan(&f.0.join(cwd), &["semak", skrip]).output().unwrap();
        assert!(o.status.success(), "{}", keluaran(&o));
        assert!(!f.0.join("baru.txt").exists());
    }
}

#[test]
fn tanpa_pakej_akar_ialah_folder_skrip() {
    let f = Folder::baru();
    f.tulis("rahsia.txt", "RAHSIA");
    f.tulis("s/d.txt", "D");
    f.tulis("s/u.ml", "tunjuk bacaFail(\"d.txt\")\ncuba\n    tunjuk bacaFail(\"rahsia.txt\")\ntangkap e\n    tunjuk \"sekat\"\nselesai\n");
    let o = arahan(&f.0, &["s/u.ml"]).output().unwrap();
    assert_eq!(keluaran(&o).trim(), "D\nsekat");
    f.tulis("s/v.ml", "tunjuk bacaFail(\"rahsia.txt\")\n");
    for hujah in [vec!["--sandbox", ".", "s/v.ml"], vec!["s/v.ml", "--sandbox=."]] {
        assert_eq!(keluaran(&arahan(&f.0, &hujah).output().unwrap()).trim(), "RAHSIA");
    }
    let o = arahan(&f.0, &["s/v.ml"]).env("MELAYU_SANDBOX", &f.0).output().unwrap();
    assert_eq!(keluaran(&o).trim(), "RAHSIA");
    for (hujah, env) in [(vec!["--sandbox", "tiada", "s/v.ml"], ""), (vec!["s/v.ml"], "tiada")] {
        let o = arahan(&f.0, &hujah).env("MELAYU_SANDBOX", env).output().unwrap();
        assert_eq!(o.status.code(), Some(1));
        assert!(keluaran(&o).contains("tidak wujud"), "{}", keluaran(&o));
    }
}

#[test]
fn pakej_json_di_folder_rumah_tidak_meluaskan_sandbox() {
    let f = Folder::baru();
    f.tulis("pakej.json", "{}");
    f.tulis("rahsia.txt", "RAHSIA");
    f.tulis("muat_turun/u.ml", "cuba\n    tunjuk bacaFail(\"rahsia.txt\")\ntangkap e\n    tunjuk \"sekat\"\nselesai\n");
    let o = arahan(&f.0.join("muat_turun"), &["u.ml"]).env("HOME", &f.0).env("USERPROFILE", &f.0).output().unwrap();
    assert_eq!(keluaran(&o).trim(), "sekat");
}

#[test]
fn taman_permainan_kekal_dalam_folder_sementara() {
    let f = Folder::baru();
    let tmp = f.0.join("tmp");
    fs::create_dir_all(&tmp).unwrap();
    fs::write(tmp.join("pakej.json"), "{}").unwrap();
    fs::write(tmp.join("rahsia.txt"), "RAHSIA").unwrap();
    let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let mut anak = arahan(&f.0, &["taman", &port.to_string()]).env("TMPDIR", &tmp).env("TEMP", &tmp).env("TMP", &tmp).stdout(std::process::Stdio::null()).spawn().unwrap();
    let hantar = |permintaan: String| -> String {
        for _ in 0..50 {
            if let Ok(mut s) = std::net::TcpStream::connect(("127.0.0.1", port)) {
                s.write_all(permintaan.as_bytes()).unwrap();
                let mut t = String::new();
                let _ = s.read_to_string(&mut t);
                return t;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        String::new()
    };
    let laman = hantar("GET / HTTP/1.1\r\nHost: x\r\n\r\n".to_string());
    let token = laman.split("X-Melayu-Token': '").nth(1).and_then(|s| s.split('\'').next()).unwrap_or("").to_string();
    let jalan = |kod: &str| {
        let badan = format!("{{\"kod\": \"{}\"}}", kod);
        hantar(format!("POST /api/jalankan HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nX-Melayu-Token: {}\r\nContent-Length: {}\r\n\r\n{}", port, token, badan.len(), badan))
    };
    let a = jalan("tulisFail(\\\"a.txt\\\", \\\"hai\\\")\\ntunjuk bacaFail(\\\"a.txt\\\")");
    let b = jalan("tunjuk bacaFail(\\\"rahsia.txt\\\")");
    let _ = anak.kill();
    let _ = anak.wait();
    assert!(a.contains("\"kejayaan\":true") && a.contains("hai"), "{}", ringkas(&a));
    assert!(b.contains("\"kejayaan\":false") && !b.contains("RAHSIA"), "{}", ringkas(&b));
}
