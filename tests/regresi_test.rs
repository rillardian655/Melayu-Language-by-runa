use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static PENGIRA: AtomicUsize = AtomicUsize::new(0);

struct Folder(PathBuf);

impl Folder {
    fn baru() -> Self {
        let n = PENGIRA.fetch_add(1, Ordering::SeqCst);
        let p = std::env::temp_dir().join(format!("melayu_regresi_{}_{}", std::process::id(), n));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        Folder(p.canonicalize().unwrap())
    }

    fn tulis(&self, nama: &str, isi: &str) -> PathBuf {
        let p = self.0.join(nama);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, isi).unwrap();
        p
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn arahan(cwd: &Path, hujah: &[&str]) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_melayu"));
    c.current_dir(cwd)
        .args(hujah)
        .env_remove("MELAYU_HAD_LANGKAH")
        .env_remove("MELAYU_GPU")
        .env_remove("MELAYU_SANDBOX");
    c
}

fn keluaran(o: &Output) -> String {
    let mut s = String::from_utf8_lossy(&o.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&o.stderr));
    s
}

fn jalankan(kod: &str) -> (Option<i32>, String) {
    let f = Folder::baru();
    f.tulis("u.ml", kod);
    let o = arahan(&f.0, &["u.ml"]).output().unwrap();
    (o.status.code(), keluaran(&o))
}

fn jaya(kod: &str) -> String {
    let (kod_keluar, teks) = jalankan(kod);
    assert_eq!(kod_keluar, Some(0), "program gagal: {}", ringkas(&teks));
    teks.trim().to_string()
}

fn ralat_bersih(kod: &str) -> String {
    let (kod_keluar, teks) = jalankan(kod);
    assert_eq!(kod_keluar, Some(1), "patut ralat biasa, bukan ranap: {}", ringkas(&teks));
    teks
}

fn ringkas(s: &str) -> String {
    s.chars().take(600).collect()
}

#[test]
fn had_langkah_boleh_diubah() {
    let f = Folder::baru();
    f.tulis("g.ml", "ada i = 0\nselagi i < 3000\n    i = i + 1\nselesai\ntunjuk i\n");
    let o = arahan(&f.0, &["g.ml"]).env("MELAYU_HAD_LANGKAH", "1000").output().unwrap();
    assert_eq!(o.status.code(), Some(1));
    assert!(keluaran(&o).contains("(1000)"));
    for hujah in [vec!["--had-langkah", "0", "g.ml"], vec!["g.ml", "--had-langkah=0"], vec!["jalankan", "g.ml", "--had-langkah", "50000"]] {
        let o = arahan(&f.0, &hujah).env("MELAYU_HAD_LANGKAH", "1000").output().unwrap();
        assert_eq!(keluaran(&o).trim(), "3000", "{:?}", hujah);
    }
    let o = arahan(&f.0, &["--had-langkah", "abc", "g.ml"]).output().unwrap();
    assert_eq!(o.status.code(), Some(1));
}

#[test]
fn nilai_kitaran_dipapar_dan_dibanding_tanpa_ranap() {
    assert_eq!(jaya("ada a = [1, 2]\na.tambah(a)\ntunjuk a\n"), "[1, 2, [...]]");
    assert_eq!(jaya("ada a = [1]\na.tambah(a)\nada b = [1]\nb.tambah(b)\ntunjuk a == b\n"), "betul");
    assert_eq!(jaya("ada a = [1]\na.tambah(a)\nada b = [2]\nb.tambah(b)\ntunjuk a == b\n"), "salah");
    assert_eq!(jaya("ada k = kamus { \"x\": 1 }\nk[\"k\"] = k\ntunjuk k == k\n"), "betul");
    assert_eq!(jaya("ada s = [1]\ntunjuk [s, s]\n"), "[[1], [1]]");
}

