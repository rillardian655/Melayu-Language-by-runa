use crate::ast::LineCol;
use crate::errors::{ErrorKind, MelayuError};
use crate::interpreter::{Interpreter, PetaNilai, Value};
use std::cell::RefCell;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    Ok(Value::Teks(args[0].to_string()))
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

static AKAR_SANDBOX: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

pub fn tetapkan_akar_sandbox(akar: PathBuf) {
    let _ = AKAR_SANDBOX.set(akar.canonicalize().unwrap_or(akar));
}

pub fn semak_laluan_selamat(
    laluan_str: &str,
    untuk_menulis: bool,
    pos: LineCol,
) -> Result<PathBuf, MelayuError> {
    if laluan_str.trim().is_empty() {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            "Nama laluan fail tidak boleh kosong.",
            pos.line,
            pos.col,
        ));
    }
    if laluan_str.contains('\0') {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            "Aksara batal (null byte) dikesan dalam laluan fail.",
            pos.line,
            pos.col,
        ));
    }

    let p = Path::new(laluan_str);
    for comp in p.components() {
        if matches!(comp, Component::ParentDir) {
            return Err(MelayuError::akses_fail_disekat(
                laluan_str,
                "Komponen laluan '..' dilarang demi keselamatan sandbox persekitaran fail.",
                pos.line,
                pos.col,
            ));
        }
        if let Component::Normal(os_str) = comp {
            let s = os_str.to_string_lossy().to_lowercase();
            if s == ".git" || s == ".env" || s == ".ssh" || s == ".gemini" || s.starts_with(".env.") {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    &format!("Akses ke fail/direktori sensitif '{}' disekat demi keselamatan.", s),
                    pos.line,
                    pos.col,
                ));
            }
        }
    }

    let base = std::env::current_dir().map_err(|e| {
        MelayuError::akses_fail_disekat(
            laluan_str,
            &format!("Gagal mendapatkan direktori kerja semasa: {}", e),
            pos.line,
            pos.col,
        )
    })?;
    let can_base = base.canonicalize().unwrap_or(base);
    let akar = AKAR_SANDBOX.get().cloned().unwrap_or_else(|| can_base.clone());
    let di_luar = format!("Laluan ini berada di luar sandbox projek ({}).", akar.display());

    if p.is_absolute() {
        if p.exists() {
            let can_target = p.canonicalize().map_err(|e| {
                MelayuError::akses_fail_disekat(
                    laluan_str,
                    &format!("Laluan mutlak tidak sah: {}", e),
                    pos.line,
                    pos.col,
                )
            })?;
            if !can_target.starts_with(&akar) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    &di_luar,
                    pos.line,
                    pos.col,
                ));
            }
            Ok(can_target)
        } else {
            if p.symlink_metadata().is_ok() {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    "Pautan simbolik yang rosak (sasaran tidak wujud) tidak dibenarkan.",
                    pos.line,
                    pos.col,
                ));
            }
            if !p.starts_with(&akar) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    &di_luar,
                    pos.line,
                    pos.col,
                ));
            }
            Ok(p.to_path_buf())
        }
    } else {
        let combined = akar.join(p);
        if combined.exists() {
            let can_target = combined.canonicalize().map_err(|e| {
                MelayuError::akses_fail_disekat(
                    laluan_str,
                    &format!("Laluan fail tidak sah: {}", e),
                    pos.line,
                    pos.col,
                )
            })?;
            if !can_target.starts_with(&akar) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    &di_luar,
                    pos.line,
                    pos.col,
                ));
            }
            Ok(can_target)
        } else {
            if combined.symlink_metadata().is_ok() {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    "Pautan simbolik yang rosak (sasaran tidak wujud) tidak dibenarkan.",
                    pos.line,
                    pos.col,
                ));
            }
            if untuk_menulis {
                if let Some(parent) = combined.parent() {
                    if parent.exists() {
                        let can_parent = parent.canonicalize().map_err(|e| {
                            MelayuError::akses_fail_disekat(
                                laluan_str,
                                &format!("Direktori induk tidak sah: {}", e),
                                pos.line,
                                pos.col,
                            )
                        })?;
                        if !can_parent.starts_with(&akar) {
                            return Err(MelayuError::akses_fail_disekat(
                                laluan_str,
                                &di_luar,
                                pos.line,
                                pos.col,
                            ));
                        }
                    }
                }
            }
            Ok(combined)
        }
    }
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

    let safe_path = semak_laluan_selamat(laluan_str, false, pos)?;
    if !safe_path.exists() {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            "Fail tidak dijumpai.",
            pos.line,
            pos.col,
        ));
    }

    let meta = fs::metadata(&safe_path).map_err(|e| {
        MelayuError::akses_fail_disekat(
            laluan_str,
            &format!("Gagal membaca metadata fail: {}", e),
            pos.line,
            pos.col,
        )
    })?;

    if meta.len() > HAD_SAIZ_FAIL_MAKSIMUM {
        return Err(MelayuError::akses_fail_disekat(
            laluan_str,
            &format!(
                "Saiz fail ({} bait) melebihi had keselamatan 5 MB.",
                meta.len()
            ),
            pos.line,
            pos.col,
        ));
    }

    let content = fs::read_to_string(&safe_path).map_err(|e| {
        MelayuError::akses_fail_disekat(
            laluan_str,
            &format!("Gagal membaca kandungan fail teks: {}", e),
            pos.line,
            pos.col,
        )
    })?;

    Ok(Value::Teks(content))
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

    let safe_path = semak_laluan_selamat(laluan_str, true, pos)?;

    fs::write(&safe_path, kandungan).map_err(|e| {
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

    let safe_path = semak_laluan_selamat(laluan_str, false, pos)?;
    Ok(if safe_path.exists() {
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

    let safe_path = semak_laluan_selamat(laluan_str, false, pos)?;
    if safe_path.exists() {
        fs::remove_file(&safe_path).map_err(|e| {
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
            return Ok(Value::Senarai(Rc::new(RefCell::new(list))));
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
        Ok(Value::Senarai(Rc::new(RefCell::new(list))))
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
    if kedalaman > 500 {
        return Err(MelayuError::had_memori_terlampau(
            "Kedalaman struktur penukaran ke JSON melebihi had keselamatan (500)",
            pos.line,
            pos.col,
        ));
    }

    match val {
        Value::Kosong => Ok("null".to_string()),
        Value::Betul => Ok("true".to_string()),
        Value::Salah => Ok("false".to_string()),
        Value::Nombor(n) => {
            if n.is_nan() || n.is_infinite() {
                Ok("null".to_string())
            } else if n.fract() == 0.0 && n.abs() < 9_007_199_254_740_992.0 {
                Ok((*n as i64).to_string())
            } else {
                Ok(n.to_string())
            }
        }
        Value::Teks(s) => Ok(escape_json_string(s)),
        Value::Senarai(list) => {
            let borrowed = list.borrow();
            if borrowed.is_empty() {
                return Ok("[]".to_string());
            }

            if cantik {
                let indent_item = "  ".repeat(kedalaman + 1);
                let indent_close = "  ".repeat(kedalaman);
                let mut parts = Vec::new();
                for item in borrowed.iter() {
                    let s = nilai_ke_json_string(item, true, kedalaman + 1, pos)?;
                    parts.push(format!("{}{}", indent_item, s));
                }
                Ok(format!("[\n{}\n{}]", parts.join(",\n"), indent_close))
            } else {
                let mut parts = Vec::new();
                for item in borrowed.iter() {
                    parts.push(nilai_ke_json_string(item, false, kedalaman + 1, pos)?);
                }
                Ok(format!("[{}]", parts.join(",")))
            }
        }
        Value::Kamus(map) => {
            let borrowed = map.borrow();
            if borrowed.is_empty() {
                return Ok("{}".to_string());
            }

            let keys: Vec<&String> = borrowed.keys().collect();

            if cantik {
                let indent_item = "  ".repeat(kedalaman + 1);
                let indent_close = "  ".repeat(kedalaman);
                let mut parts = Vec::new();
                for k in keys {
                    let v = borrowed.get(k).unwrap();
                    let v_str = nilai_ke_json_string(v, true, kedalaman + 1, pos)?;
                    let k_escaped = escape_json_string(k);
                    parts.push(format!("{}{}: {}", indent_item, k_escaped, v_str));
                }
                Ok(format!("{{\n{}\n{}}}", parts.join(",\n"), indent_close))
            } else {
                let mut parts = Vec::new();
                for k in keys {
                    let v = borrowed.get(k).unwrap();
                    let v_str = nilai_ke_json_string(v, false, kedalaman + 1, pos)?;
                    let k_escaped = escape_json_string(k);
                    parts.push(format!("{}:{}", k_escaped, v_str));
                }
                Ok(format!("{{{}}}", parts.join(",")))
            }
        }
        Value::Objek(o) => {
            let borrowed = o.sifat.borrow();
            if borrowed.is_empty() {
                return Ok("{}".to_string());
            }

            let keys: Vec<&String> = borrowed.keys().collect();

            if cantik {
                let indent_item = "  ".repeat(kedalaman + 1);
                let indent_close = "  ".repeat(kedalaman);
                let mut parts = Vec::new();
                for k in keys {
                    let v = borrowed.get(k).unwrap();
                    let v_str = nilai_ke_json_string(v, true, kedalaman + 1, pos)?;
                    let k_escaped = escape_json_string(k);
                    parts.push(format!("{}{}: {}", indent_item, k_escaped, v_str));
                }
                Ok(format!("{{\n{}\n{}}}", parts.join(",\n"), indent_close))
            } else {
                let mut parts = Vec::new();
                for k in keys {
                    let v = borrowed.get(k).unwrap();
                    let v_str = nilai_ke_json_string(v, false, kedalaman + 1, pos)?;
                    let k_escaped = escape_json_string(k);
                    parts.push(format!("{}:{}", k_escaped, v_str));
                }
                Ok(format!("{{{}}}", parts.join(",")))
            }
        }
        Value::Tensor { data, shape } => {
            if shape.len() == 2 {
                let (rows, cols) = (shape[0], shape[1]);
                let mut row_strs = Vec::with_capacity(rows);
                for r in 0..rows {
                    let col_strs: Vec<String> = (0..cols)
                        .map(|c| {
                            let v = data[r * cols + c];
                            if v.fract() == 0.0 && v.is_finite() {
                                format!("{:.0}", v)
                            } else {
                                format!("{}", v)
                            }
                        })
                        .collect();
                    row_strs.push(format!("[{}]", col_strs.join(",")));
                }
                Ok(format!("[{}]", row_strs.join(",")))
            } else {
                let strs: Vec<String> = data
                    .iter()
                    .map(|&v| {
                        if v.fract() == 0.0 && v.is_finite() {
                            format!("{:.0}", v)
                        } else {
                            format!("{}", v)
                        }
                    })
                    .collect();
                Ok(format!("[{}]", strs.join(",")))
            }
        }
        other => Err(MelayuError::new(
            "Alamak!",
            &format!(
                "Jenis '{}' tidak boleh ditukar ke format JSON.",
                other.jenis()
            ),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime)),
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
        Value::Senarai(Rc::new(RefCell::new(cols)))
    }).collect();
    Value::Senarai(Rc::new(RefCell::new(rows)))
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
        return Ok(Value::Senarai(Rc::new(RefCell::new(Vec::new()))));
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
        return Ok(Value::Senarai(Rc::new(RefCell::new(Vec::new()))));
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

    let safe_path = semak_laluan_selamat(&path_str, false, pos)?;
    if let Ok(meta) = fs::metadata(&safe_path) {
        if meta.len() > HAD_SAIZ_FAIL_MAKSIMUM {
            return Err(MelayuError::had_memori_terlampau(
                "Saiz fail CSV melebihi had keselamatan maksimum (5 MB).",
                pos.line,
                pos.col,
            ));
        }
    }
    let content = fs::read_to_string(&safe_path).map_err(|e| {
        MelayuError::akses_fail_disekat(&path_str, &format!("Gagal membaca fail CSV: {}", e), pos.line, pos.col)
    })?;

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
        data_rows.push(Value::Senarai(Rc::new(RefCell::new(row_vals))));
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
    map.insert("lajur".to_string(), Value::Senarai(Rc::new(RefCell::new(columns))));
    map.insert("data".to_string(), Value::Senarai(Rc::new(RefCell::new(data_rows))));
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

    Ok(Value::Senarai(Rc::new(RefCell::new(vec![
        Value::Senarai(Rc::new(RefCell::new(x_train))),
        Value::Senarai(Rc::new(RefCell::new(x_test))),
        Value::Senarai(Rc::new(RefCell::new(y_train))),
        Value::Senarai(Rc::new(RefCell::new(y_test))),
    ]))))
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

pub fn builtin_mulakan_pelayan_http(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(MelayuError::new(
            "Alamak!",
            "Fungsi 'mulakanPelayanHTTP' memerlukan 2 atau 3 hujah (port, fungsiPengendali, [hadPermintaan]).",
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime));
    }

    let port = match &args[0] {
        Value::Nombor(n) => *n as u16,
        _ => return Err(MelayuError::new("Alamak!", "Port mestilah nombor bulat.", pos.line, pos.col).with_kind(ErrorKind::Runtime)),
    };

    let handler_val = args[1].clone();

    let had_permintaan = if args.len() == 3 {
        match &args[2] {
            Value::Nombor(n) => *n as usize,
            _ => 0,
        }
    } else {
        0
    };

    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(l) => l,
        Err(e) => return Err(MelayuError::new(
            "Alamak!",
            &format!("Gagal memulakan pelayan HTTP pada port {}: {}", port, e),
            pos.line,
            pos.col,
        ).with_kind(ErrorKind::Runtime)),
    };

    let mut count = 0;

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

        // Baca pengepala dan badan secara berperingkat sehingga Content-Length (had 1 MB)
        let mut raw_bytes = Vec::with_capacity(4096);
        let mut temp_buf = [0u8; 4096];
        let mut header_end_idx: Option<usize> = None;
        let mut expected_content_len: usize = 0;

        loop {
            match stream.read(&mut temp_buf) {
                Ok(0) => break,
                Ok(n) => {
                    raw_bytes.extend_from_slice(&temp_buf[..n]);
                    if header_end_idx.is_none() {
                        if let Some(pos_hdr) = raw_bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                            header_end_idx = Some(pos_hdr + 4);
                            let hdr_str = String::from_utf8_lossy(&raw_bytes[..pos_hdr]);
                            for hl in hdr_str.lines().skip(1) {
                                if let Some((k, v)) = hl.split_once(':') {
                                    if k.trim().eq_ignore_ascii_case("content-length") {
                                        expected_content_len = v.trim().parse().unwrap_or(0);
                                    }
                                }
                            }
                            if expected_content_len > 1_048_576 {
                                break;
                            }
                        } else if raw_bytes.len() > 16_384 {
                            break; // Had saiz pengepala 16 KB
                        }
                    }
                    if let Some(hdr_end) = header_end_idx {
                        if raw_bytes.len() >= hdr_end + expected_content_len {
                            break;
                        }
                    }
                    if raw_bytes.len() > 1_064_960 {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        if raw_bytes.is_empty() {
            continue;
        }

        let req_text = String::from_utf8_lossy(&raw_bytes);
        let mut lines = req_text.lines();
        let first_line = lines.next().unwrap_or("");
        let mut first_parts = first_line.split_whitespace();
        let method = first_parts.next().unwrap_or("GET");
        let path_full = first_parts.next().unwrap_or("/");

        let mut headers_map = PetaNilai::new();
        let mut content_length: usize = 0;

        for line in lines.by_ref() {
            if line.is_empty() || line == "\r" {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                let k_trim = k.trim().to_lowercase();
                let v_trim = v.trim().to_string();
                if k_trim == "content-length" {
                    content_length = v_trim.parse().unwrap_or(0);
                }
                headers_map.insert(k_trim, Value::Teks(v_trim));
            }
        }

        if content_length > 1_048_576 {
            let err_body = "{\"ralat\": \"Saiz muatan melebihi had keselamatan (1 MB)\"}";
            let resp = format!(
                "HTTP/1.1 413 Payload Too Large\r\nContent-Type: application/json\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n{}",
                err_body.len(),
                err_body
            );
            let _ = stream.write_all(resp.as_bytes());
            let _ = stream.flush();
            continue;
        }

        let body = if let Some(idx) = req_text.find("\r\n\r\n") {
            let body_part = &req_text[idx + 4..];
            if content_length > 0 && body_part.len() > content_length {
                &body_part[..content_length]
            } else {
                body_part
            }
        } else {
            ""
        };

        let mut req_kamus = PetaNilai::new();
        req_kamus.insert("kaedah".to_string(), Value::Teks(method.to_string()));
        req_kamus.insert("laluan".to_string(), Value::Teks(path_full.to_string()));
        req_kamus.insert("badan".to_string(), Value::Teks(body.to_string()));
        req_kamus.insert("pengepala".to_string(), Value::Kamus(Rc::new(RefCell::new(headers_map))));

        let req_val = Value::Kamus(Rc::new(RefCell::new(req_kamus)));

        let mut interp = Interpreter::new();
        let res_val = match &handler_val {
            Value::Objek(_) => {
                interp.panggil_kaedah(handler_val.clone(), "kendaliPermintaan", &[req_val.clone()], pos)
                    .or_else(|_| interp.panggil_kaedah(handler_val.clone(), "kendali", &[req_val], pos))
            }
            _ => interp.panggil_nilai(&handler_val, &[req_val], pos),
        };

        let (status_code, content_type, res_body) = match res_val {
            Ok(Value::Kamus(rc)) => {
                let m = rc.borrow();
                let status = match m.get("status") {
                    Some(Value::Nombor(n)) => *n as u16,
                    _ => 200,
                };
                let c_type = match m.get("jenisKandungan") {
                    Some(Value::Teks(s)) => s.clone(),
                    _ => "application/json".to_string(),
                };
                let b_str = match m.get("badan") {
                    Some(Value::Teks(s)) => s.clone(),
                    Some(other) => nilai_ke_json_string(other, false, 0, pos).unwrap_or_else(|_| "{}".to_string()),
                    None => "".to_string(),
                };
                (status, c_type, b_str)
            }
            Ok(Value::Teks(s)) => (200, "text/plain".to_string(), s),
            Ok(other) => {
                let json = nilai_ke_json_string(&other, false, 0, pos).unwrap_or_else(|_| "{}".to_string());
                (200, "application/json".to_string(), json)
            }
            Err(e) => {
                eprintln!("[Selasih Ralat Dalaman] {}", e.message);
                let err_json = "{\"ralat\": \"Ralat dalaman pelayan.\"}".to_string();
                (500, "application/json".to_string(), err_json)
            }
        };

        let safe_content_type = content_type.replace(['\r', '\n'], "");
        let status_msg = match status_code {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            413 => "Payload Too Large",
            500 => "Internal Server Error",
            _ => "Response",
        };

        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nX-XSS-Protection: 1; mode=block\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\nConnection: close\r\n\r\n{}",
            status_code, status_msg, safe_content_type, res_body.len(), res_body
        );

        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();

        count += 1;
        if had_permintaan > 0 && count >= had_permintaan {
            break;
        }
    }

    Ok(Value::Nombor(count as f64))
}

fn alamat_dalaman_disekat(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_link_local() || v4.is_unspecified() || v4.octets() == [100, 100, 100, 200]
        }
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                return alamat_dalaman_disekat(&IpAddr::V4(v4));
            }
            let seg = v6.segments();
            v6.is_unspecified()
                || (seg[0] & 0xffc0) == 0xfe80
                || seg == [0xfd00, 0x0ec2, 0, 0, 0, 0, 0, 0x0254]
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

    let url_clean = if let Some(baki) = url_raw.strip_prefix("http://") {
        baki
    } else if url_raw.to_lowercase().starts_with("https://") {
        return Err(ralat_url(
            "HTTPS belum disokong kerana MELAYU tiada pustaka TLS terbina-dalam. Guna alamat http:// atau proksi tempatan.",
        ));
    } else if url_raw.contains("://") {
        return Err(ralat_url("Hanya URL http:// disokong oleh hantarPermintaanHTTP."));
    } else {
        url_raw
    };

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
            None if selepas.is_empty() => 80,
            None => return Err(ralat_url("Format hos URL tidak sah.")),
        };
        (&baki[..tutup], p)
    } else if let Some(colon_idx) = host_port.find(':') {
        (&host_port[..colon_idx], baca_port(&host_port[colon_idx + 1..])?)
    } else {
        (host_port, 80)
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
    let mut stream = match sambungan {
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

    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));

    let req_content = format!(
        "{} {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        kaedah,
        path,
        if host.contains(':') { format!("[{}]", host) } else { host.to_string() },
        port,
        badan_str.len(),
        badan_str
    );

    if let Err(e) = stream.write_all(req_content.as_bytes()) {
        return Err(MelayuError::new("Alamak!", &format!("Gagal menghantar permintaan HTTP: {}", e), pos.line, pos.col).with_kind(ErrorKind::Runtime));
    }
    let _ = stream.flush();

    let mut response_bytes = Vec::new();
    let _ = stream.take(5 * 1024 * 1024).read_to_end(&mut response_bytes);

    let res_str = String::from_utf8_lossy(&response_bytes);
    let mut parts = res_str.splitn(2, "\r\n\r\n");
    let header_part = parts.next().unwrap_or("");
    let body_part = parts.next().unwrap_or("").to_string();

    let mut header_lines = header_part.lines();
    let status_line = header_lines.next().unwrap_or("");
    let status_code: f64 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let mut map_resp = PetaNilai::new();
    map_resp.insert("status".to_string(), Value::Nombor(status_code));
    map_resp.insert("badan".to_string(), Value::Teks(body_part));

    let mut headers_resp = PetaNilai::new();
    for hl in header_lines {
        if let Some((k, v)) = hl.split_once(':') {
            headers_resp.insert(k.trim().to_lowercase(), Value::Teks(v.trim().to_string()));
        }
    }
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
        out.push(Value::Senarai(Rc::new(RefCell::new(probs))));
    }
    Ok(Value::Senarai(Rc::new(RefCell::new(out))))
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
            Value::Senarai(Rc::new(RefCell::new(r)))
        })
        .collect();
    Ok(Value::Senarai(Rc::new(RefCell::new(out))))
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
        if ada_cuda { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "gpu_rocm_dikesan".to_string(),
        if ada_rocm { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "gpu_aktif".to_string(),
        if gpu_aktif { Value::Betul } else { Value::Salah },
    );
    map.insert(
        "ketepatan_gpu".to_string(),
        Value::Teks(if gpu_aktif { "FP32" } else { "tiada" }.to_string()),
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

pub fn builtin_cari_pecahan_terbaik(args: &[Value], pos: LineCol) -> Result<Value, MelayuError> {
    let ralat = |mesej: &str| {
        MelayuError::new("Alamak!", mesej, pos.line, pos.col).with_kind(ErrorKind::Runtime)
    };
    if args.len() != 3 {
        return Err(ralat("Fungsi 'cariPecahanTerbaik' memerlukan 3 hujah (X, y, senaraiCiri)."));
    }
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
        let teks_label = v.to_string();
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

    let baris: Vec<std::cell::Ref<Vec<Value>>> = x_senarai
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
    for c in ciri_rc.borrow().iter() {
        let f = match c {
            Value::Nombor(f) if *f >= 0.0 && f.fract() == 0.0 => *f as usize,
            _ => return Err(ralat("senaraiCiri mestilah senarai indeks lajur (nombor bulat).")),
        };
        pasangan.clear();
        for (i, b) in baris.iter().enumerate() {
            match b.get(f) {
                Some(Value::Nombor(v)) => pasangan.push((*v, indeks_label[i])),
                _ => return Err(ralat("Nilai ciri dalam X mestilah nombor.")),
            }
        }
        pasangan.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut kiri = vec![0usize; k];
        let mut kanan = jumlah.clone();
        for i in 0..n - 1 {
            let (v, c) = pasangan[i];
            kiri[c] += 1;
            kanan[c] -= 1;
            let v_seterusnya = pasangan[i + 1].0;
            if v == v_seterusnya {
                continue;
            }
            let n_kiri = i + 1;
            let n_kanan = n - n_kiri;
            let skor = (n_kiri as f64 * gini(&kiri, n_kiri) + n_kanan as f64 * gini(&kanan, n_kanan)) / n as f64;
            if !matches!(terbaik, Some((s, _, _)) if s <= skor) {
                terbaik = Some((skor, f, (v + v_seterusnya) / 2.0));
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
