use crate::ast::LineCol;
use crate::errors::{ErrorKind, MelayuError};
use crate::fail_selamat;
use crate::interpreter::{Interpreter, PetaNilai, SenaraiNilai, Value, HAD_PAPARAN};
use std::cell::RefCell;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub type BuiltinFn = fn(&[Value], LineCol) -> Result<Value, MelayuError>;

pub fn daftar_fungsi_bawaan() -> Vec<(&'static str, BuiltinFn)> {
    vec![
        ("panjang", builtin_panjang),
        ("jenis", builtin_jenis),
        ("hurufBesar", builtin_huruf_besar),
        ("hurufKecil", builtin_huruf_kecil),
        ("puncaKuasaDua", builtin_punca_kuasa_dua),
        ("teks", builtin_ke_teks),
        ("nombor", builtin_ke_nombor),
        ("rawak", builtin_rawak),
        ("bacaFail", builtin_baca_fail),
        ("tulisFail", builtin_tulis_fail),
        ("wujudFail", builtin_wujud_fail),
        ("padamFail", builtin_padam_fail),
        ("bundar", builtin_bundar),
        ("lantai", builtin_lantai),
        ("siling", builtin_siling),
        ("mutlak", builtin_mutlak),
        ("maksimum", builtin_maksimum),
        ("minimum", builtin_minimum),
        ("bacaJSON", builtin_baca_json),
        ("dariJSON", builtin_baca_json),
        ("tulisJSON", builtin_tulis_json),
        ("keJSON", builtin_tulis_json),
        ("capMasa", builtin_cap_masa),
        ("masaKini", builtin_masa_kini),
        ("tarikhKini", builtin_tarikh_kini),
        // AkalML: Asas Matematik & Pembelajaran Mesin Native
        ("darabMatriks", builtin_darab_matriks),
        ("transposisiMatriks", builtin_transposisi_matriks),
        ("songsangMatriks", builtin_songsang_matriks),
        ("titikDarab", builtin_titik_darab),
        ("jarakEuclidean", builtin_jarak_euclidean),
        ("bacaCSV", builtin_baca_csv),
        ("bahagiLatihUji", builtin_bahagi_latih_uji),
        ("purata", builtin_purata),
        ("sisihanPiawai", builtin_sisihan_piawai),
        ("skorMSE", builtin_skor_mse),
        ("skorR2", builtin_skor_r2),
        ("skorAkurasi", builtin_skor_akurasi),
        // AkalML Mid-High: Deep Learning & Statistik Lanjutan
        ("eksponen", builtin_eksponen),
        ("logaritmaAsli", builtin_logaritma_asli),
        ("fungsiSigmoid", builtin_fungsi_sigmoid),
        ("fungsiReLU", builtin_fungsi_relu),
        ("darabHadamard", builtin_darab_hadamard),
        ("matriksTambah", builtin_matriks_tambah),
        ("matriksTolak", builtin_matriks_tolak),
        ("matriksSkala", builtin_matriks_skala),
        ("matriksJumlahLajur", builtin_matriks_jumlah_lajur),
        ("matriksSigmoid", builtin_matriks_sigmoid),
        ("matriksDerivatifSigmoid", builtin_matriks_derivatif_sigmoid),
        ("matriksKekeliruan", builtin_matriks_kekeliruan),
        ("skorF1", builtin_skor_f1),
        ("kejituan", builtin_kejituan),
        ("kepekaan", builtin_kepekaan),
        ("matriksSoftmax", builtin_matriks_softmax),
        ("matriksReLU", builtin_matriks_relu),
        ("ralatEntropiSilang", builtin_ralat_entropi_silang),
        // Primitif Tensor Native
        ("tensor", builtin_tensor),
        ("sifarTensor", builtin_sifar_tensor),
        ("satuTensor", builtin_satu_tensor),
        ("rawakTensor", builtin_rawak_tensor),
        ("benihRawak", builtin_benih_rawak),
        ("cariPecahanTerbaik", builtin_cari_pecahan_terbaik),
        ("infoPerkakas", builtin_info_perkakas),
        // Kerangka Kerja Web & Rangkaian (Selasih)
        ("mulakanPelayanHTTP", builtin_mulakan_pelayan_http),
        ("hantarPermintaanHTTP", builtin_hantar_permintaan_http),
    ]
}