#[test]
fn objek_yang_menyimpan_induk_boleh_dibanding() {
    let kod = "kelas A\n    bina()\n        diri.x = 1\n    selesai\nselesai\nkelas B warisi A\n    bina(n)\n        induk.bina()\n        diri.simpan = induk\n        diri.n = n\n    selesai\nselesai\ntunjuk B(1) == B(1)\ntunjuk B(1) == B(2)\n";
    assert_eq!(jaya(kod), "betul\nsalah");
}

#[test]
fn ungkapan_bersarang_tidak_meranapkan_proses() {
    for n in [1000usize, 3000, 20_000] {
        let (k, t) = jalankan(&format!("ada x = {}1{}\ntunjuk x\n", "(".repeat(n), ")".repeat(n)));
        assert!(k == Some(0) || k == Some(1), "kurungan {}: {:?} {}", n, k, ringkas(&t));
    }
    assert_eq!(jaya(&format!("ada x = 1{}\ntunjuk x\n", " + 1".repeat(5500))), "5501");
    for n in [20_000usize, 200_000] {
        ralat_bersih(&format!("ada x = 1{}\ntunjuk x\n", " + 1".repeat(n)));
        ralat_bersih(&format!("tunjuk {}1\n", "- ".repeat(n)));
        ralat_bersih(&format!("ada s = \"x\"\ntunjuk s{}\n", ".hurufBesar()".repeat(n)));
        ralat_bersih(&format!("ada x = {}{}\n", "[".repeat(n), "]".repeat(n)));
    }
}

#[test]
fn pernyataan_bersarang_tidak_meranapkan_proses() {
    assert_eq!(jaya(&format!("{}tunjuk 1\n{}", "jika betul\n".repeat(3000), "selesai\n".repeat(3000))), "1");
    for buka in ["jika betul\n", "selagi betul\n", "kerja f()\n", "cuba\n"] {
        ralat_bersih(&format!("{}tunjuk 1\n{}", buka.repeat(200_000), "selesai\n".repeat(200_000)));
    }
}

#[test]
fn rekursi_dengan_ungkapan_panjang_tidak_meranapkan_proses() {
    let kod = format!(
        "kerja f(n)\n    jika n == 0\n        pulang 0\n    selesai\n    pulang f(n - 1){}\nselesai\ntunjuk f(900)\n",
        " + 1".repeat(3000)
    );
    let (k, t) = jalankan(&kod);
    assert!(k == Some(0) || k == Some(1), "{:?} {}", k, ringkas(&t));
    assert_eq!(jaya("kerja f(n)\n    jika n == 0\n        pulang 0\n    selesai\n    pulang 1 + f(n - 1)\nselesai\ntunjuk f(990)\n"), "990");
}

#[test]
fn senarai_sangat_dalam_boleh_dipapar_dibanding_dan_dilepas() {
    let bina = "ada a = []\nada b = []\nuntuk i dari 1 hingga 1500000\n    a = [a]\n    b = [b]\nselesai\n";
    assert_eq!(jaya(&format!("{}ada p = panjang(teks(a))\ntunjuk p > 1000\n", bina)), "betul");
    assert_eq!(jaya(&format!("{}tunjuk a == b\nb = [b]\ntunjuk a == b\n", bina)), "betul\nsalah");
    assert_eq!(jaya("ada a = []\nuntuk i dari 1 hingga 5000000\n    a = [a]\nselesai\na = 0\ntunjuk \"ok\"\n"), "ok");
    assert_eq!(jaya("ada k = {}\nuntuk i dari 1 hingga 1500000\n    k = {\"k\": k}\nselesai\nk = 0\ntunjuk \"ok\"\n"), "ok");
}

