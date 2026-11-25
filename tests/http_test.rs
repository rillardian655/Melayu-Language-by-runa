mod bantuan;

use bantuan::*;
use std::io::{Read, Write};
use std::net::TcpListener;

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