pub fn builtin_panjang(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'panjang' memerlukan 1 hujah, tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion("Contoh: panjang(senarai) atau panjang(\"teks\")"));
    }

    match &args[0] {
        Value::Senarai(list) => Ok(Value::Nombor(list.borrow().len() as f64)),
        Value::Teks(s) => Ok(Value::Nombor(s.chars().count() as f64)),
        Value::Kamus(map) => Ok(Value::Nombor(map.borrow().len() as f64)),
        other => Err(MelayuError::jenis_salah(
            "senarai, teks, atau kamus",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_jenis(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'jenis' memerlukan 1 hujah, tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Objek(o) => Ok(Value::Teks(o.nama_kelas.clone())),
        other => Ok(Value::Teks(other.jenis().to_string())),
    }
}

pub fn builtin_huruf_besar(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'hurufBesar' memerlukan 1 hujah teks.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Teks(s) => Ok(Value::Teks(s.to_uppercase())),
        other => Err(MelayuError::jenis_salah(
            "teks",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_huruf_kecil(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'hurufKecil' memerlukan 1 hujah teks.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Teks(s) => Ok(Value::Teks(s.to_lowercase())),
        other => Err(MelayuError::jenis_salah(
            "teks",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_punca_kuasa_dua(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'puncaKuasaDua' memerlukan 1 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => {
            if *n < 0.0 {
                return Err(MelayuError::new(
                    "Aduh!",
                    "Tidak boleh mengira punca kuasa dua bagi nombor negatif.",
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime));
            }
            Ok(Value::Nombor(n.sqrt()))
        }
        other => Err(MelayuError::jenis_salah(
            "nombor",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_bundar(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'bundar' memerlukan 1 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.round())),
        other => Err(MelayuError::jenis_salah(
            "nombor",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_lantai(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'lantai' memerlukan 1 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.floor())),
        other => Err(MelayuError::jenis_salah(
            "nombor",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_siling(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'siling' memerlukan 1 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.ceil())),
        other => Err(MelayuError::jenis_salah(
            "nombor",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_mutlak(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'mutlak' memerlukan 1 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.abs())),
        other => Err(MelayuError::jenis_salah(
            "nombor",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn builtin_maksimum(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() < 2 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'maksimum' memerlukan sekurang-kurangnya 2 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let mut max_val = match &args[0] {
        Value::Nombor(n) => *n,
        other => {
            return Err(MelayuError::jenis_salah(
                "nombor",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };
    for arg in &args[1..] {
        match arg {
            Value::Nombor(n) => {
                if *n > max_val {
                    max_val = *n;
                }
            }
            other => {
                return Err(MelayuError::jenis_salah(
                    "nombor",
                    other.jenis(),
                    pos.line,
                    pos.col,
                ))
            }
        }
    }
    Ok(Value::Nombor(max_val))
}

pub fn builtin_minimum(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() < 2 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'minimum' memerlukan sekurang-kurangnya 2 hujah nombor.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let mut min_val = match &args[0] {
        Value::Nombor(n) => *n,
        other => {
            return Err(MelayuError::jenis_salah(
                "nombor",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };
    for arg in &args[1..] {
        match arg {
            Value::Nombor(n) => {
                if *n < min_val {
                    min_val = *n;
                }
            }
            other => {
                return Err(MelayuError::jenis_salah(
                    "nombor",
                    other.jenis(),
                    pos.line,
                    pos.col,
                ))
            }
        }
    }
    Ok(Value::Nombor(min_val))
}

pub fn builtin_ke_teks(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'teks' memerlukan 1 hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    args[0].ke_teks(pos).map(Value::Teks)
}

pub fn builtin_ke_nombor(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'nombor' memerlukan 1 hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(*n)),
        Value::Teks(s) => match s.trim().parse::<f64>() {
            Ok(n) => Ok(Value::Nombor(n)),
            Err(_) => Err(MelayuError::new(
                "Aduh!",
                &format!("Rentetan \"{}\" tidak dapat ditukar menjadi nombor sah.", s),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime)
            .with_suggestion(
                "Pastikan rentetan mengandungi angka sah seperti \"42\" atau \"3.14\".",
            )),
        },
        other => Err(MelayuError::jenis_salah(
            "nombor atau teks angka",
            other.jenis(),
            pos.line,
            pos.col,
        )),
    }
}

pub fn lakukan_baca_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;
    // Buang \r dan \n
    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }
    Ok(buffer)
}

// -----------------------------------------------------------------------------
// PENGGILING RAWAK (PRNG) & KESELAMATAN SANDBOX FAIL
// -----------------------------------------------------------------------------

static PRNG_STATE: AtomicU64 = AtomicU64::new(0);

fn jana_nombor_rawak_u64() -> u64 {
    let mut current = PRNG_STATE.load(Ordering::Relaxed);
    if current == 0 {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(123456789);
        current = seed ^ (std::process::id() as u64).rotate_left(32);
        if current == 0 {
            current = 0x853c49e6748fea9b;
        }
    }
    // SplitMix64 step
    let z = current.wrapping_add(0x9e3779b97f4a7c15);
    PRNG_STATE.store(z, Ordering::Relaxed);
    let mut v = z;
    v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    v ^ (v >> 31)
}

pub fn builtin_benih_rawak(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let benih = match args {
        [Value::Nombor(n)] if n.is_finite() => *n,
        _ => {
            return Err(MelayuError::new(
                "Alamak!",
                "Fungsi 'benihRawak' memerlukan 1 hujah nombor, contoh: benihRawak(42).",
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime))
        }
    };
    let keadaan = (benih as i64 as u64) ^ 0x5851_f42d_4c95_7f2d;
    PRNG_STATE.store(if keadaan == 0 { 0x853c_49e6_748f_ea9b } else { keadaan }, Ordering::Relaxed);
    Ok(Value::Kosong)
}

pub fn builtin_rawak(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    match args.len() {
        0 => {
            let r = (jana_nombor_rawak_u64() as f64) / (u64::MAX as f64);
            Ok(Value::Nombor(r))
        }
        2 => {
            let min = match args[0] {
                Value::Nombor(n) => n,
                ref other => {
                    return Err(MelayuError::jenis_salah(
                        "nombor",
                        other.jenis(),
                        pos.line,
                        pos.col,
                    ))
                }
            };
            let max = match args[1] {
                Value::Nombor(n) => n,
                ref other => {
                    return Err(MelayuError::jenis_salah(
                        "nombor",
                        other.jenis(),
                        pos.line,
                        pos.col,
                    ))
                }
            };
            if !min.is_finite() || !max.is_finite() {
                return Err(MelayuError::new(
                    "Aduh!",
                    "Nilai minimum dan maksimum untuk 'rawak(min, max)' mestilah nombor terhingga.",
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime));
            }
            if min > max {
                return Err(MelayuError::new(
                    "Aduh!",
                    &format!(
                        "Nilai minimum ({}) tidak boleh lebih besar daripada nilai maksimum ({}).",
                        min, max
                    ),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime));
            }
            let range = (max - min + 1.0).floor();
            let r = (jana_nombor_rawak_u64() as f64) / (u64::MAX as f64);
            let val = (min + (r * range).floor()).min(max);
            Ok(Value::Nombor(val))
        }
        _ => Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'rawak' menerima 0 atau 2 hujah (min, max), tetapi menerima {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion(
            "Contoh: rawak() untuk 0.0-1.0 atau rawak(1, 100) untuk nombor rawak antara 1 dan 100.",
        )),
    }
}

pub const HAD_SAIZ_FAIL_MAKSIMUM: u64 = 5 * 1024 * 1024; // 5 MB
pub const HAD_ELEMEN_TENSOR_MAKSIMUM: usize = 1_000_000; // 1 juta elemen (~8 MB)

const NAMA_SENSITIF: [&str; 12] = [
    ".git", ".env", ".ssh", ".gemini", ".aws", ".gnupg", ".kube", ".docker", ".netrc", ".npmrc", ".pypirc", ".git-credentials",
];

fn nama_sensitif(laluan: &Path) -> Option<String> {
    laluan.components().find_map(|bahagian| match bahagian {
        Component::Normal(nama) => {
            let nama = nama.to_string_lossy().to_lowercase();
            let disekat = NAMA_SENSITIF.contains(&nama.as_str()) || nama.starts_with(".env.");
            disekat.then_some(nama)
        }
        _ => None,
    })
}

static AKAR_SANDBOX: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

pub fn tetapkan_akar_sandbox(akar: PathBuf) {
    let _ = AKAR_SANDBOX.set(akar);
}

fn akar_sandbox() -> io::Result<PathBuf> {
    match AKAR_SANDBOX.get() {
        Some(akar) => Ok(akar.clone()),
        None => std::env::current_dir()?.canonicalize(),
    }
}

pub fn semak_laluan_selamat(laluan_str: &str, pos: LineCol) -> Result<PathBuf, MelayuError> {
    let ralat = |sebab: &str| MelayuError::akses_fail_disekat(laluan_str, sebab, pos.line, pos.col);
    if laluan_str.trim().is_empty() {
        return Err(ralat("Nama laluan fail tidak boleh kosong."));
    }
    if laluan_str.contains('\0') {
        return Err(ralat("Aksara batal (null byte) dikesan dalam laluan fail."));
    }

    let p = Path::new(laluan_str);
    if p.components().any(|bahagian| matches!(bahagian, Component::ParentDir)) {
        return Err(ralat("Komponen laluan '..' dilarang demi keselamatan sandbox persekitaran fail."));
    }

    let akar = akar_sandbox()
        .map_err(|e| ralat(&format!("Gagal mendapatkan folder sandbox: {}", e)))?;
    let penuh = akar.join(p);
    if let Some(nama) = nama_sensitif(penuh.strip_prefix(&akar).unwrap_or(p)) {
        return Err(ralat(&format!(
            "Akses ke fail/direktori sensitif '{}' disekat demi keselamatan.",
            nama
        )));
    }

    let mut wujud = penuh;
    let mut baki = Vec::new();
    while fs::symlink_metadata(&wujud).is_err() {
        match wujud.file_name().map(|n| n.to_os_string()) {
            Some(nama) => baki.push(nama),
            None => return Err(ralat("Laluan fail tidak sah.")),
        }
        wujud.pop();
    }
    let mut sebenar = wujud
        .canonicalize()
        .map_err(|_| ralat("Pautan simbolik yang rosak (sasaran tidak wujud) tidak dibenarkan."))?;
    sebenar.extend(baki.into_iter().rev());

    match sebenar.strip_prefix(&akar) {
        Err(_) => Err(ralat(&format!(
            "Laluan ini berada di luar sandbox projek ({}).",
            akar.display()
        ))),
        Ok(dalam) => match nama_sensitif(dalam) {
            Some(nama) => Err(ralat(&format!(
                "Akses ke fail/direktori sensitif '{}' disekat demi keselamatan.",
                nama
            ))),
            None => Ok(sebenar),
        },
    }
}

fn baca_teks_terhad(laluan: &Path, nama: &str, pos: LineCol) -> Result<String, MelayuError> {
    let ralat = |sebab: String| MelayuError::akses_fail_disekat(nama, &sebab, pos.line, pos.col);
    let isi = akar_sandbox()
        .and_then(|akar| fail_selamat::baca(&akar, laluan, HAD_SAIZ_FAIL_MAKSIMUM))
        .map_err(|e| ralat(format!("Gagal membaca fail: {}", e)))?;
    String::from_utf8(isi).map_err(|_| ralat("Kandungan fail bukan teks UTF-8 yang sah.".to_string()))
}

fn modul_terbina(nama: &str) -> Option<&'static str> {
    match nama {
        "akal" | "akal_ml" | "akalml" | "std/akal" | "std/akal_ml" => Some(AKAL_ML_STDLIB),
        "selasih" | "wau" | "web" | "std/selasih" | "std/wau" => Some(SELASIH_STDLIB),
        "pangkalan" | "db" | "std/pangkalan" | "std/db" => Some(PANGKALAN_STDLIB),
        _ => None,
    }
}

pub fn baca_modul(sasaran: &str, pos: LineCol) -> Result<String, MelayuError> {
    let mut calon = vec![sasaran.to_string()];
    if !sasaran.ends_with(".ml") {
        calon.push(format!("{}.ml", sasaran));
        calon.push(Path::new(sasaran).with_extension("ml").to_string_lossy().to_string());
    }
    for nama in &calon {
        let laluan = semak_laluan_selamat(nama, pos)?;
        if laluan.is_file() {
            return baca_teks_terhad(&laluan, sasaran, pos);
        }
    }
    if let Some(isi) = modul_terbina(sasaran) {
        return Ok(isi.to_string());
    }
    for nama in [
        format!("modul_melayu/{}.ml", sasaran),
        format!("modul_melayu/{}/mod.ml", sasaran),
        format!("modul_melayu/{}/utama.ml", sasaran),
    ] {
        let laluan = semak_laluan_selamat(&nama, pos)?;
        if laluan.is_file() {
            return baca_teks_terhad(&laluan, sasaran, pos);
        }
    }
    Err(MelayuError::akses_fail_disekat(
        sasaran,
        "Fail modul tidak dijumpai (termasuk dalam modul_melayu/).",
        pos.line,
        pos.col,
    ))
}

pub fn builtin_baca_fail(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'bacaFail' memerlukan 1 hujah nama fail, tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion("Contoh: bacaFail(\"data.txt\")"));
    }

    let laluan_str = match &args[0] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };

    let safe_path = semak_laluan_selamat(laluan_str, pos)?;
    if !safe_path.exists() {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            "Fail tidak dijumpai.",
            pos.line,
            pos.col,
        ));
    }

    baca_teks_terhad(&safe_path, laluan_str, pos).map(Value::Teks)
}

pub fn builtin_tulis_fail(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'tulisFail' memerlukan 2 hujah (namaFail, kandungan), tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion("Contoh: tulisFail(\"nota.txt\", \"Kandungan nota\")"));
    }

    let laluan_str = match &args[0] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };

    let kandungan = match &args[1] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };

    if (kandungan.len() as u64) > HAD_SAIZ_FAIL_MAKSIMUM {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            &format!(
                "Kandungan teks yang hendak ditulis ({} bait) melebihi had keselamatan 5 MB.",
                kandungan.len()
            ),
            pos.line,
            pos.col,
        ));
    }

    let safe_path = semak_laluan_selamat(laluan_str, pos)?;

    akar_sandbox()
        .and_then(|akar| fail_selamat::tulis(&akar, &safe_path, kandungan.as_bytes()))
        .map_err(|e| {
            MelayuError::akses_fail_disekat(
                laluan_str,
                &format!("Gagal menulis ke fail: {}", e),
                pos.line,
                pos.col,
            )
        })?;

    Ok(Value::Betul)
}

pub fn builtin_wujud_fail(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'wujudFail' memerlukan 1 hujah nama fail, tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let laluan_str = match &args[0] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };

    let safe_path = semak_laluan_selamat(laluan_str, pos)?;
    Ok(if akar_sandbox().is_ok_and(|akar| fail_selamat::wujud(&akar, &safe_path)) {
        Value::Betul
    } else {
        Value::Salah
    })
}

pub fn builtin_padam_fail(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'padamFail' memerlukan 1 hujah nama fail, tetapi dapat {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let laluan_str = match &args[0] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ))
        }
    };

    let safe_path = semak_laluan_selamat(laluan_str, pos)?;
    let akar = akar_sandbox().map_err(|e| {
        MelayuError::akses_fail_disekat(laluan_str, &format!("Gagal mencari folder sandbox: {}", e), pos.line, pos.col)
    })?;
    if fail_selamat::wujud(&akar, &safe_path) {
        fail_selamat::padam(&akar, &safe_path).map_err(|e| {
            MelayuError::akses_fail_disekat(
                laluan_str,
                &format!("Gagal memadam fail: {}", e),
                pos.line,
                pos.col,
            )
        })?;
        Ok(Value::Betul)
    } else {
        Ok(Value::Salah)
    }
}

// ==========================================
// JSON Parser & Serializer Bawaan
// ==========================================

struct SimpleJsonParser {
    chars: Vec<char>,
    pos: usize,
    depth: usize,
    call_pos: LineCol,
}

impl SimpleJsonParser {
    fn new(input: &str, call_pos: LineCol) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            depth: 0,
            call_pos,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip_whitespace();
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.skip_whitespace();
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), MelayuError> {
        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == expected {
            self.pos += 1;
            Ok(())
        } else {
            let found = if self.pos < self.chars.len() {
                format!("'{}'", self.chars[self.pos])
            } else {
                "akhir teks".to_string()
            };
            Err(MelayuError::new(
                "Aduh!",
                &format!(
                    "Diharapkan aksara '{}' dalam JSON, tetapi menjumpai {}.",
                    expected, found
                ),
                self.call_pos.line,
                self.call_pos.col,
            )
            .with_kind(ErrorKind::Runtime))
        }
    }

    fn parse_value(&mut self) -> Result<Value, MelayuError> {
        if self.depth > 500 {
            return Err(MelayuError::had_memori_terlampau(
                "Kedalaman struktur JSON melebihi had keselamatan (500)",
                self.call_pos.line,
                self.call_pos.col,
            ));
        }

        self.skip_whitespace();
        let ch = match self.peek() {
            Some(c) => c,
            None => {
                return Err(MelayuError::new(
                    "Aduh!",
                    "Format JSON kosong atau tidak lengkap.",
                    self.call_pos.line,
                    self.call_pos.col,
                )
                .with_kind(ErrorKind::Runtime));
            }
        };

        match ch {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string().map(Value::Teks),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            '-' | '0'..='9' => self.parse_number(),
            other => Err(MelayuError::new(
                "Aduh!",
                &format!("Aksara tidak dijangka dalam JSON: '{}'", other),
                self.call_pos.line,
                self.call_pos.col,
            )
            .with_kind(ErrorKind::Runtime)),
        }
    }

    fn parse_object(&mut self) -> Result<Value, MelayuError> {
        self.expect_char('{')?;
        self.depth += 1;
        let mut map = PetaNilai::new();

        self.skip_whitespace();
        if let Some('}') = self.peek() {
            self.advance();
            self.depth -= 1;
            return Ok(Value::Kamus(Rc::new(RefCell::new(map))));
        }

        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.expect_char(':')?;
            let val = self.parse_value()?;
            map.insert(key, val);

            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                    if let Some('}') = self.peek() {
                        return Err(MelayuError::new(
                            "Aduh!",
                            "Koma berlebihan (trailing comma) tidak dibenarkan dalam JSON.",
                            self.call_pos.line,
                            self.call_pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                }
                Some('}') => {
                    self.advance();
                    break;
                }
                other => {
                    let found = match other {
                        Some(c) => format!("'{}'", c),
                        None => "akhir teks".to_string(),
                    };
                    return Err(MelayuError::new(
                        "Aduh!",
                        &format!(
                            "Diharapkan ',' atau '}}' dalam objek JSON, tetapi menjumpai {}.",
                            found
                        ),
                        self.call_pos.line,
                        self.call_pos.col,
                    )
                    .with_kind(ErrorKind::Runtime));
                }
            }
        }

        self.depth -= 1;
        Ok(Value::Kamus(Rc::new(RefCell::new(map))))
    }

    fn parse_array(&mut self) -> Result<Value, MelayuError> {
        self.expect_char('[')?;
        self.depth += 1;
        let mut list = Vec::new();

        self.skip_whitespace();
        if let Some(']') = self.peek() {
            self.advance();
            self.depth -= 1;
            return Ok(Value::senarai(list));
        }

        loop {
            let val = self.parse_value()?;
            list.push(val);

            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                    if let Some(']') = self.peek() {
                        return Err(MelayuError::new(
                            "Aduh!",
                            "Koma berlebihan (trailing comma) tidak dibenarkan dalam senarai JSON.",
                            self.call_pos.line,
                            self.call_pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                }
                Some(']') => {
                    self.advance();
                    break;
                }
                other => {
                    let found = match other {
                        Some(c) => format!("'{}'", c),
                        None => "akhir teks".to_string(),
                    };
                    return Err(MelayuError::new(
                        "Aduh!",
                        &format!(
                            "Diharapkan ',' atau ']' dalam senarai JSON, tetapi menjumpai {}.",
                            found
                        ),
                        self.call_pos.line,
                        self.call_pos.col,
                    )
                    .with_kind(ErrorKind::Runtime));
                }
            }
        }

        self.depth -= 1;
        Ok(Value::senarai(list))
    }

    fn parse_string(&mut self) -> Result<String, MelayuError> {
        self.expect_char('"')?;
        let mut s = String::new();

        while self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            match ch {
                '"' => return Ok(s),
                '\\' => {
                    if self.pos >= self.chars.len() {
                        return Err(MelayuError::new(
                            "Aduh!",
                            "Aksara lolos '\\' tidak lengkap pada akhir teks JSON.",
                            self.call_pos.line,
                            self.call_pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let esc = self.chars[self.pos];
                    self.pos += 1;
                    match esc {
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        '/' => s.push('/'),
                        'b' => s.push('\x08'),
                        'f' => s.push('\x0C'),
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        'u' => {
                            if self.pos + 4 > self.chars.len() {
                                return Err(MelayuError::new(
                                    "Aduh!",
                                    "Kod Unicode '\\uXXXX' tidak lengkap dalam JSON.",
                                    self.call_pos.line,
                                    self.call_pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                            let hex_str: String =
                                self.chars[self.pos..self.pos + 4].iter().collect();
                            self.pos += 4;
                            match u32::from_str_radix(&hex_str, 16) {
                                Ok(code) => match char::from_u32(code) {
                                    Some(c) => s.push(c),
                                    None => {
                                        return Err(MelayuError::new(
                                            "Aduh!",
                                            &format!(
                                                "Titik kod Unicode tidak sah '\\u{}'.",
                                                hex_str
                                            ),
                                            self.call_pos.line,
                                            self.call_pos.col,
                                        )
                                        .with_kind(ErrorKind::Runtime));
                                    }
                                },
                                Err(_) => {
                                    return Err(MelayuError::new(
                                        "Aduh!",
                                        &format!("Perenam-belas (hex) tidak sah '\\u{}'.", hex_str),
                                        self.call_pos.line,
                                        self.call_pos.col,
                                    )
                                    .with_kind(ErrorKind::Runtime));
                                }
                            }
                        }
                        other => {
                            return Err(MelayuError::new(
                                "Aduh!",
                                &format!("Aksara lolos tidak sah '\\{}' dalam JSON.", other),
                                self.call_pos.line,
                                self.call_pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    }
                }
                other => s.push(other),
            }
        }

        Err(MelayuError::new(
            "Aduh!",
            "Tanda petik penutup teks '\"' tidak dijumpai sebelum akhir fail.",
            self.call_pos.line,
            self.call_pos.col,
        )
        .with_kind(ErrorKind::Runtime))
    }

    fn parse_number(&mut self) -> Result<Value, MelayuError> {
        let start = self.pos;
        if self.pos < self.chars.len() && self.chars[self.pos] == '-' {
            self.pos += 1;
        }

        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        if self.pos < self.chars.len() && self.chars[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        if self.pos < self.chars.len()
            && (self.chars[self.pos] == 'e' || self.chars[self.pos] == 'E')
        {
            self.pos += 1;
            if self.pos < self.chars.len()
                && (self.chars[self.pos] == '+' || self.chars[self.pos] == '-')
            {
                self.pos += 1;
            }
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        let num_str: String = self.chars[start..self.pos].iter().collect();
        match num_str.parse::<f64>() {
            Ok(n) => Ok(Value::Nombor(n)),
            Err(_) => Err(MelayuError::new(
                "Aduh!",
                &format!("Format nombor tidak sah dalam JSON: '{}'", num_str),
                self.call_pos.line,
                self.call_pos.col,
            )
            .with_kind(ErrorKind::Runtime)),
        }
    }

    fn parse_bool(&mut self) -> Result<Value, MelayuError> {
        if self.pos + 4 <= self.chars.len() {
            let slice: String = self.chars[self.pos..self.pos + 4].iter().collect();
            if slice == "true" {
                self.pos += 4;
                return Ok(Value::Betul);
            }
        }
        if self.pos + 5 <= self.chars.len() {
            let slice: String = self.chars[self.pos..self.pos + 5].iter().collect();
            if slice == "false" {
                self.pos += 5;
                return Ok(Value::Salah);
            }
        }
        Err(MelayuError::new(
            "Aduh!",
            "Nilai boolean tidak sah dalam JSON.",
            self.call_pos.line,
            self.call_pos.col,
        )
        .with_kind(ErrorKind::Runtime))
    }

    fn parse_null(&mut self) -> Result<Value, MelayuError> {
        if self.pos + 4 <= self.chars.len() {
            let slice: String = self.chars[self.pos..self.pos + 4].iter().collect();
            if slice == "null" {
                self.pos += 4;
                return Ok(Value::Kosong);
            }
        }
        Err(MelayuError::new(
            "Aduh!",
            "Nilai null tidak sah dalam JSON.",
            self.call_pos.line,
            self.call_pos.col,
        )
        .with_kind(ErrorKind::Runtime))
    }
}

pub fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\x08' => out.push_str("\\b"),
            '\x0C' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn nilai_ke_json_string(
    val: &Value,
    cantik: bool,
    kedalaman: usize,
    pos: LineCol,
) -> Result<String, MelayuError> {
    let mut keluar = String::new();
    tulis_json(val, cantik, kedalaman, pos, &mut keluar)?;
    Ok(keluar)
}

fn nombor_json(n: f64) -> String {
    if !n.is_finite() {
        "null".to_string()
    } else if n.fract() == 0.0 && n.abs() < 9_007_199_254_740_992.0 {
        (n as i64).to_string()
    } else {
        n.to_string()
    }
}

fn baris_json(keluar: &mut String, cantik: bool, kedalaman: usize) {
    if cantik {
        keluar.push('\n');
        keluar.push_str(&"  ".repeat(kedalaman));
    }
}

fn peta_json(
    peta: &PetaNilai,
    cantik: bool,
    kedalaman: usize,
    pos: LineCol,
    keluar: &mut String,
) -> Result<(), MelayuError> {
    if peta.is_empty() {
        keluar.push_str("{}");
        return Ok(());
    }
    keluar.push('{');
    for (i, (k, v)) in peta.iter().enumerate() {
        if i > 0 {
            keluar.push(',');
        }
        baris_json(keluar, cantik, kedalaman + 1);
        keluar.push_str(&escape_json_string(k));
        keluar.push_str(if cantik { ": " } else { ":" });
        tulis_json(v, cantik, kedalaman + 1, pos, keluar)?;
    }
    baris_json(keluar, cantik, kedalaman);
    keluar.push('}');
    Ok(())
}

fn tulis_json(
    val: &Value,
    cantik: bool,
    kedalaman: usize,
    pos: LineCol,
    keluar: &mut String,
) -> Result<(), MelayuError> {
    if kedalaman > 500 {
        return Err(MelayuError::had_memori_terlampau(
            "Kedalaman struktur penukaran ke JSON melebihi had keselamatan (500)",
            pos.line,
            pos.col,
        ));
    }
    if keluar.len() > HAD_PAPARAN {
        return Err(MelayuError::had_memori_terlampau(
            "Teks JSON melebihi 64 MB",
            pos.line,
            pos.col,
        ));
    }

    match val {
        Value::Kosong => keluar.push_str("null"),
        Value::Betul => keluar.push_str("true"),
        Value::Salah => keluar.push_str("false"),
        Value::Nombor(n) => keluar.push_str(&nombor_json(*n)),
        Value::Teks(s) => keluar.push_str(&escape_json_string(s)),
        Value::Senarai(list) => {
            let list = list.borrow();
            if list.is_empty() {
                keluar.push_str("[]");
                return Ok(());
            }
            keluar.push('[');
            for (i, item) in list.iter().enumerate() {
                if i > 0 {
                    keluar.push(',');
                }
                baris_json(keluar, cantik, kedalaman + 1);
                tulis_json(item, cantik, kedalaman + 1, pos, keluar)?;
            }
            baris_json(keluar, cantik, kedalaman);
            keluar.push(']');
        }
        Value::Kamus(map) => peta_json(&map.borrow(), cantik, kedalaman, pos, keluar)?,
        Value::Objek(o) => peta_json(&o.sifat.borrow(), cantik, kedalaman, pos, keluar)?,
        Value::Tensor { data, shape } => {
            let lajur = if shape.len() == 2 { shape[1].max(1) } else { data.len().max(1) };
            let baris: Vec<String> = data
                .chunks(lajur)
                .map(|b| format!("[{}]", b.iter().map(|&v| tensor_json(v)).collect::<Vec<_>>().join(",")))
                .collect();
            if shape.len() == 2 {
                keluar.push_str(&format!("[{}]", baris.join(",")));
            } else {
                keluar.push_str(baris.first().map_or("[]", |b| b));
            }
        }
        other => {
            return Err(MelayuError::new(
                "Alamak!",
                &format!("Jenis '{}' tidak boleh ditukar ke format JSON.", other.jenis()),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime))
        }
    }
    Ok(())
}

fn tensor_json(v: f64) -> String {
    if !v.is_finite() {
        "null".to_string()
    } else if v.fract() == 0.0 {
        format!("{:.0}", v)
    } else {
        v.to_string()
    }
}

pub fn builtin_baca_json(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'bacaJSON' memerlukan 1 hujah teks, tetapi menerima {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion("Contoh: ada data = bacaJSON(\"{\\\"nama\\\": \\\"Ali\\\"}\")"));
    }

    let teks = match &args[0] {
        Value::Teks(s) => s.as_str(),
        other => {
            return Err(MelayuError::jenis_salah(
                "teks",
                other.jenis(),
                pos.line,
                pos.col,
            ));
        }
    };

    let mut parser = SimpleJsonParser::new(teks, pos);
    let val = parser.parse_value()?;
    parser.skip_whitespace();
    if parser.pos < parser.chars.len() {
        return Err(MelayuError::new(
            "Aduh!",
            "Terdapat aksara berlebihan selepas data JSON yang sah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    Ok(val)
}

pub fn builtin_tulis_json(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.is_empty() || args.len() > 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Fungsi 'tulisJSON' memerlukan 1 atau 2 hujah (nilai, cantik?), tetapi menerima {}.",
                args.len()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)
        .with_suggestion("Contoh: tulisJSON(data) atau tulisJSON(data, betul)"));
    }

    let cantik = if args.len() == 2 {
        args[1].is_truthy()
    } else {
        false
    };

    let json_str = nilai_ke_json_string(&args[0], cantik, 0, pos)?;
    Ok(Value::Teks(json_str))
}

// ==========================================
// Utiliti Tarikh & Masa Bawaan
// ==========================================

fn hari_ke_tarikh(hari_sejak_epoch: i64) -> (i32, u32, u32) {
    let z = hari_sejak_epoch + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

fn dapatkan_masa_kini_komponen() -> (i32, u32, u32, u64, u64, u64, u64) {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let total_secs = duration.as_secs();
    let hari = (total_secs / 86400) as i64;
    let (tahun, bulan, hari_hb) = hari_ke_tarikh(hari);
    let sisa = total_secs % 86400;
    let jam = sisa / 3600;
    let minit = (sisa % 3600) / 60;
    let saat = sisa % 60;
    (tahun, bulan, hari_hb, jam, minit, saat, total_secs)
}

pub fn builtin_cap_masa(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if !args.is_empty() {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'capMasa' tidak memerlukan sebarang hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    Ok(Value::Nombor(duration.as_secs() as f64))
}

pub fn builtin_masa_kini(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if !args.is_empty() {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'masaKini' tidak memerlukan sebarang hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let (_, _, _, jam, minit, saat, _) = dapatkan_masa_kini_komponen();
    Ok(Value::Teks(format!("{:02}:{:02}:{:02}", jam, minit, saat)))
}

pub fn builtin_tarikh_kini(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if !args.is_empty() {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'tarikhKini' tidak memerlukan sebarang hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let (tahun, bulan, hari_hb, _, _, _, _) = dapatkan_masa_kini_komponen();
    Ok(Value::Teks(format!(
        "{:04}-{:02}-{:02}",
        tahun, bulan, hari_hb
    )))
}


// ==============================================================================
// AKALML - PRIMITIF PEMBELAJARAN MESIN & ALJABAR LINEAR NATIVE
// ==============================================================================

fn ekstrak_matriks(val: &Value, pos: LineCol) -> Result<Vec<Vec<f64>>, MelayuError> {
    match val {
        Value::Senarai(list_rc) => {
            let list = list_rc.borrow();
            if list.is_empty() {
                return Ok(Vec::new());
            }
            let mut matrix = Vec::with_capacity(list.len());
            let mut expected_cols: Option<usize> = None;
            for row_val in list.iter() {
                match row_val {
                    Value::Senarai(row_rc) => {
                        let row = row_rc.borrow();
                        if let Some(exp) = expected_cols {
                            if row.len() != exp {
                                return Err(MelayuError::new(
                                    "Alamak!",
                                    &format!("Semua baris matriks mestilah mempunyai panjang yang sama (dijangka {}, dapat {}).", exp, row.len()),
                                    pos.line,
                                    pos.col,
                                ).with_kind(ErrorKind::Runtime));
                            }
                        } else {
                            expected_cols = Some(row.len());
                        }
                        let mut num_row = Vec::with_capacity(row.len());
                        for el in row.iter() {
                            match el {
                                Value::Nombor(n) => num_row.push(*n),
                                _ => return Err(MelayuError::new(
                                    "Alamak!",
                                    "Semua elemen matriks mestilah bernilai nombor.",
                                    pos.line,
                                    pos.col,
                                ).with_kind(ErrorKind::Runtime)),
                            }
                        }
                        matrix.push(num_row);
                    }
                    _ => return Err(MelayuError::new(
                        "Alamak!",
                        "Matriks mestilah senarai 2-dimensi (senarai di dalam senarai).",
                        pos.line,
                        pos.col,
                    ).with_kind(ErrorKind::Runtime)),
                }
            }
            Ok(matrix)
        }
        _ => Err(MelayuError::new(
            "Alamak!",
            "Argumen matriks mestilah bernilai senarai 2-dimensi.",
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime)),
    }
}

fn matriks_ke_nilai(mat: &[Vec<f64>]) -> Value {
    let rows: Vec<Value> = mat.iter().map(|row| {
        let cols: Vec<Value> = row.iter().map(|&n| Value::Nombor(n)).collect();
        Value::senarai(cols)
    }).collect();
    Value::senarai(rows)
}

fn ekstrak_vektor(val: &Value, pos: LineCol) -> Result<Vec<f64>, MelayuError> {
    match val {
        Value::Senarai(list_rc) => {
            let list = list_rc.borrow();
            let mut vec = Vec::with_capacity(list.len());
            for el in list.iter() {
                match el {
                    Value::Nombor(n) => vec.push(*n),
                    _ => return Err(MelayuError::new(
                        "Alamak!",
                        "Elemen vektor mestilah nombor.",
                        pos.line,
                        pos.col,
                    ).with_kind(ErrorKind::Runtime)),
                }
            }
            Ok(vec)
        }
        _ => Err(MelayuError::new(
            "Alamak!",
            "Vektor mestilah bernilai senarai nombor.",
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_darab_matriks(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'darabMatriks' memerlukan 2 hujah (A, B), tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let b = ekstrak_matriks(&args[1], pos)?;
    if a.is_empty() || b.is_empty() {
        return Ok(Value::senarai(Vec::new()));
    }
    let m = a.len();
    let k = a[0].len();
    let k2 = b.len();
    let n = b[0].len();
    if k != k2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Dimensi matriks tidak serasi untuk didarab: A ({}x{}) dan B ({}x{}). Lajur A mestilah sama dengan baris B.", m, k, k2, n),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    if m.saturating_mul(n) > HAD_ELEMEN_TENSOR_MAKSIMUM {
        return Err(MelayuError::had_memori_terlampau(
            "Hasil darab matriks melebihi had elemen maksimum (1,000,000).",
            pos.line,
            pos.col,
        ));
    }
    let mut c = vec![vec![0.0; n]; m];
    let num_threads = std::thread::available_parallelism()
        .map(|t| t.get())
        .unwrap_or(1)
        .min(m);

    if m * k * n >= 32_768 && num_threads > 1 {
        let chunk_rows = m.div_ceil(num_threads);
        let a_ref = &a;
        let b_ref = &b;
        std::thread::scope(|scope| {
            for (t_idx, c_chunk) in c.chunks_mut(chunk_rows).enumerate() {
                let row_start = t_idx * chunk_rows;
                scope.spawn(move || {
                    for (local_r, c_row) in c_chunk.iter_mut().enumerate() {
                        let i = row_start + local_r;
                        for p in 0..k {
                            let a_ip = a_ref[i][p];
                            let b_row = &b_ref[p];
                            for j in 0..n {
                                c_row[j] += a_ip * b_row[j];
                            }
                        }
                    }
                });
            }
        });
    } else {
        for i in 0..m {
            for p in 0..k {
                let a_ip = a[i][p];
                for j in 0..n {
                    c[i][j] += a_ip * b[p][j];
                }
            }
        }
    }
    Ok(matriks_ke_nilai(&c))
}

pub fn builtin_transposisi_matriks(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'transposisiMatriks' memerlukan 1 hujah, tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    if a.is_empty() {
        return Ok(Value::senarai(Vec::new()));
    }
    let m = a.len();
    let n = a[0].len();
    let mut t = vec![vec![0.0; m]; n];
    for i in 0..m {
        for j in 0..n {
            t[j][i] = a[i][j];
        }
    }
    Ok(matriks_ke_nilai(&t))
}

pub fn builtin_songsang_matriks(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'songsangMatriks' memerlukan 1 hujah, tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let n = a.len();
    if n == 0 || a[0].len() != n {
        return Err(MelayuError::new(
            "Alamak!",
            "Matriks mestilah segi empat sama (N x N) dan tidak kosong untuk disongsangkan.",
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }

    let mut aug = vec![vec![0.0; 2 * n]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n + i] = 1.0;
    }

    for col in 0..n {
        let mut max_row = col;
        let mut max_val = aug[col][col].abs();
        for row in (col + 1)..n {
            let val = aug[row][col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }

        if max_val < 1e-12 {
            return Err(MelayuError::new(
                "Alamak!",
                "Matriks ini bersifat singular (penentu sifar) dan tidak mempunyai songsangan.",
                pos.line,
                pos.col,
            ).with_kind(ErrorKind::Runtime));
        }

        if max_row != col {
            aug.swap(col, max_row);
        }

        let pivot = aug[col][col];
        for j in 0..(2 * n) {
            aug[col][j] /= pivot;
        }

        for row in 0..n {
            if row != col {
                let factor = aug[row][col];
                if factor.abs() > 1e-15 {
                    for j in 0..(2 * n) {
                        aug[row][j] -= factor * aug[col][j];
                    }
                }
            }
        }
    }

    let mut inv = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            inv[i][j] = aug[i][n + j];
        }
    }
    Ok(matriks_ke_nilai(&inv))
}

pub fn builtin_titik_darab(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'titikDarab' memerlukan 2 hujah (vektor1, vektor2), tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let v1 = ekstrak_vektor(&args[0], pos)?;
    let v2 = ekstrak_vektor(&args[1], pos)?;
    if v1.len() != v2.len() {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Panjang vektor tidak sama: {} vs {}.", v1.len(), v2.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let mut sum = 0.0;
    for i in 0..v1.len() {
        sum += v1[i] * v2[i];
    }
    Ok(Value::Nombor(sum))
}

pub fn builtin_jarak_euclidean(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'jarakEuclidean' memerlukan 2 hujah (titik1, titik2), tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let v1 = ekstrak_vektor(&args[0], pos)?;
    let v2 = ekstrak_vektor(&args[1], pos)?;
    if v1.len() != v2.len() {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Dimensi titik tidak sama: {} vs {}.", v1.len(), v2.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let mut sum_sq = 0.0;
    for i in 0..v1.len() {
        let diff = v1[i] - v2[i];
        sum_sq += diff * diff;
    }
    Ok(Value::Nombor(sum_sq.sqrt()))
}

fn parse_baris_csv(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

pub fn builtin_baca_csv(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.is_empty() || args.len() > 2 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'bacaCSV' memerlukan 1 atau 2 hujah (laluanFail, adaTajuk?), tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let path_str = match &args[0] {
        Value::Teks(s) => s.clone(),
        _ => return Err(MelayuError::new("Alamak!", "Laluan fail CSV mestilah bernilai teks.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    let ada_tajuk = if args.len() == 2 {
        match &args[1] {
            Value::Betul => true,
            Value::Salah => false,
            _ => true,
        }
    } else {
        true
    };

    let safe_path = semak_laluan_selamat(&path_str, pos)?;
    if let Ok(meta) = fs::metadata(&safe_path) {
        if meta.len() > HAD_SAIZ_FAIL_MAKSIMUM {
            return Err(MelayuError::had_memori_terlampau(
                "Saiz fail CSV melebihi had keselamatan maksimum (5 MB).",
                pos.line,
                pos.col,
            ));
        }
    }
    let content = baca_teks_terhad(&safe_path, &path_str, pos)?;

    let mut lines = content.lines().filter(|l| !l.trim().is_empty());
    let mut columns = Vec::new();

    if ada_tajuk {
        if let Some(header_line) = lines.next() {
            for col in parse_baris_csv(header_line) {
                columns.push(Value::Teks(col));
            }
        }
    }

    let mut data_rows = Vec::new();
    for line in lines {
        let cells = parse_baris_csv(line);
        let row_vals: Vec<Value> = cells.into_iter().map(|c| {
            if let Ok(n) = c.trim().parse::<f64>() {
                Value::Nombor(n)
            } else {
                Value::Teks(c)
            }
        }).collect();
        data_rows.push(Value::senarai(row_vals));
    }

    if !ada_tajuk && !data_rows.is_empty() {
        let col_count = if let Value::Senarai(ref r) = data_rows[0] {
            r.borrow().len()
        } else {
            0
        };
        for i in 0..col_count {
            columns.push(Value::Teks(format!("lajur_{}", i)));
        }
    }

    let mut map = PetaNilai::new();
    let row_count = data_rows.len() as f64;
    let col_count = columns.len() as f64;
    map.insert("lajur".to_string(), Value::senarai(columns));
    map.insert("data".to_string(), Value::senarai(data_rows));
    map.insert("baris".to_string(), Value::Nombor(row_count));
    map.insert("bil_lajur".to_string(), Value::Nombor(col_count));

    Ok(Value::Kamus(Rc::new(RefCell::new(map))))
}

pub fn builtin_bahagi_latih_uji(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Fungsi 'bahagiLatihUji' memerlukan 2 atau 3 hujah (X, y, nisbahUji?), tetapi dapat {}.", args.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let x_list = match &args[0] {
        Value::Senarai(rc) => rc.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "Argumen X mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    let y_list = match &args[1] {
        Value::Senarai(rc) => rc.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "Argumen y mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    if x_list.len() != y_list.len() {
        return Err(MelayuError::new(
            "Alamak!",
            &format!("Panjang X ({}) dan y ({}) mestilah sama.", x_list.len(), y_list.len()),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }
    let nisbah = if args.len() == 3 {
        match &args[2] {
            Value::Nombor(n) if *n > 0.0 && *n < 1.0 => *n,
            _ => 0.2,
        }
    } else {
        0.2
    };

    let total = x_list.len();
    let test_size = ((total as f64) * nisbah).round() as usize;
    let train_size = total.saturating_sub(test_size);

    let mut x_train = Vec::with_capacity(train_size);
    let mut x_test = Vec::with_capacity(test_size);
    let mut y_train = Vec::with_capacity(train_size);
    let mut y_test = Vec::with_capacity(test_size);

    let mut pasangan: Vec<(Value, Value)> = x_list.into_iter().zip(y_list).collect();
    for i in (1..pasangan.len()).rev() {
        let j = (jana_nombor_rawak_u64() % (i as u64 + 1)) as usize;
        pasangan.swap(i, j);
    }

    for (i, (x_val, y_val)) in pasangan.into_iter().enumerate() {
        if i < train_size {
            x_train.push(x_val);
            y_train.push(y_val);
        } else {
            x_test.push(x_val);
            y_test.push(y_val);
        }
    }

    Ok(Value::senarai(vec![
        Value::senarai(x_train),
        Value::senarai(x_test),
        Value::senarai(y_train),
        Value::senarai(y_test),
    ]))
}

pub fn builtin_purata(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'purata' memerlukan 1 hujah (senarai).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let v = ekstrak_vektor(&args[0], pos)?;
    if v.is_empty() {
        return Err(MelayuError::new("Alamak!", "Senarai tidak boleh kosong untuk menghitung purata.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let sum: f64 = v.iter().sum();
    Ok(Value::Nombor(sum / (v.len() as f64)))
}

pub fn builtin_sisihan_piawai(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'sisihanPiawai' memerlukan 1 hujah (senarai).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let v = ekstrak_vektor(&args[0], pos)?;
    if v.len() < 2 {
        return Err(MelayuError::new("Alamak!", "Senarai memerlukan sekurang-kurangnya 2 elemen untuk sisihan piawai.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mean = v.iter().sum::<f64>() / (v.len() as f64);
    let var = v.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / ((v.len() - 1) as f64);
    Ok(Value::Nombor(var.sqrt()))
}

pub fn builtin_skor_mse(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'skorMSE' memerlukan 2 hujah (ySebenar, yRamalan).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let y_true = ekstrak_vektor(&args[0], pos)?;
    let y_pred = ekstrak_vektor(&args[1], pos)?;
    if y_true.len() != y_pred.len() || y_true.is_empty() {
        return Err(MelayuError::new("Alamak!", "Panjang senarai ySebenar dan yRamalan mestilah sama dan tidak kosong.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let n = y_true.len() as f64;
    let mut sum_sq = 0.0;
    for i in 0..y_true.len() {
        let diff = y_true[i] - y_pred[i];
        sum_sq += diff * diff;
    }
    Ok(Value::Nombor(sum_sq / n))
}

pub fn builtin_skor_r2(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'skorR2' memerlukan 2 hujah (ySebenar, yRamalan).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let y_true = ekstrak_vektor(&args[0], pos)?;
    let y_pred = ekstrak_vektor(&args[1], pos)?;
    if y_true.len() != y_pred.len() || y_true.is_empty() {
        return Err(MelayuError::new("Alamak!", "Panjang senarai ySebenar dan yRamalan mestilah sama dan tidak kosong.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mean_true = y_true.iter().sum::<f64>() / (y_true.len() as f64);
    let ss_tot: f64 = y_true.iter().map(|&y| (y - mean_true).powi(2)).sum();
    let ss_res: f64 = y_true.iter().zip(y_pred.iter()).map(|(&y, &y_hat)| (y - y_hat).powi(2)).sum();
    if ss_tot.abs() < 1e-12 {
        return Ok(Value::Nombor(1.0));
    }
    let r2 = 1.0 - (ss_res / ss_tot);
    Ok(Value::Nombor(r2))
}

pub fn builtin_skor_akurasi(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'skorAkurasi' memerlukan 2 hujah (ySebenar, yRamalan).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let y_true = match &args[0] {
        Value::Senarai(rc) => rc.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "ySebenar mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    let y_pred = match &args[1] {
        Value::Senarai(rc) => rc.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "yRamalan mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    if y_true.len() != y_pred.len() || y_true.is_empty() {
        return Err(MelayuError::new("Alamak!", "Panjang senarai ySebenar dan yRamalan mestilah sama dan tidak kosong.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mut matches = 0;
    for (t, p) in y_true.iter().zip(y_pred.iter()) {
        if t == p {
            matches += 1;
        }
    }
    let acc = (matches as f64) / (y_true.len() as f64);
    Ok(Value::Nombor(acc))
}


pub fn builtin_eksponen(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'eksponen' memerlukan 1 hujah (nombor).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.exp())),
        _ => Err(MelayuError::new("Alamak!", "Hujah untuk 'eksponen' mestilah nombor.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_logaritma_asli(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'logaritmaAsli' memerlukan 1 hujah (nombor).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => {
            if *n <= 0.0 {
                return Err(MelayuError::new("Alamak!", "Logaritma asli memerlukan nombor positif lebih besar daripada sifar.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
            }
            Ok(Value::Nombor(n.ln()))
        }
        _ => Err(MelayuError::new("Alamak!", "Hujah untuk 'logaritmaAsli' mestilah nombor.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_fungsi_sigmoid(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'fungsiSigmoid' memerlukan 1 hujah (nombor).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => {
            let clamped = n.clamp(-30.0, 30.0);
            Ok(Value::Nombor(1.0 / (1.0 + (-clamped).exp())))
        }
        _ => Err(MelayuError::new("Alamak!", "Hujah mestilah nombor.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_fungsi_relu(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'fungsiReLU' memerlukan 1 hujah (nombor).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Nombor(n) => Ok(Value::Nombor(n.max(0.0))),
        _ => Err(MelayuError::new("Alamak!", "Hujah mestilah nombor.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_darab_hadamard(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'darabHadamard' memerlukan 2 matriks sama saiz.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let b = ekstrak_matriks(&args[1], pos)?;
    if a.len() != b.len() || (!a.is_empty() && a[0].len() != b[0].len()) {
        return Err(MelayuError::new("Alamak!", "Dimensi matriks mestilah sama untuk darab Hadamard.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mut res = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            res[i][j] = a[i][j] * b[i][j];
        }
    }
    Ok(matriks_ke_nilai(&res))
}

pub fn builtin_matriks_tambah(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksTambah' memerlukan 2 matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let b = ekstrak_matriks(&args[1], pos)?;
    let broadcast_b = b.len() == 1 && (!a.is_empty() && b[0].len() == a[0].len());
    if !broadcast_b && (a.len() != b.len() || (!a.is_empty() && a[0].len() != b[0].len())) {
        return Err(MelayuError::new("Alamak!", "Dimensi matriks mestilah sama atau vektor bias sepadan untuk penambahan.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mut res = vec![vec![0.0; if a.is_empty() { 0 } else { a[0].len() }]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            let b_val = if broadcast_b { b[0][j] } else { b[i][j] };
            res[i][j] = a[i][j] + b_val;
        }
    }
    Ok(matriks_ke_nilai(&res))
}

pub fn builtin_matriks_tolak(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksTolak' memerlukan 2 matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let b = ekstrak_matriks(&args[1], pos)?;
    if a.len() != b.len() || (!a.is_empty() && a[0].len() != b[0].len()) {
        return Err(MelayuError::new("Alamak!", "Dimensi matriks mestilah sama untuk penolakan.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mut res = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            res[i][j] = a[i][j] - b[i][j];
        }
    }
    Ok(matriks_ke_nilai(&res))
}

pub fn builtin_matriks_skala(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksSkala' memerlukan matriks dan faktor skala.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let k = match &args[1] {
        Value::Nombor(n) => *n,
        _ => return Err(MelayuError::new("Alamak!", "Faktor skala mestilah nombor.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    let mut res = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            res[i][j] = a[i][j] * k;
        }
    }
    Ok(matriks_ke_nilai(&res))
}

pub fn builtin_matriks_jumlah_lajur(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksJumlahLajur' memerlukan 1 matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    if a.is_empty() {
        return Ok(matriks_ke_nilai(&vec![vec![]]));
    }
    let rows = a.len();
    let cols = a[0].len();
    let mut sum_row = vec![0.0; cols];
    for i in 0..rows {
        for j in 0..cols {
            sum_row[j] += a[i][j];
        }
    }
    Ok(matriks_ke_nilai(&vec![sum_row]))
}

pub fn builtin_matriks_sigmoid(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksSigmoid' memerlukan 1 matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let mut res = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            let clamped = a[i][j].clamp(-30.0, 30.0);
            res[i][j] = 1.0 / (1.0 + (-clamped).exp());
        }
    }
    Ok(matriks_ke_nilai(&res))
}

pub fn builtin_matriks_derivatif_sigmoid(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksDerivatifSigmoid' memerlukan 1 matriks (output sigmoid A).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let a = ekstrak_matriks(&args[0], pos)?;
    let mut res = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            let s = a[i][j];
            res[i][j] = s * (1.0 - s);
        }
    }
    Ok(matriks_ke_nilai(&res))
}

fn nilai_ke_boolean_positif(val: &Value, pos_label: Option<&Value>) -> bool {
    if let Some(target) = pos_label {
        return val == target;
    }
    match val {
        Value::Nombor(n) => *n >= 0.5,
        Value::Betul => true,
        Value::Salah => false,
        Value::Teks(s) => {
            let lower = s.to_lowercase();
            lower == "1" || lower == "ya" || lower == "true" || lower == "positif" || lower == "p" || lower == "yes"
        }
        _ => false,
    }
}

pub fn builtin_matriks_kekeliruan(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 && args.len() != 3 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksKekeliruan' memerlukan 2 atau 3 hujah (ySebenar, yRamal, [labelPositif]).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let y_true = match &args[0] {
        Value::Senarai(l) => l.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "Hujah pertama 'ySebenar' mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    let y_pred = match &args[1] {
        Value::Senarai(l) => l.borrow().clone(),
        _ => return Err(MelayuError::new("Alamak!", "Hujah kedua 'yRamal' mestilah senarai.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    if y_true.len() != y_pred.len() {
        return Err(MelayuError::new("Alamak!", "Panjang senarai ySebenar dan yRamal mestilah sama.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let pos_label = if args.len() == 3 { Some(&args[2]) } else { None };
    let mut tp = 0.0;
    let mut fp = 0.0;
    let mut tn = 0.0;
    let mut fn_val = 0.0;
    for (t, p) in y_true.iter().zip(y_pred.iter()) {
        let is_t = nilai_ke_boolean_positif(t, pos_label);
        let is_p = nilai_ke_boolean_positif(p, pos_label);
        match (is_t, is_p) {
            (true, true) => tp += 1.0,
            (false, true) => fp += 1.0,
            (false, false) => tn += 1.0,
            (true, false) => fn_val += 1.0,
        }
    }
    let mut map = PetaNilai::new();
    map.insert("TP".to_string(), Value::Nombor(tp));
    map.insert("FP".to_string(), Value::Nombor(fp));
    map.insert("TN".to_string(), Value::Nombor(tn));
    map.insert("FN".to_string(), Value::Nombor(fn_val));
    Ok(Value::Kamus(Rc::new(RefCell::new(map))))
}

pub fn builtin_kejituan(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let conf = builtin_matriks_kekeliruan(args, pos)?;
    if let Value::Kamus(rc) = conf {
        let m = rc.borrow();
        let tp = match m.get("TP") { Some(Value::Nombor(n)) => *n, _ => 0.0 };
        let fp = match m.get("FP") { Some(Value::Nombor(n)) => *n, _ => 0.0 };
        if tp + fp == 0.0 {
            return Ok(Value::Nombor(1.0));
        }
        return Ok(Value::Nombor(tp / (tp + fp)));
    }
    Ok(Value::Nombor(0.0))
}

pub fn builtin_kepekaan(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let conf = builtin_matriks_kekeliruan(args, pos)?;
    if let Value::Kamus(rc) = conf {
        let m = rc.borrow();
        let tp = match m.get("TP") { Some(Value::Nombor(n)) => *n, _ => 0.0 };
        let fn_val = match m.get("FN") { Some(Value::Nombor(n)) => *n, _ => 0.0 };
        if tp + fn_val == 0.0 {
            return Ok(Value::Nombor(1.0));
        }
        return Ok(Value::Nombor(tp / (tp + fn_val)));
    }
    Ok(Value::Nombor(0.0))
}

pub fn builtin_skor_f1(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let p_val = builtin_kejituan(args, pos)?;
    let r_val = builtin_kepekaan(args, pos)?;
    let p = match p_val { Value::Nombor(n) => n, _ => 0.0 };
    let r = match r_val { Value::Nombor(n) => n, _ => 0.0 };
    if p + r == 0.0 {
        return Ok(Value::Nombor(0.0));
    }
    Ok(Value::Nombor(2.0 * p * r / (p + r)))
}

const MAKS_SAMBUNGAN: usize = 128;
const HAD_PENGEPALA: usize = 16_384;
const HAD_BADAN: usize = 1_048_576;
const MASA_BACA: Duration = Duration::from_secs(10);

struct PermintaanMentah {
    kaedah: String,
    laluan: String,
    pengepala: Vec<(String, String)>,
    badan: String,
}

fn bina_respons(status: u16, jenis: &str, badan: &str) -> String {
    let mesej = match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        408 => "Request Timeout",
        413 => "Payload Too Large",
        431 => "Request Header Fields Too Large",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Response",
    };
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nX-XSS-Protection: 1; mode=block\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\nConnection: close\r\n\r\n{}",
        status,
        mesej,
        jenis.replace(['\r', '\n'], ""),
        badan.len(),
        badan
    )
}

fn respons_ralat(status: u16, mesej: &str) -> String {
    bina_respons(status, "application/json", &format!("{{\"ralat\": \"{}\"}}", mesej))
}

fn baca_permintaan(aliran: &mut TcpStream) -> Result<Option<PermintaanMentah>, String> {
    let tamat = Instant::now() + MASA_BACA;
    let mut data: Vec<u8> = Vec::with_capacity(4096);
    let mut penimbal = [0u8; 4096];
    let mut hujung_pengepala = None;
    let mut panjang_badan = 0usize;
    loop {
        if let Some(hujung) = hujung_pengepala {
            if data.len() >= hujung + panjang_badan {
                break;
            }
        }
        let baki = tamat.saturating_duration_since(Instant::now());
        if baki.is_zero() || aliran.set_read_timeout(Some(baki)).is_err() {
            return Err(respons_ralat(408, "Permintaan terlalu perlahan."));
        }
        match aliran.read(&mut penimbal) {
            Ok(0) if data.is_empty() => return Ok(None),
            Ok(0) => return Err(respons_ralat(400, "Permintaan tidak lengkap.")),
            Ok(n) => data.extend_from_slice(&penimbal[..n]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) if data.is_empty() => return Ok(None),
            Err(_) => return Err(respons_ralat(408, "Permintaan terlalu perlahan.")),
        }
        if hujung_pengepala.is_none() {
            match data.windows(4).position(|w| w == b"\r\n\r\n") {
                Some(i) if i <= HAD_PENGEPALA => {
                    hujung_pengepala = Some(i + 4);
                    let teks = String::from_utf8_lossy(&data[..i]);
                    panjang_badan = teks
                        .lines()
                        .skip(1)
                        .filter_map(|b| b.split_once(':'))
                        .filter(|(k, _)| k.trim().eq_ignore_ascii_case("content-length"))
                        .map(|(_, v)| {
                            let v = v.trim();
                            let digit = !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit());
                            v.parse().unwrap_or(if digit { usize::MAX } else { 0 })
                        })
                        .last()
                        .unwrap_or(0);
                    if panjang_badan > HAD_BADAN {
                        return Err(respons_ralat(413, "Saiz muatan melebihi had keselamatan (1 MB)"));
                    }
                }
                None if data.len() <= HAD_PENGEPALA => {}
                _ => return Err(respons_ralat(431, "Pengepala permintaan terlalu besar.")),
            }
        }
    }

    let hujung = hujung_pengepala.unwrap_or(data.len());
    let teks = String::from_utf8_lossy(&data[..hujung]);
    let mut baris = teks.lines();
    let mut pertama = baris.next().unwrap_or("").split_whitespace();
    let kaedah = pertama.next().unwrap_or("GET").to_string();
    let laluan = pertama.next().unwrap_or("/").to_string();
    let pengepala = baris
        .filter_map(|b| b.split_once(':'))
        .map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_string()))
        .collect();
    let akhir = if panjang_badan > 0 { hujung + panjang_badan } else { data.len() };
    Ok(Some(PermintaanMentah {
        kaedah,
        laluan,
        pengepala,
        badan: String::from_utf8_lossy(&data[hujung..akhir]).to_string(),
    }))
}

fn kendali_permintaan(pengendali: &Value, permintaan: PermintaanMentah, pos: LineCol) -> String {
    let mut pengepala = PetaNilai::new();
    for (k, v) in permintaan.pengepala {
        pengepala.insert(k, Value::Teks(v));
    }
    let mut kamus = PetaNilai::new();
    kamus.insert("kaedah".to_string(), Value::Teks(permintaan.kaedah));
    kamus.insert("laluan".to_string(), Value::Teks(permintaan.laluan));
    kamus.insert("badan".to_string(), Value::Teks(permintaan.badan));
    kamus.insert("pengepala".to_string(), Value::Kamus(Rc::new(RefCell::new(pengepala))));
    let hujah = [Value::Kamus(Rc::new(RefCell::new(kamus)))];

    let mut interp = Interpreter::new();
    let hasil = match pengendali {
        Value::Objek(o) => {
            let nama = if o.kaedah.contains_key("kendaliPermintaan")
                || o.sifat.borrow().contains_key("kendaliPermintaan")
            {
                "kendaliPermintaan"
            } else {
                "kendali"
            };
            interp.panggil_kaedah(pengendali.clone(), nama, &hujah, pos)
        }
        _ => interp.panggil_nilai(pengendali, &hujah, pos),
    };

    let ke_json = |v: &Value| nilai_ke_json_string(v, false, 0, pos).unwrap_or_else(|_| "{}".to_string());
    match hasil {
        Ok(Value::Kamus(rc)) => {
            let m = rc.borrow();
            let status = match m.get("status") {
                Some(Value::Nombor(n)) => *n as u16,
                _ => 200,
            };
            let jenis = match m.get("jenisKandungan") {
                Some(Value::Teks(s)) => s.clone(),
                _ => "application/json".to_string(),
            };
            let badan = match m.get("badan") {
                Some(Value::Teks(s)) => s.clone(),
                Some(lain) => ke_json(lain),
                None => String::new(),
            };
            bina_respons(status, &jenis, &badan)
        }
        Ok(Value::Teks(s)) => bina_respons(200, "text/plain", &s),
        Ok(lain) => bina_respons(200, "application/json", &ke_json(&lain)),
        Err(e) => {
            eprintln!("[Selasih Ralat Dalaman] {}", e.message);
            respons_ralat(500, "Ralat dalaman pelayan.")
        }
    }
}

fn layan_sambungan(
    mut aliran: TcpStream,
    hantar: &mpsc::Sender<(PermintaanMentah, mpsc::Sender<String>)>,
    sedang_tulis: &AtomicUsize,
) {
    let _ = aliran.set_write_timeout(Some(Duration::from_secs(5)));
    match baca_permintaan(&mut aliran) {
        Ok(None) => {}
        Err(respons) => {
            let _ = aliran.write_all(respons.as_bytes());
        }
        Ok(Some(permintaan)) => {
            let (balas, terima) = mpsc::channel();
            if hantar.send((permintaan, balas)).is_ok() {
                if let Ok(respons) = terima.recv() {
                    let _ = aliran.write_all(respons.as_bytes());
                    let _ = aliran.flush();
                    let _ = aliran.shutdown(Shutdown::Write);
                    sedang_tulis.fetch_sub(1, Ordering::SeqCst);
                }
            }
        }
    }
}

fn terima_sambungan(
    pendengar: TcpListener,
    hantar: mpsc::Sender<(PermintaanMentah, mpsc::Sender<String>)>,
    berhenti: Arc<AtomicBool>,
    sedang_tulis: Arc<AtomicUsize>,
) {
    let aktif = Arc::new(AtomicUsize::new(0));
    for aliran in pendengar.incoming() {
        if berhenti.load(Ordering::SeqCst) {
            break;
        }
        let Ok(mut aliran) = aliran else {
            std::thread::sleep(Duration::from_millis(10));
            continue;
        };
        if aktif.load(Ordering::SeqCst) >= MAKS_SAMBUNGAN {
            let _ = aliran.set_write_timeout(Some(Duration::from_millis(100)));
            let _ = aliran.write_all(respons_ralat(503, "Pelayan sibuk.").as_bytes());
            continue;
        }
        aktif.fetch_add(1, Ordering::SeqCst);
        let (hantar, kiraan, tulis) = (hantar.clone(), aktif.clone(), sedang_tulis.clone());
        let utas = std::thread::Builder::new().stack_size(256 * 1024).spawn(move || {
            layan_sambungan(aliran, &hantar, &tulis);
            kiraan.fetch_sub(1, Ordering::SeqCst);
        });
        if utas.is_err() {
            aktif.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

pub fn builtin_mulakan_pelayan_http(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let ralat = |mesej: &str| {
        MelayuError::new("Alamak!", mesej, pos.line, pos.col).with_kind(ErrorKind::Runtime)
    };
    if args.len() < 2 || args.len() > 3 {
        return Err(ralat("Fungsi 'mulakanPelayanHTTP' memerlukan 2 atau 3 hujah (port, fungsiPengendali, [hadPermintaan])."));
    }
    let port = match &args[0] {
        Value::Nombor(n) if n.fract() == 0.0 && (0.0..=65535.0).contains(n) => *n as u16,
        _ => return Err(ralat("Port mestilah nombor bulat antara 0 dan 65535.")),
    };
    let had = match args.get(2) {
        Some(Value::Nombor(n)) => *n as usize,
        _ => 0,
    };
    let pendengar = TcpListener::bind(("127.0.0.1", port)).map_err(|e| {
        ralat(&format!("Gagal memulakan pelayan HTTP pada port {}: {}", port, e))
    })?;
    let alamat = pendengar
        .local_addr()
        .map_err(|e| ralat(&format!("Gagal membaca alamat pelayan: {}", e)))?;

    let (hantar, terima) = mpsc::channel::<(PermintaanMentah, mpsc::Sender<String>)>();
    let berhenti = Arc::new(AtomicBool::new(false));
    let sedang_tulis = Arc::new(AtomicUsize::new(0));
    let (b, t) = (berhenti.clone(), sedang_tulis.clone());
    let penerima = std::thread::Builder::new()
        .spawn(move || terima_sambungan(pendengar, hantar, b, t))
        .map_err(|e| ralat(&format!("Gagal memulakan utas pelayan: {}", e)))?;

    let mut kiraan = 0usize;
    while let Ok((permintaan, balas)) = terima.recv() {
        let respons = kendali_permintaan(&args[1], permintaan, pos);
        sedang_tulis.fetch_add(1, Ordering::SeqCst);
        if balas.send(respons).is_err() {
            sedang_tulis.fetch_sub(1, Ordering::SeqCst);
        }
        kiraan += 1;
        if had > 0 && kiraan >= had {
            break;
        }
    }

    berhenti.store(true, Ordering::SeqCst);
    let _ = TcpStream::connect_timeout(&alamat, Duration::from_secs(1));
    let _ = penerima.join();
    let tamat = Instant::now() + Duration::from_secs(6);
    while sedang_tulis.load(Ordering::SeqCst) > 0 && Instant::now() < tamat {
        std::thread::sleep(Duration::from_millis(1));
    }
    Ok(Value::Nombor(kiraan as f64))
}

fn nyahkod_cebisan(badan: &[u8]) -> Option<Vec<u8>> {
    let mut hasil = Vec::new();
    let mut baki = badan;
    loop {
        let hujung = baki.windows(2).position(|w| w == b"\r\n")?;
        let baris = std::str::from_utf8(&baki[..hujung]).ok()?;
        let saiz = usize::from_str_radix(baris.split(';').next()?.trim(), 16).ok()?;
        baki = &baki[hujung + 2..];
        if saiz == 0 {
            return Some(hasil);
        }
        hasil.extend_from_slice(baki.get(..saiz)?);
        baki = baki.get(saiz..)?.strip_prefix(b"\r\n")?;
    }
}

fn pengepala_tersuai(nilai: Option<&Value>) -> Result<(bool, String), String> {
    let peta = match nilai {
        None | Some(Value::Kosong) => return Ok((false, String::new())),
        Some(Value::Kamus(peta)) => peta.borrow(),
        Some(lain) => return Err(format!("Pengepala HTTP mestilah kamus, tetapi dapat {}.", lain.jenis())),
    };
    let mut ada_jenis = false;
    let mut teks = String::new();
    for (nama, nilai) in peta.iter() {
        let nilai = match nilai {
            Value::Teks(t) => t.clone(),
            Value::Nombor(_) | Value::Betul | Value::Salah => nilai.to_string(),
            lain => {
                return Err(format!(
                    "Nilai pengepala HTTP '{}' mestilah teks atau nombor, bukan {}.",
                    nama.escape_debug(),
                    lain.jenis()
                ))
            }
        };
        let nama_sah = !nama.is_empty()
            && nama.bytes().all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b));
        if !nama_sah || nilai.chars().any(|c| c.is_control() && c != '\t') {
            return Err(format!("Pengepala HTTP '{}' mengandungi aksara yang tidak sah.", nama.escape_debug()));
        }
        match nama.to_ascii_lowercase().as_str() {
            "host" | "content-length" | "connection" | "transfer-encoding" => {
                return Err(format!("Pengepala HTTP '{}' ditetapkan secara automatik dan tidak boleh diubah.", nama));
            }
            "content-type" => ada_jenis = true,
            _ => {}
        }
        teks.push_str(&format!("{}: {}\r\n", nama, nilai));
    }
    Ok((ada_jenis, teks))
}

enum Aliran {
    Biasa(TcpStream),
    #[cfg(feature = "https")]
    Tls(Box<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>),
}

impl Read for Aliran {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Aliran::Biasa(s) => s.read(buf),
            #[cfg(feature = "https")]
            Aliran::Tls(s) => s.read(buf),
        }
    }
}

impl Write for Aliran {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Aliran::Biasa(s) => s.write(buf),
            #[cfg(feature = "https")]
            Aliran::Tls(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Aliran::Biasa(s) => s.flush(),
            #[cfg(feature = "https")]
            Aliran::Tls(s) => s.flush(),
        }
    }
}

#[cfg(feature = "https")]
fn konfigurasi_tls() -> Result<std::sync::Arc<rustls::ClientConfig>, String> {
    static KONFIGURASI: std::sync::OnceLock<std::sync::Arc<rustls::ClientConfig>> = std::sync::OnceLock::new();
    if let Some(k) = KONFIGURASI.get() {
        return Ok(k.clone());
    }
    let akar = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let konfigurasi = rustls::ClientConfig::builder_with_provider(std::sync::Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|e| format!("Konfigurasi TLS gagal: {}", e))?
    .with_root_certificates(akar)
    .with_no_client_auth();
    Ok(KONFIGURASI.get_or_init(|| std::sync::Arc::new(konfigurasi)).clone())
}

fn sambung_aliran(tcp: TcpStream, guna_tls: bool, host: &str) -> Result<Aliran, String> {
    if !guna_tls {
        return Ok(Aliran::Biasa(tcp));
    }
    #[cfg(feature = "https")]
    {
        let nama = rustls::pki_types::ServerName::try_from(host.to_string())
            .map_err(|_| format!("Nama hos '{}' tidak sah untuk TLS.", host))?;
        let mut sambungan = rustls::ClientConnection::new(konfigurasi_tls()?, nama)
            .map_err(|e| format!("Sambungan TLS gagal: {}", e))?;
        let mut tcp = tcp;
        while sambungan.is_handshaking() {
            sambungan
                .complete_io(&mut tcp)
                .map_err(|e| format!("Jabat tangan TLS dengan '{}' gagal: {}", host, e))?;
        }
        Ok(Aliran::Tls(Box::new(rustls::StreamOwned::new(sambungan, tcp))))
    }
    #[cfg(not(feature = "https"))]
    {
        let _ = (tcp, host);
        Err("HTTPS tidak tersedia dalam binaan ini.".to_string())
    }
}

fn ipv4_terbenam(v6: &Ipv6Addr) -> Vec<Ipv4Addr> {
    let o = v6.octets();
    let seg = v6.segments();
    let akhir = Ipv4Addr::new(o[12], o[13], o[14], o[15]);
    let mut hasil = Vec::new();
    if let Some(v4) = v6.to_ipv4_mapped() {
        hasil.push(v4);
    }
    if seg[..6] == [0; 6] && !v6.is_loopback() && !v6.is_unspecified() {
        hasil.push(akhir);
    }
    if seg[0] == 0x0064 && seg[1] == 0xff9b {
        hasil.push(akhir);
        hasil.push(Ipv4Addr::new(o[6], o[7], o[9], o[10]));
    }
    if seg[0] == 0x2002 {
        hasil.push(Ipv4Addr::new(o[2], o[3], o[4], o[5]));
    }
    if seg[0] == 0x2001 && seg[1] == 0 {
        hasil.push(Ipv4Addr::new(!o[12], !o[13], !o[14], !o[15]));
    }
    hasil
}

fn ipv4_disekat(v4: &Ipv4Addr) -> bool {
    v4.is_link_local() || v4.is_unspecified() || v4.octets() == [100, 100, 100, 200]
}

fn alamat_dalaman_disekat(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => ipv4_disekat(v4),
        IpAddr::V6(v6) => {
            let seg = v6.segments();
            v6.is_unspecified()
                || (seg[0] & 0xffc0) == 0xfe80
                || seg == [0xfd00, 0x0ec2, 0, 0, 0, 0, 0, 0x0254]
                || ipv4_terbenam(v6).iter().any(ipv4_disekat)
        }
    }
}

pub fn builtin_hantar_permintaan_http(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() < 2 || args.len() > 4 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'hantarPermintaanHTTP' memerlukan 2 hingga 4 hujah (kaedah, url, [badan, pengepala]).",
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }

    let kaedah = match &args[0] {
        Value::Teks(s) => s.to_uppercase(),
        _ => return Err(MelayuError::new("Alamak!", "Kaedah HTTP mestilah teks (cth: 'GET', 'POST').", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    if kaedah.contains('\r') || kaedah.contains('\n') || kaedah.contains(' ') {
        return Err(MelayuError::new("Alamak!", "Suntikan CRLF atau aksara tidak sah dikesan pada kaedah HTTP.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }

    let url_raw = match &args[1] {
        Value::Teks(s) => s.as_str(),
        _ => return Err(MelayuError::new("Alamak!", "URL mestilah teks.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };
    if url_raw.contains('\r') || url_raw.contains('\n') {
        return Err(MelayuError::new("Alamak!", "Suntikan CRLF dikesan pada URL HTTP.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }

    let badan_str = if args.len() >= 3 {
        match &args[2] {
            Value::Teks(s) => s.clone(),
            other => nilai_ke_json_string(other, false, 0, pos)?,
        }
    } else {
        "".to_string()
    };

    let ralat_url = |mesej: &str| {
        MelayuError::new("Alamak!", mesej, pos.line, pos.col).with_kind(ErrorKind::Runtime)
    };

    let (ada_jenis, pengepala) = pengepala_tersuai(args.get(3)).map_err(|e| ralat_url(&e))?;

    let (guna_tls, url_clean) = if let Some(baki) = url_raw.strip_prefix("http://") {
        (false, baki)
    } else if url_raw.get(..8).is_some_and(|awalan| awalan.eq_ignore_ascii_case("https://")) {
        (true, &url_raw[8..])
    } else if url_raw.contains("://") {
        return Err(ralat_url("Hanya URL http:// dan https:// disokong oleh hantarPermintaanHTTP."));
    } else {
        (false, url_raw)
    };
    if guna_tls && !cfg!(feature = "https") {
        return Err(ralat_url(
            "HTTPS tidak tersedia kerana MELAYU ini dibina tanpa ciri 'https'. Guna alamat http:// atau bina semula dengan ciri lalai.",
        ));
    }
    let port_lalai: u16 = if guna_tls { 443 } else { 80 };

    let (host_port, path) = if let Some(slash_idx) = url_clean.find('/') {
        (&url_clean[..slash_idx], &url_clean[slash_idx..])
    } else {
        (url_clean, "/")
    };

    let baca_port = |teks: &str| -> Result<u16, MelayuError> {
        teks.parse().map_err(|_| ralat_url("Nombor port URL tidak sah."))
    };

    let (host, port) = if let Some(baki) = host_port.strip_prefix('[') {
        let tutup = baki
            .find(']')
            .ok_or_else(|| ralat_url("Alamat IPv6 dalam URL tidak lengkap (tiada ']')."))?;
        let selepas = &baki[tutup + 1..];
        let p = match selepas.strip_prefix(':') {
            Some(teks_port) => baca_port(teks_port)?,
            None if selepas.is_empty() => port_lalai,
            None => return Err(ralat_url("Format hos URL tidak sah.")),
        };
        (&baki[..tutup], p)
    } else if let Some(colon_idx) = host_port.find(':') {
        (&host_port[..colon_idx], baca_port(&host_port[colon_idx + 1..])?)
    } else {
        (host_port, port_lalai)
    };

    let host_lower = host.to_lowercase();
    if host_lower == "metadata.google.internal" || host_lower == "metadata" {
        return Err(ralat_url(&format!(
            "Akses HTTP ke hos dalaman/metadata '{}' disekat (Anti-SSRF).",
            host
        )));
    }

    let senarai_alamat: Vec<SocketAddr> = (host, port)
        .to_socket_addrs()
        .map_err(|e| ralat_url(&format!("Gagal mencari alamat hos '{}': {}", host, e)))?
        .collect();
    if senarai_alamat.is_empty() {
        return Err(ralat_url(&format!("Hos '{}' tidak mempunyai alamat IP.", host)));
    }
    if let Some(disekat) = senarai_alamat.iter().find(|a| alamat_dalaman_disekat(&a.ip())) {
        return Err(ralat_url(&format!(
            "Akses HTTP ke hos dalaman/metadata '{}' ({}) disekat (Anti-SSRF).",
            host,
            disekat.ip()
        )));
    }

    let mut sambungan = None;
    let mut ralat_akhir = None;
    for alamat in &senarai_alamat {
        match TcpStream::connect_timeout(alamat, Duration::from_secs(10)) {
            Ok(s) => {
                sambungan = Some(s);
                break;
            }
            Err(e) => ralat_akhir = Some(e),
        }
    }
    let tcp = match sambungan {
        Some(s) => s,
        None => {
            return Err(ralat_url(&format!(
                "Gagal menyambung ke {}:{}: {}",
                host,
                port,
                ralat_akhir.map(|e| e.to_string()).unwrap_or_default()
            )))
        }
    };

    let _ = tcp.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = tcp.set_write_timeout(Some(Duration::from_secs(10)));
    let mut stream = sambung_aliran(tcp, guna_tls, host).map_err(|e| ralat_url(&e))?;

    let req_content = format!(
        "{} {} HTTP/1.1\r\nHost: {}:{}\r\n{}{}Content-Length: {}\r\nConnection: close\r\n\r\n{}",
        kaedah,
        path,
        if host.contains(':') { format!("[{}]", host) } else { host.to_string() },
        port,
        if ada_jenis { "" } else { "Content-Type: application/json\r\n" },
        pengepala,
        badan_str.len(),
        badan_str
    );

    if let Err(e) = stream.write_all(req_content.as_bytes()) {
        return Err(MelayuError::new("Alamak!", &format!("Gagal menghantar permintaan HTTP: {}", e), pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let _ = stream.flush();

    let mut response_bytes = Vec::new();
    let _ = stream.take(5 * 1024 * 1024).read_to_end(&mut response_bytes);

    let (bait_pengepala, bait_badan) = match response_bytes.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(i) => (&response_bytes[..i], &response_bytes[i + 4..]),
        None => (&response_bytes[..], &[][..]),
    };
    let header_part = String::from_utf8_lossy(bait_pengepala).to_string();

    let mut header_lines = header_part.lines();
    let status_line = header_lines.next().unwrap_or("");
    let status_code: f64 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let mut headers_resp = PetaNilai::new();
    for hl in header_lines {
        if let Some((k, v)) = hl.split_once(':') {
            headers_resp.insert(k.trim().to_lowercase(), Value::Teks(v.trim().to_string()));
        }
    }
    let bercebisan = matches!(
        headers_resp.get("transfer-encoding"),
        Some(Value::Teks(t)) if t.to_lowercase().contains("chunked")
    );
    let panjang_dijangka = match headers_resp.get("content-length") {
        Some(Value::Teks(t)) => t.parse::<usize>().ok(),
        _ => None,
    };
    let tanpa_badan = kaedah == "HEAD" || status_code < 200.0 || status_code == 204.0 || status_code == 304.0;
    let isi = if tanpa_badan {
        Some(bait_badan.to_vec())
    } else if bercebisan {
        nyahkod_cebisan(bait_badan)
    } else if panjang_dijangka.is_some_and(|n| bait_badan.len() < n) {
        None
    } else {
        Some(bait_badan.to_vec())
    };
    let body_part = match isi {
        Some(bait) => String::from_utf8_lossy(&bait).to_string(),
        None => return Err(ralat_url("Respons HTTP terputus sebelum lengkap atau melebihi had 5 MB.")),
    };

    let mut map_resp = PetaNilai::new();
    map_resp.insert("status".to_string(), Value::Nombor(status_code));
    map_resp.insert("badan".to_string(), Value::Teks(body_part));
    map_resp.insert("pengepala".to_string(), Value::Kamus(Rc::new(RefCell::new(headers_resp))));

    Ok(Value::Kamus(Rc::new(RefCell::new(map_resp))))
}

// ==============================================================================
// PRIMITIF TENSOR NATIVE & DEEP LEARNING LANJUTAN
// ==============================================================================

fn ekstrak_bentuk_tensor(val: &Value, pos: LineCol) -> Result<Vec<usize>, MelayuError> {
    match val {
        Value::Senarai(rc) => {
            let mut dims = Vec::new();
            let mut total: usize = 1;
            for item in rc.borrow().iter() {
                match item {
                    Value::Nombor(n) if n.is_finite() && *n > 0.0 && *n <= (HAD_ELEMEN_TENSOR_MAKSIMUM as f64) => {
                        let d = *n as usize;
                        total = total.checked_mul(d).ok_or_else(|| {
                            MelayuError::had_memori_terlampau(
                                "Saiz dimensi tensor melimpah (overflow).",
                                pos.line,
                                pos.col,
                            )
                        })?;
                        if total > HAD_ELEMEN_TENSOR_MAKSIMUM {
                            return Err(MelayuError::had_memori_terlampau(
                                "Jumlah elemen tensor melebihi had keselamatan maksimum (1,000,000).",
                                pos.line,
                                pos.col,
                            ));
                        }
                        dims.push(d);
                    }
                    _ => {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Setiap dimensi bentuk tensor mestilah nombor positif yang sah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                }
            }
            if dims.is_empty() || dims.len() > 2 {
                return Err(MelayuError::new(
                    "Alamak!",
                    "Bentuk tensor kini menyokong 1 atau 2 dimensi sahaja.",
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime));
            }
            Ok(dims)
        }
        _ => Err(MelayuError::new(
            "Alamak!",
            "Bentuk tensor mestilah senarai dimensi, contoh: [2, 3].",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_tensor(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'tensor' memerlukan 1 hujah (senarai 1D atau 2D).",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    match &args[0] {
        Value::Senarai(rc) => {
            let borrowed = rc.borrow();
            if borrowed.is_empty() {
                return Ok(Value::Tensor {
                    data: Rc::new(vec![]),
                    shape: vec![0],
                });
            }
            if matches!(borrowed[0], Value::Senarai(_)) {
                let mut data = Vec::new();
                let rows = borrowed.len();
                let mut cols = None;
                for row_val in borrowed.iter() {
                    let row = match row_val {
                        Value::Senarai(r) => r.borrow(),
                        _ => {
                            return Err(MelayuError::new(
                                "Alamak!",
                                "Semua baris tensor 2D mestilah senarai.",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    };
                    match cols {
                        None => cols = Some(row.len()),
                        Some(c) if c != row.len() => {
                            return Err(MelayuError::new(
                                "Alamak!",
                                "Semua baris tensor 2D mestilah sama panjang.",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                        _ => {}
                    }
                    for v in row.iter() {
                        match v {
                            Value::Nombor(n) => data.push(*n),
                            _ => {
                                return Err(MelayuError::new(
                                    "Alamak!",
                                    "Semua elemen tensor mestilah nombor.",
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                        }
                    }
                }
                Ok(Value::Tensor {
                    data: Rc::new(data),
                    shape: vec![rows, cols.unwrap_or(0)],
                })
            } else {
                let mut data = Vec::with_capacity(borrowed.len());
                for v in borrowed.iter() {
                    match v {
                        Value::Nombor(n) => data.push(*n),
                        _ => {
                            return Err(MelayuError::new(
                                "Alamak!",
                                "Semua elemen tensor mestilah nombor.",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    }
                }
                let len = data.len();
                Ok(Value::Tensor {
                    data: Rc::new(data),
                    shape: vec![len],
                })
            }
        }
        Value::Tensor { data, shape } => Ok(Value::Tensor {
            data: data.clone(),
            shape: shape.clone(),
        }),
        _ => Err(MelayuError::new(
            "Alamak!",
            "Hujah 'tensor()' mestilah senarai nombor 1D atau 2D.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)),
    }
}

pub fn builtin_sifar_tensor(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'sifarTensor' memerlukan 1 hujah (senarai bentuk).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let shape = ekstrak_bentuk_tensor(&args[0], pos)?;
    let total: usize = shape.iter().product();
    Ok(Value::Tensor {
        data: Rc::new(vec![0.0; total]),
        shape,
    })
}

pub fn builtin_satu_tensor(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'satuTensor' memerlukan 1 hujah (senarai bentuk).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let shape = ekstrak_bentuk_tensor(&args[0], pos)?;
    let total: usize = shape.iter().product();
    Ok(Value::Tensor {
        data: Rc::new(vec![1.0; total]),
        shape,
    })
}

pub fn builtin_rawak_tensor(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'rawakTensor' memerlukan 1 hujah (senarai bentuk).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let shape = ekstrak_bentuk_tensor(&args[0], pos)?;
    let total: usize = shape.iter().product();
    let mut data = Vec::with_capacity(total);
    for _ in 0..total {
        let u = ((jana_nombor_rawak_u64() >> 11) as f64) / ((1u64 << 53) as f64);
        data.push(u * 2.0 - 1.0); // julat [-1.0, 1.0]
    }
    Ok(Value::Tensor { data: Rc::new(data), shape })
}

pub fn builtin_matriks_softmax(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksSoftmax' memerlukan 1 hujah matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mat = ekstrak_matriks(&args[0], pos)?;
    let mut out = Vec::with_capacity(mat.len());
    for row in mat {
        let max_v = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = row.iter().map(|&x| (x - max_v).exp()).collect();
        let sum: f64 = exps.iter().sum();
        let probs: Vec<Value> = exps
            .into_iter()
            .map(|e| Value::Nombor(if sum > 0.0 { e / sum } else { 0.0 }))
            .collect();
        out.push(Value::senarai(probs));
    }
    Ok(Value::senarai(out))
}

pub fn builtin_matriks_relu(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 1 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'matriksReLU' memerlukan 1 hujah matriks.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mat = ekstrak_matriks(&args[0], pos)?;
    let out: Vec<Value> = mat
        .into_iter()
        .map(|row| {
            let r: Vec<Value> = row
                .into_iter()
                .map(|x| Value::Nombor(if x > 0.0 { x } else { 0.0 }))
                .collect();
            Value::senarai(r)
        })
        .collect();
    Ok(Value::senarai(out))
}

pub fn builtin_ralat_entropi_silang(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() != 2 {
        return Err(MelayuError::new("Alamak!", "Fungsi 'ralatEntropiSilang' memerlukan 2 hujah (y_benar_matriks, y_ramal_matriks).", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let y_true = ekstrak_matriks(&args[0], pos)?;
    let y_pred = ekstrak_matriks(&args[1], pos)?;
    if y_true.len() != y_pred.len() || y_true.is_empty() {
        return Err(MelayuError::new("Alamak!", "Saiz matriks y_benar dan y_ramal mestilah sama dan tidak kosong.", pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let mut total_loss = 0.0;
    let eps = 1e-12;
    for (r_t, r_p) in y_true.iter().zip(y_pred.iter()) {
        for (&yt, &yp) in r_t.iter().zip(r_p.iter()) {
            if yt > 0.0 {
                let p_clamped = yp.clamp(eps, 1.0 - eps);
                total_loss -= yt * p_clamped.ln();
            }
        }
    }
    Ok(Value::Nombor(total_loss / (y_true.len() as f64)))
}

pub fn builtin_info_perkakas(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if !args.is_empty() {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'infoPerkakas' tidak memerlukan sebarang hujah.",
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime));
    }
    let teras_cpu = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let ada_cuda = std::path::Path::new("/usr/local/cuda").exists()
        || std::path::Path::new("/dev/nvidia0").exists();
    let ada_rocm = std::path::Path::new("/opt/rocm").exists();

    let gpu_kind = crate::gpu::gpu_kind();
    let gpu_aktif = gpu_kind != crate::gpu::GpuKind::None;

    let mut map = PetaNilai::new();
    map.insert("teras_cpu".to_string(), Value::Nombor(teras_cpu as f64));
    map.insert(
        "enjin_matriks".to_string(),
        Value::Teks(if gpu_aktif {
            format!("{} + CPU Selari Multi-Teras", gpu_kind.nama())
        } else {
            "CPU Selari Multi-Teras (Rust std::thread::scope)".to_string()
        }),
    );
    map.insert(
        "gpu_cuda_dikesan".to_string(),
        if ada_cuda || gpu_kind == crate::gpu::GpuKind::Cuda { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "gpu_rocm_dikesan".to_string(),
        if ada_rocm || gpu_kind == crate::gpu::GpuKind::Rocm { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "gpu_aktif".to_string(),
        if gpu_aktif { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "ketepatan_gpu".to_string(),
        Value::Teks(if gpu_aktif { crate::gpu::ketepatan() } else { "tiada" }.to_string()),
    );
    map.insert(
        "status_gpu".to_string(),
        Value::Teks(if gpu_aktif {
            format!("Aktif pada perkakasan: {}", gpu_kind.nama())
        } else {
            "Fallback ke CPU selari multi-teras".to_string()
        }),
    );

    Ok(Value::Kamus(Rc::new(RefCell::new(map))))
}

pub const AKAL_ML_STDLIB: &str = include_str!("../std/akal.ml");
pub const SELASIH_STDLIB: &str = include_str!("../std/selasih.ml");
pub const PANGKALAN_STDLIB: &str = include_str!("../std/pangkalan.ml");

fn banding_nan_akhir(a: f64, b: f64) -> std::cmp::Ordering {
    a.is_nan().cmp(&b.is_nan()).then(a.total_cmp(&b))
}

pub fn builtin_cari_pecahan_terbaik(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let ralat = |mesej: &str| {
        MelayuError::new("Alamak!", mesej, pos.line, pos.col).with_kind(ErrorKind::Runtime)
    };
    if args.len() != 3 && args.len() != 4 {
        return Err(ralat("Fungsi 'cariPecahanTerbaik' memerlukan 3 atau 4 hujah (X, y, senaraiCiri, [maksCiri])."));
    }
    let maks_ciri = match args.get(3) {
        None | Some(Value::Kosong) => usize::MAX,
        Some(Value::Nombor(n)) if *n >= 1.0 => *n as usize,
        _ => return Err(ralat("maksCiri mestilah nombor bulat 1 atau lebih.")),
    };
    let (Value::Senarai(x_rc), Value::Senarai(y_rc), Value::Senarai(ciri_rc)) = (&args[0], &args[1], &args[2]) else {
        return Err(ralat("Hujah 'cariPecahanTerbaik' mestilah senarai (X, y, senaraiCiri)."));
    };
    let x_senarai = x_rc.borrow();
    let y_senarai = y_rc.borrow();
    let n = x_senarai.len();
    if n != y_senarai.len() {
        return Err(ralat("Bilangan baris X dan y mestilah sama."));
    }
    if n < 2 {
        return Ok(Value::Kosong);
    }

    let mut label_unik: Vec<String> = Vec::new();
    let mut indeks_label = Vec::with_capacity(n);
    for v in y_senarai.iter() {
        let teks_label = v.ke_teks(pos)?;
        let i = match label_unik.iter().position(|l| *l == teks_label) {
            Some(i) => i,
            None => {
                label_unik.push(teks_label);
                label_unik.len() - 1
            }
        };
        indeks_label.push(i);
    }
    let k = label_unik.len();
    let mut jumlah = vec![0usize; k];
    for &c in &indeks_label {
        jumlah[c] += 1;
    }

    let baris: Vec<std::cell::Ref<'_, SenaraiNilai>> = x_senarai
        .iter()
        .map(|b| match b {
            Value::Senarai(r) => Ok(r.borrow()),
            _ => Err(ralat("Setiap baris X mestilah senarai nombor.")),
        })
        .collect::<Result<_, _>>()?;

    let gini = |kiraan: &[usize], jumlah_n: usize| -> f64 {
        if jumlah_n == 0 {
            return 0.0;
        }
        let nf = jumlah_n as f64;
        1.0 - kiraan.iter().map(|&c| (c as f64 / nf).powi(2)).sum::<f64>()
    };

    let mut terbaik: Option<(f64, usize, f64)> = None;
    let mut pasangan: Vec<(f64, usize)> = Vec::with_capacity(n);
    let mut ada_ciri_berubah = false;
    for (dilawati, c) in ciri_rc.borrow().iter().enumerate() {
        if dilawati >= maks_ciri && ada_ciri_berubah {
            break;
        }
        let f = match c {
            Value::Nombor(f) if *f >= 0.0 && f.fract() == 0.0 => *f as usize,
            _ => return Err(ralat("senaraiCiri mestilah senarai indeks lajur (nombor bulat).")),
        };
        pasangan.clear();
        for (i, b) in baris.iter().enumerate() {
            match b.get(f) {
                Some(Value::Nombor(v)) => {
                    pasangan.push((if v.is_nan() { f64::NAN } else { *v }, indeks_label[i]))
                }
                _ => return Err(ralat("Nilai ciri dalam X mestilah nombor.")),
            }
        }
        pasangan.sort_by(|a, b| banding_nan_akhir(a.0, b.0));
        if pasangan[0].0 == pasangan[n - 1].0 || pasangan[0].0.is_nan() {
            continue;
        }
        ada_ciri_berubah = true;
        let mut kiri = vec![0usize; k];
        let mut kanan = jumlah.clone();
        for i in 0..n - 1 {
            let (v, c) = pasangan[i];
            kiri[c] += 1;
            kanan[c] -= 1;
            let v_seterusnya = pasangan[i + 1].0;
            if v == v_seterusnya || v.is_nan() {
                continue;
            }
            let tengah = v / 2.0 + v_seterusnya / 2.0;
            let ambang = if v <= tengah && tengah < v_seterusnya { tengah } else { v };
            let n_kiri = i + 1;
            let n_kanan = n - n_kiri;
            let skor = (n_kiri as f64 * gini(&kiri, n_kiri) + n_kanan as f64 * gini(&kanan, n_kanan)) / n as f64;
            if !matches!(terbaik, Some((s, _, _)) if s <= skor) {
                terbaik = Some((skor, f, ambang));
            }
        }
    }

    Ok(match terbaik {
        Some((skor, ciri, ambang)) => {
            let mut peta = PetaNilai::new();
            peta.insert("ciri".to_string(), Value::Nombor(ciri as f64));
            peta.insert("ambang".to_string(), Value::Nombor(ambang));
            peta.insert("skor".to_string(), Value::Nombor(skor));
            Value::Kamus(Rc::new(RefCell::new(peta)))
        }
        None => Value::Kosong,
    })
}