#[cfg(unix)]
#[test]
fn sandbox_menolak_semua_laluan_keluar() {
    use std::os::unix::fs::symlink;
    let f = Folder::baru();
    let luar = f.0.join("luar");
    let proj = f.0.join("proj");
    fs::create_dir_all(&luar).unwrap();
    fs::create_dir_all(proj.join(".GIT")).unwrap();
    fs::write(luar.join("rahsia.txt"), "RAHSIA").unwrap();
    fs::write(luar.join("x.ml"), "kerja k()\n    pulang \"KOD LUAR\"\nselesai\n").unwrap();
    fs::write(proj.join(".env"), "RAHSIA=1").unwrap();
    fs::write(proj.join(".GIT/config"), "x").unwrap();
    fs::write(proj.join("biasa.txt"), "BIASA").unwrap();
    symlink("../luar", proj.join("pautdir")).unwrap();
    symlink("../luar/rahsia.txt", proj.join("pautfail.txt")).unwrap();
    symlink("../luar/baru1.txt", proj.join("rosak.txt")).unwrap();
    symlink(".env", proj.join("nota")).unwrap();
    symlink("biasa.txt", proj.join("pautdalam.txt")).unwrap();
    symlink("../luar/x.ml", proj.join("pautmodul.ml")).unwrap();
    fs::create_dir_all(proj.join("modul_melayu")).unwrap();
    symlink("../../luar/x.ml", proj.join("modul_melayu/pautpakej.ml")).unwrap();

    let abs = |r: &str| proj.join(r).display().to_string();
    let luar_abs = luar.join("baru2.txt").display().to_string();
    let mut baca = vec!["../luar/rahsia.txt".to_string(), "pautfail.txt".into(), "pautdir/rahsia.txt".into(), "nota".into(), ".env".into(), ".GIT/config".into(), ".gIt/config".into()];
    baca.extend([abs("pautfail.txt"), abs("pautdir/rahsia.txt"), abs("nota"), luar.join("rahsia.txt").display().to_string()]);
    let mut tulis = vec!["rosak.txt".to_string(), "pautdir/baru3.txt".into(), "pautfail.txt".into(), ".env".into(), ".env.local".into()];
    tulis.extend([abs("rosak.txt"), abs("pautdir/baru4.txt"), abs("pautdir/sub/baru5.txt"), luar_abs]);

    let mut kod = String::new();
    for p in &baca {
        kod.push_str(&format!("cuba\n    ada t = bacaFail(\"{}\")\n    tunjuk \"BOCOR BACA {}\"\ntangkap e\n    ada q = 1\nselesai\n", p, p));
    }
    for p in &tulis {
        kod.push_str(&format!("cuba\n    tulisFail(\"{}\", \"x\")\n    tunjuk \"BOCOR TULIS {}\"\ntangkap e\n    ada q = 1\nselesai\n", p, p));
    }
    kod.push_str(&format!("cuba\n    padamFail(\"{}\")\n    tunjuk \"BOCOR PADAM\"\ntangkap e\n    ada q = 1\nselesai\n", abs("pautdir/rahsia.txt")));
    kod.push_str("tunjuk bacaFail(\"biasa.txt\")\ntunjuk bacaFail(\"pautdalam.txt\")\ntulisFail(\"baru.txt\", \"1\")\ntunjuk wujudFail(\"baru.txt\")\n");
    kod.push_str(&format!("tulisFail(\"{}\", \"2\")\ntunjuk bacaFail(\"baru.txt\")\n", abs("baru.txt")));
    fs::write(proj.join("u.ml"), kod).unwrap();
    let o = arahan(&proj, &["u.ml"]).output().unwrap();
    assert_eq!(keluaran(&o).trim(), "BIASA\nBIASA\nbetul\n2");
    let mut baki: Vec<String> = fs::read_dir(&luar).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
    baki.sort();
    assert_eq!(baki, ["rahsia.txt", "x.ml"]);

    for modul in ["pautmodul", "pautmodul.ml", "pautpakej", "pautdir/x", "pautdir/x.ml"] {
        fs::write(proj.join("m.ml"), format!("ambil \"{}\" sebagai M\ntunjuk M.k()\n", modul)).unwrap();
        let o = arahan(&proj, &["m.ml"]).output().unwrap();
        assert_eq!(o.status.code(), Some(1), "modul luar dimuat: {}", modul);
        assert!(!keluaran(&o).contains("KOD LUAR"), "{}", modul);
    }
}

