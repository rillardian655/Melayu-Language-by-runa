#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static PENGIRA: AtomicUsize = AtomicUsize::new(0);

pub struct Folder(pub PathBuf);

impl Folder {
    pub fn baru() -> Self {
        let n = PENGIRA.fetch_add(1, Ordering::SeqCst);
        let p = std::env::temp_dir().join(format!("melayu_regresi_{}_{}", std::process::id(), n));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        Folder(p.canonicalize().unwrap())
    }

    pub fn tulis(&self, nama: &str, isi: &str) -> PathBuf {
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

pub fn arahan(cwd: &Path, hujah: &[&str]) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_melayu"));
    c.current_dir(cwd)
        .args(hujah)
        .env_remove("MELAYU_HAD_LANGKAH")
        .env_remove("MELAYU_GPU")
        .env_remove("MELAYU_SANDBOX");
    c
}

pub fn jalankan_terhad(kod: &str) -> (Option<i32>, String) {
    let f = Folder::baru();
    f.tulis("u.ml", kod);
    if !cfg!(target_os = "linux") {
        let o = arahan(&f.0, &["u.ml"]).output().unwrap();
        return (o.status.code(), keluaran(&o));
    }
    let o = Command::new("sh")
        .current_dir(&f.0)
        .args(["-c", "ulimit -v 2000000; ulimit -t 30; exec \"$0\" u.ml", env!("CARGO_BIN_EXE_melayu")])
        .output()
        .unwrap();
    (o.status.code(), keluaran(&o))
}

pub fn keluaran(o: &Output) -> String {
    let mut s = String::from_utf8_lossy(&o.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&o.stderr));
    s
}

pub fn jalankan(kod: &str) -> (Option<i32>, String) {
    let f = Folder::baru();
    f.tulis("u.ml", kod);
    let o = arahan(&f.0, &["u.ml"]).output().unwrap();
    (o.status.code(), keluaran(&o))
}

pub fn jaya(kod: &str) -> String {
    let (kod_keluar, teks) = jalankan(kod);
    assert_eq!(kod_keluar, Some(0), "program gagal: {}", ringkas(&teks));
    teks.trim().to_string()
}

pub fn ralat_bersih(kod: &str) -> String {
    let (kod_keluar, teks) = jalankan(kod);
    assert_eq!(kod_keluar, Some(1), "patut ralat biasa, bukan ranap: {}", ringkas(&teks));
    teks
}

pub fn ringkas(s: &str) -> String {
    s.chars().take(600).collect()
}