#[test]
fn modul_biasa_dan_modul_melayu_masih_dimuat() {
    let f = Folder::baru();
    f.tulis("mm.ml", "kerja k()\n    pulang \"mm\"\nselesai\n");
    f.tulis("modul_melayu/pakej.ml", "kerja p()\n    pulang \"pakej\"\nselesai\n");
    f.tulis("modul_melayu/besar/mod.ml", "kerja q()\n    pulang \"besar\"\nselesai\n");
    f.tulis("u.ml", "ambil \"mm\"\nambil \"mm.ml\" sebagai M\nambil p daripada \"pakej\"\nambil \"besar\"\nambil \"akal\"\ntunjuk k() + M.k() + p() + q()\n");
    let o = arahan(&f.0, &["u.ml"]).output().unwrap();
    assert_eq!(keluaran(&o).trim(), "mmmmpakejbesar");
}

#[test]
fn alamat_metadata_disekat_dalam_semua_bentuk() {
    for hos in [
        "169.254.169.254", "2852039166", "0xa9fea9fe", "169.254.43518", "0", "0.0.0.0", "100.100.100.200", "metadata.google.internal",
        "[::ffff:169.254.169.254]", "[::169.254.169.254]", "[64:ff9b::a9fe:a9fe]", "[64:ff9b:1::a9fe:a9fe]", "[64:ff9b:1:a9fe:a9:fe00::]",
        "[2002:a9fe:a9fe::]", "[fe80::1]", "[fd00:ec2::254]", "[::]",
    ] {
        let t = ralat_bersih(&format!("hantarPermintaanHTTP(\"GET\", \"http://{}/\")\n", hos));
        assert!(t.contains("Anti-SSRF"), "{} tidak disekat: {}", hos, ringkas(&t));
    }
}

#[test]
fn localhost_masih_boleh_dihubungi() {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    let h = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut s, _) = l.accept().unwrap();
            let mut b = [0u8; 2048];
            let _ = s.read(&mut b);
            s.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok").unwrap();
        }
    });
    let t = jaya(&format!(
        "tunjuk hantarPermintaanHTTP(\"GET\", \"http://127.0.0.1:{p}/\")[\"badan\"]\ntunjuk hantarPermintaanHTTP(\"GET\", \"http://localhost:{p}/\")[\"status\"]\n",
        p = port
    ));
    assert_eq!(t, "ok\n200");
    h.join().unwrap();
}

#[test]
fn susunan_kamus_ikut_urutan_masuk() {
    let kod = "ada k = kamus { \"c\": 3, \"a\": 1, \"b\": 2 }\nk[\"d\"] = 4\nk[\"a\"] = 10\nk.buang(\"c\")\nk[\"c\"] = 5\ntunjuk k.kunci()\ntunjuk k.nilai()\ntunjuk tulisJSON(k)\ntunjuk k == kamus { \"c\": 5, \"b\": 2, \"a\": 10, \"d\": 4 }\ntunjuk tulisJSON(bacaJSON(\"{\\\"z\\\": 1, \\\"y\\\": {\\\"q\\\": 2, \\\"p\\\": 3}}\"))\n";
    for _ in 0..3 {
        assert_eq!(jaya(kod), "[\"a\", \"b\", \"d\", \"c\"]\n[10, 2, 4, 5]\n{\"a\":10,\"b\":2,\"d\":4,\"c\":5}\nbetul\n{\"z\":1,\"y\":{\"q\":2,\"p\":3}}");
    }
}

#[test]
fn darab_matriks_tepat_dan_tidak_melimpah() {
    let kod = "ada b = []\nada i = 0\nselagi i < 65\n    ada r = []\n    ada j = 0\n    selagi j < 65\n        r.tambah(12345)\n        j = j + 1\n    selesai\n    b.tambah(r)\n    i = i + 1\nselesai\nada A = tensor(b)\ntunjuk (A @ A).keSenarai()[0][0] == 9905936625\nada B = satuTensor([100, 100]) * 100000000000000000000\ntunjuk (B @ B).keSenarai()[0][0] > 900000000000000000000000000000000000000000\n";
    assert_eq!(jaya(kod), "betul\nbetul");
    let f = Folder::baru();
    f.tulis("u.ml", kod);
    let o = arahan(&f.0, &["u.ml"]).env("MELAYU_GPU", "0").output().unwrap();
    assert_eq!(keluaran(&o).trim(), "betul\nbetul");
    let o = arahan(&f.0, &["u.ml"]).env("MELAYU_GPU", "fp32").output().unwrap();
    assert!(keluaran(&o).trim().ends_with("betul"), "fp32 melimpah patut balik ke CPU");
}

#[test]
fn hasil_darab_terlalu_besar_memberi_ralat() {
    let t = ralat_bersih("ada C = satuTensor([1000000, 1]) @ satuTensor([1, 1000000])\n");
    assert!(t.contains("Had penggunaan memori"), "{}", ringkas(&t));
    let t = ralat_bersih("ada a = []\nada b = [[]]\nuntuk i dari 1 hingga 100000\n    a.tambah([1])\n    b[0].tambah(1)\nselesai\nada c = darabMatriks(a, b)\n");
    assert!(t.contains("Had penggunaan memori"), "{}", ringkas(&t));
}

#[test]
fn notasi_saintifik_dan_ralat_baris_panjang() {
    assert_eq!(jaya("tunjuk 1e3\ntunjuk 5.9e-05\ntunjuk 1.5E+2\nada e = 3\ntunjuk 2 * e\n"), "1000\n0.000059\n150\n6");
    let t = ralat_bersih(&format!("ada x = {}tak_wujud\n", "1 + ".repeat(3000)));
    assert!(t.contains("tak_wujud"));
    let t = ralat_bersih(&format!("ada s = \"{}\" + tak_wujud\n", "a".repeat(100_000)));
    assert!(t.contains("tak_wujud"));
}

#[test]
fn isih_hanya_jenis_seragam() {
    assert_eq!(jaya("tunjuk [10, 9, 100, -3.5].isih()\ntunjuk [\"b\", \"a\"].isih()\ntunjuk [].isih()\ntunjuk [[1, 2]].isih()\ntunjuk [kosong].isih()\n"), "[-3.5, 9, 10, 100]\n[\"a\", \"b\"]\n[]\n[[1, 2]]\n[kosong]");
    assert!(ralat_bersih("tunjuk [3, kosong, 1].isih()\n").contains("bercampur") || ralat_bersih("tunjuk [3, kosong, 1].isih()\n").contains("mengandungi"));
}

#[test]
fn ralat_dalam_modul_menunjuk_fail_yang_betul() {
    let f = Folder::baru();
    f.tulis("m.ml", "# satu\n# dua\nkerja bagi(x)\n    pulang x / 0\nselesai\nkerja panggil(g)\n    pulang g()\nselesai\n");
    f.tulis("lempar.ml", "baling \"KOD_1\"\n");
    f.tulis("rosak.ml", "ada y = (1 +\n");
    f.tulis("a.ml", "ambil \"m.ml\"\n\n\ntunjuk \"baris empat\"\nbagi(1)\n");
    let t = keluaran(&arahan(&f.0, &["a.ml"]).output().unwrap());
    assert!(t.contains("pulang x / 0") && t.contains("m.ml") && !t.contains("  tunjuk \"baris empat\""), "{}", t);
    f.tulis("b.ml", "ambil \"m.ml\"\nkerja utama()\n    pulang tak_ada[0]\nselesai\nada tak_ada = 5\npanggil(utama)\n");
    let t = keluaran(&arahan(&f.0, &["b.ml"]).output().unwrap());
    assert!(t.contains("pulang tak_ada[0]") && !t.contains("m.ml"), "{}", t);
    f.tulis("c.ml", "cuba\n    ambil \"lempar.ml\"\ntangkap e\n    tunjuk e == \"KOD_1\"\nselesai\n");
    assert_eq!(keluaran(&arahan(&f.0, &["c.ml"]).output().unwrap()).trim(), "betul");
    f.tulis("d.ml", "ambil \"rosak.ml\" sebagai R\n");
    let t = keluaran(&arahan(&f.0, &["d.ml"]).output().unwrap());
    assert!(t.contains("rosak.ml"), "{}", t);
}

#[cfg(unix)]
#[test]
fn bermula_walau_ruang_alamat_terhad() {
    let f = Folder::baru();
    f.tulis("u.ml", "tunjuk \"hidup\"\n");
    let o = Command::new("sh")
        .current_dir(&f.0)
        .args(["-c", "ulimit -v 200000; exec \"$0\" u.ml", env!("CARGO_BIN_EXE_melayu")])
        .output()
        .unwrap();
    assert_eq!(keluaran(&o).trim(), "hidup");
}

#[test]
fn templat_mvc_boleh_melayan_permintaan() {
    let f = Folder::baru();
    assert!(arahan(&f.0, &["mula", "p", "--mvc"]).output().unwrap().status.success());
    let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let utama = f.0.join("p/utama.ml");
    let isi = fs::read_to_string(&utama).unwrap();
    fs::write(&utama, isi.replace("PelayanWeb(8080)", &format!("PelayanWeb({})", port)).replace("dengar(0)", "dengar(3)")).unwrap();
    let anak = arahan(&f.0.join("p"), &["utama.ml"]).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    for laluan in ["/", "/api/items", "/api/statistik"] {
        let mut jawapan = String::new();
        for _ in 0..50 {
            if let Ok(mut s) = std::net::TcpStream::connect(("127.0.0.1", port)) {
                s.write_all(format!("GET {} HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n", laluan).as_bytes()).unwrap();
                let _ = s.read_to_string(&mut jawapan);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(jawapan.starts_with("HTTP/1.1 200"), "{} -> {}", laluan, ringkas(&jawapan));
    }
    let o = anak.wait_with_output().unwrap();
    assert!(o.status.success(), "{}", keluaran(&o));
}

#[test]
fn benih_rawak_boleh_diulang() {
    let kod = "benihRawak(42)\nada a = [rawak(1, 100), rawak(), teks(rawakTensor([2, 2]))]\nbenihRawak(42)\nada b = [rawak(1, 100), rawak(), teks(rawakTensor([2, 2]))]\nbenihRawak(43)\nada c = [rawak(1, 100), rawak(), teks(rawakTensor([2, 2]))]\ntunjuk a == b\ntunjuk a == c\n";
    assert_eq!(jaya(kod), "betul\nsalah");
}

#[test]
fn pembahagian_data_dikocok_tanpa_kehilangan_baris() {
    let kod = "benihRawak(1)\nada X = []\nada y = []\nuntuk i dari 0 hingga 99\n    X.tambah([i])\n    y.tambah(i * 2)\nselesai\nada d = bahagiLatihUji(X, y, 0.2)\nada semua = []\nada sepadan = betul\nuntuk b dari 0 hingga 1\n    ada n = panjang(d[b])\n    untuk i dari 0 hingga n - 1\n        semua.tambah(d[b][i][0])\n        jika d[b + 2][i] != d[b][i][0] * 2\n            sepadan = salah\n        selesai\n    selesai\nselesai\ntunjuk panjang(d[1])\ntunjuk sepadan\ntunjuk d[1][0][0] == 80 dan d[1][1][0] == 81 dan d[1][2][0] == 82\nsemua.isih()\ntunjuk semua[0] == 0 dan semua[99] == 99 dan panjang(semua) == 100\ntunjuk X[0][0]\n";
    assert_eq!(jaya(kod), "20\nbetul\nsalah\nbetul\n0");
}

#[test]
fn kfold_iris_dan_saiz_dataset() {
    let kod = "ambil \"akal\"\nbenihRawak(1)\nada iris = muatIris()\nkerja b()\n    pulang PengelasKNN(5)\nselesai\nada k = pengesahanSilang(b, iris[\"ciri\"], iris[\"sasaran\"], 3)\ntunjuk panjang(iris[\"ciri\"])\ntunjuk k[\"purata_skor\"] > 0.9\n";
    assert_eq!(jaya(kod), "150\nbetul");
}

#[test]
fn pecahan_pokok_kes_tepi() {
    let kod = "ambil \"akal\"\nada m = PokokKeputusan(3)\nm.latih([[1.0000000000000002], [1.0000000000000004], [1.0000000000000002], [1.0000000000000004]], [0, 1, 0, 1])\ntunjuk m.ramal([[1.0000000000000002], [1.0000000000000004]])\nm.latih([[1.0], [1.0], [1.0]], [0, 1, 1])\ntunjuk m.ramal([[1.0]])\nada p = cariPecahanTerbaik([[0, 1], [0, 2], [0, 9]], [0, 0, 1], [0, 1], 1)\ntunjuk p[\"ciri\"]\ntunjuk p[\"ambang\"]\ntunjuk cariPecahanTerbaik([[1], [1]], [0, 1], [0])\nada inf = 1e308 * 10\nada q = cariPecahanTerbaik([[1], [2], [inf]], [0, 0, 1], [0])\ntunjuk q[\"ambang\"]\n";
    assert_eq!(jaya(kod), "[0, 1]\n[1]\n1\n5.5\nkosong\n2");
}

#[test]
fn hutan_dan_bayes_pada_data_jauh() {
    let kod = "ambil \"akal\"\nada nb = PengelasBayesNaif()\nnb.latih([[0.0, 0.0], [0.1, 0.1], [10.0, 0.0], [10.1, 0.1]], [\"a\", \"a\", \"b\", \"b\"])\ntunjuk nb.ramal([[4.0, 0.05], [6.5, 0.05], [1000.0, 0.0], [-1000.0, 0.0]])\nbenihRawak(3)\nada X = []\nada y = []\nuntuk i dari 0 hingga 59\n    ada c = i % 2\n    X.tambah([0, 0, 0, 0, 0, 0, 0, 0, c * 5 + rawak()])\n    y.tambah(c)\nselesai\nada h = HutanRawak(15, 3)\nh.latih(X, y)\ntunjuk h.skor(X, y)\n";
    assert_eq!(jaya(kod), "[\"a\", \"b\", \"b\", \"a\"]\n1");
}

#[test]
fn fungsi_dan_tensor_yang_dikongsi_tidak_bocor() {
    let kod = "kerja buat()\n    ada n = 0\n    kerja t()\n        n = n + 1\n        pulang n\n    selesai\n    pulang t\nselesai\nada a = buat()\nada b = buat()\na()\na()\ntunjuk a()\ntunjuk b()\nada A = satuTensor([2, 2])\nada B = A\nB = B + 1\ntunjuk A.jumlah()\ntunjuk B.jumlah()\n";
    assert_eq!(jaya(kod), "3\n1\n4\n8");
}

fn pelayan(jawapan: Vec<Vec<u8>>) -> (u16, std::thread::JoinHandle<Vec<String>>) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    let h = std::thread::spawn(move || {
        let mut diterima = Vec::new();
        for j in jawapan {
            let (mut s, _) = l.accept().unwrap();
            let mut b = [0u8; 4096];
            let n = s.read(&mut b).unwrap_or(0);
            diterima.push(String::from_utf8_lossy(&b[..n]).to_string());
            let _ = s.write_all(&j);
        }
        diterima
    });
    (port, h)
}

#[test]
fn respons_bercebisan_dinyahkod() {
    let mut a = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: Chunked\r\n\r\n".to_vec();
    for c in ["{\"n\": \"", "\u{e9}", "\", \"x\": 42}"] {
        a.extend(format!("{:X};ext=1\r\n", c.len()).bytes());
        a.extend(c.bytes());
        a.extend(b"\r\n");
    }
    a.extend(b"0\r\n\r\n");
    let (port, h) = pelayan(vec![a]);
    let t = jaya(&format!("ada r = hantarPermintaanHTTP(\"GET\", \"http://127.0.0.1:{}/\")\ntunjuk r[\"badan\"]\ntunjuk bacaJSON(r[\"badan\"])[\"x\"]\n", port));
    assert_eq!(t, "{\"n\": \"\u{e9}\", \"x\": 42}\n42");
    h.join().unwrap();
}

#[test]
fn respons_terputus_memberi_ralat() {
    let kes: Vec<&[u8]> = vec![
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nabc",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhel",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\nx",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nzz\r\nx",
    ];
    for j in kes {
        let (port, h) = pelayan(vec![j.to_vec()]);
        let t = ralat_bersih(&format!("hantarPermintaanHTTP(\"GET\", \"http://127.0.0.1:{}/\")\n", port));
        assert!(t.contains("terputus"), "{}", ringkas(&t));
        h.join().unwrap();
    }
    for (kaedah, j) in [("HEAD", &b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n"[..]), ("GET", b"HTTP/1.1 204 No Content\r\n\r\n"), ("GET", b"HTTP/1.1 200 OK\r\n\r\nabc")] {
        let (port, h) = pelayan(vec![j.to_vec()]);
        jaya(&format!("hantarPermintaanHTTP(\"{}\", \"http://127.0.0.1:{}/\")\n", kaedah, port));
        h.join().unwrap();
    }
}

#[test]
fn pengepala_tersuai_dihantar_dan_disahkan() {
    let ok = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok".to_vec();
    let (port, h) = pelayan(vec![ok.clone(), ok]);
    let u = format!("http://127.0.0.1:{}/", port);
    jaya(&format!("hantarPermintaanHTTP(\"POST\", \"{}\", \"isi\", {{\"X-Kunci\": \"abc\", \"Content-Type\": \"text/plain\"}})\nhantarPermintaanHTTP(\"GET\", \"{}\")\n", u, u));
    let d = h.join().unwrap();
    assert!(d[0].contains("X-Kunci: abc\r\n") && d[0].contains("Content-Type: text/plain\r\n") && !d[0].contains("application/json") && d[0].ends_with("\r\n\r\nisi"), "{}", d[0]);
    assert!(d[1].contains("Content-Type: application/json\r\n"), "{}", d[1]);
    for p in ["{\"A\": \"x\\r\\nB: y\"}", "{\"A\\nB\": \"y\"}", "{\"A: b\": \"y\"}", "{\"Host\": \"lain\"}", "{\"content-length\": \"1\"}", "5"] {
        ralat_bersih(&format!("hantarPermintaanHTTP(\"GET\", \"http://127.0.0.1:9/\", \"\", {})\n", p));
    }
}

#[test]
fn https_menolak_pelayan_bukan_tls_dan_metadata() {
    let (port, h) = pelayan(vec![b"HTTP/1.1 200 OK\r\n\r\nbukan tls".to_vec()]);
    let t = ralat_bersih(&format!("hantarPermintaanHTTP(\"GET\", \"https://127.0.0.1:{}/\")\n", port));
    if cfg!(feature = "https") {
        assert!(t.contains("TLS"), "{}", ringkas(&t));
        h.join().unwrap();
        for u in ["https://169.254.169.254/", "HTTPS://2852039166/", "https://[64:ff9b::a9fe:a9fe]/"] {
            assert!(ralat_bersih(&format!("hantarPermintaanHTTP(\"GET\", \"{}\")\n", u)).contains("Anti-SSRF"));
        }
    } else {
        assert!(t.contains("HTTPS tidak tersedia"), "{}", ringkas(&t));
    }
}
