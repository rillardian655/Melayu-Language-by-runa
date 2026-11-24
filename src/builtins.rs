use crate::ast::LineCol;
use crate::errors::{ErrorKind, MelayuError};
use crate::interpreter::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

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
            if !can_target.starts_with(&can_base) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    "Akses ke laluan fail di luar direktori kerja disekat.",
                    pos.line,
                    pos.col,
                ));
            }
            Ok(can_target)
        } else {
            if !p.starts_with(&can_base) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    "Laluan mutlak sasaran berada di luar direktori kerja.",
                    pos.line,
                    pos.col,
                ));
            }
            Ok(p.to_path_buf())
        }
    } else {
        let combined = can_base.join(p);
        if combined.exists() {
            let can_target = combined.canonicalize().map_err(|e| {
                MelayuError::akses_fail_disekat(
                    laluan_str,
                    &format!("Laluan fail tidak sah: {}", e),
                    pos.line,
                    pos.col,
                )
            })?;
            if !can_target.starts_with(&can_base) {
                return Err(MelayuError::akses_fail_disekat(
                    laluan_str,
                    "Laluan fail terkeluar daripada sandbox.",
                    pos.line,
                    pos.col,
                ));
            }
            Ok(can_target)
        } else {
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
                        if !can_parent.starts_with(&can_base) {
                            return Err(MelayuError::akses_fail_disekat(
                                laluan_str,
                                "Direktori sasaran di luar direktori kerja dibenarkan.",
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
        let mut map = HashMap::new();

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

            let mut keys: Vec<&String> = borrowed.keys().collect();
            keys.sort();

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

            let mut keys: Vec<&String> = borrowed.keys().collect();
            keys.sort();

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
    for i in 0..m {
        for p in 0..k {
            let a_ip = a[i][p];
            for j in 0..n {
                c[i][j] += a_ip * b[p][j];
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

    let mut map = HashMap::new();
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

    for (i, (x_val, y_val)) in x_list.into_iter().zip(y_list.into_iter()).enumerate() {
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

pub const AKAL_ML_STDLIB: &str = r#"
# ==============================================================================
# KERANGKA KERJA AKALML (MACHINE LEARNING BAWAAN BAHASA MELAYU)
# ==============================================================================

kelas Matriks
    bina(data)
        diri.data = data
        diri.baris = panjang(data)
        jika diri.baris > 0
            diri.lajur = panjang(data[0])
        jika tidak
            diri.lajur = 0
        selesai
    selesai

    kerja darab(mLain)
        ada hasil = darabMatriks(diri.data, mLain.data)
        pulang Matriks(hasil)
    selesai

    kerja transposisi()
        ada hasil = transposisiMatriks(diri.data)
        pulang Matriks(hasil)
    selesai

    kerja songsang()
        ada hasil = songsangMatriks(diri.data)
        pulang Matriks(hasil)
    selesai

    kerja nilai()
        pulang diri.data
    selesai

    kerja paparkan()
        tunjuk "Matriks (" + teks(diri.baris) + " x " + teks(diri.lajur) + "):"
        untuk baris dalam diri.data
            ada barisTeks = "["
            ada pertama = betul
            untuk el dalam baris
                jika bukan pertama
                    barisTeks = barisTeks + ", "
                selesai
                barisTeks = barisTeks + teks(el)
                pertama = salah
            selesai
            barisTeks = barisTeks + "]"
            tunjuk barisTeks
        selesai
    selesai
selesai

kelas RegresiLinear
    bina(denganPintas)
        diri.denganPintas = denganPintas
        diri.pekali = kosong
        diri.pintas = 0
    selesai

    kerja latih(X, y)
        ada n_sampel = panjang(X)
        jika n_sampel == 0
            baling "Dataset latih kosong!"
        selesai

        ada X_mat = []
        jika diri.denganPintas
            untuk baris dalam X
                ada barisBaru = [1.0]
                untuk c dalam baris
                    barisBaru.tambah(c)
                selesai
                X_mat.tambah(barisBaru)
            selesai
        jika tidak
            X_mat = X
        selesai

        ada y_mat = []
        untuk val dalam y
            y_mat.tambah([val])
        selesai

        ada Xt = transposisiMatriks(X_mat)
        ada XtX = darabMatriks(Xt, X_mat)
        ada invXtX = songsangMatriks(XtX)
        ada Xty = darabMatriks(Xt, y_mat)
        ada beta = darabMatriks(invXtX, Xty)

        ada hasil_beta = []
        untuk b dalam beta
            hasil_beta.tambah(b[0])
        selesai

        jika diri.denganPintas
            diri.pintas = hasil_beta[0]
            ada pekali_tanpa_pintas = []
            ada idx = 1
            selagi idx < panjang(hasil_beta)
                pekali_tanpa_pintas.tambah(hasil_beta[idx])
                idx = idx + 1
            selesai
            diri.pekali = pekali_tanpa_pintas
        jika tidak
            diri.pintas = 0
            diri.pekali = hasil_beta
        selesai
    selesai

    kerja ramal(X_baru)
        jika diri.pekali == kosong
            baling "Model belum dilatih! Sila panggil .latih(X, y) dahulu."
        selesai

        ada ramalan = []
        untuk baris dalam X_baru
            ada y_topi = diri.pintas
            ada i = 0
            selagi i < panjang(diri.pekali)
                y_topi = y_topi + (baris[i] * diri.pekali[i])
                i = i + 1
            selesai
            ramalan.tambah(y_topi)
        selesai
        pulang ramalan
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        pulang skorR2(y_uji, y_ramal)
    selesai
selesai

kelas PengelasKNN
    bina(k)
        diri.k = k
        diri.X_latih = []
        diri.y_latih = []
    selesai

    kerja latih(X, y)
        diri.X_latih = X
        diri.y_latih = y
    selesai

    kerja ramal(X_baru)
        jika panjang(diri.X_latih) == 0
            baling "Model KNN belum dilatih! Sila panggil .latih(X, y) dahulu."
        selesai

        ada ramalan = []
        untuk titik dalam X_baru
            ada jarak_senarai = []
            ada i = 0
            selagi i < panjang(diri.X_latih)
                ada d = jarakEuclidean(titik, diri.X_latih[i])
                jarak_senarai.tambah([d, diri.y_latih[i]])
                i = i + 1
            selesai

            ada n = panjang(jarak_senarai)
            ada a = 0
            selagi a < n
                ada min_idx = a
                ada b = a + 1
                selagi b < n
                    jika jarak_senarai[b][0] < jarak_senarai[min_idx][0]
                        min_idx = b
                    selesai
                    b = b + 1
                selesai
                ada temp = jarak_senarai[a]
                jarak_senarai[a] = jarak_senarai[min_idx]
                jarak_senarai[min_idx] = temp
                a = a + 1
            selesai

            ada undian = {}
            ada c = 0
            ada had_k = diri.k
            jika had_k > n
                had_k = n
            selesai

            selagi c < had_k
                ada label = teks(jarak_senarai[c][1])
                jika undian.adaKunci(label)
                    undian[label] = undian[label] + 1
                jika tidak
                    undian[label] = 1
                selesai
                c = c + 1
            selesai

            ada label_terbaik = kosong
            ada undian_maks = -1
            untuk kunci dalam undian.kunci()
                jika undian[kunci] > undian_maks
                    undian_maks = undian[kunci]
                    label_terbaik = kunci
                selesai
            selesai

            ramalan.tambah(label_terbaik)
        selesai
        pulang ramalan
    selesai

    kerja skor(X_uji, y_uji)
        ada y_ramal = diri.ramal(X_uji)
        ada y_uji_teks = []
        untuk val dalam y_uji
            y_uji_teks.tambah(teks(val))
        selesai
        pulang skorAkurasi(y_uji_teks, y_ramal)
    selesai
selesai

kelas PengelompokanKMeans
    bina(k, lelaranMaksimum)
        diri.k = k
        diri.lelaranMaksimum = lelaranMaksimum
        diri.pusat = []
    selesai

    kerja latih(X)
        ada n_sampel = panjang(X)
        jika n_sampel < diri.k
            baling "Bilangan sampel mestilah lebih besar atau sama dengan k!"
        selesai

        diri.pusat = []
        ada i = 0
        selagi i < diri.k
            diri.pusat.tambah(X[i])
            i = i + 1
        selesai

        ada n_dim = panjang(X[0])
        ada pusingan = 0

        selagi pusingan < diri.lelaranMaksimum
            ada kelompok = []
            ada idx = 0
            selagi idx < diri.k
                kelompok.tambah([])
                idx = idx + 1
            selesai

            untuk titik dalam X
                ada jarak_min = 1000000000.0
                ada pusat_terpilih = 0
                ada c = 0
                selagi c < diri.k
                    ada dist = jarakEuclidean(titik, diri.pusat[c])
                    jika dist < jarak_min
                        jarak_min = dist
                        pusat_terpilih = c
                    selesai
                    c = c + 1
                selesai
                kelompok[pusat_terpilih].tambah(titik)
            selesai

            ada pusat_baharu = []
            ada c = 0
            selagi c < diri.k
                ada anggota = kelompok[c]
                jika panjang(anggota) == 0
                    pusat_baharu.tambah(diri.pusat[c])
                jika tidak
                    ada pusat_c = []
                    ada d = 0
                    selagi d < n_dim
                        ada jumlah = 0.0
                        untuk pt dalam anggota
                            jumlah = jumlah + pt[d]
                        selesai
                        pusat_c.tambah(jumlah / panjang(anggota))
                        d = d + 1
                    selesai
                    pusat_baharu.tambah(pusat_c)
                selesai
                c = c + 1
            selesai

            diri.pusat = pusat_baharu
            pusingan = pusingan + 1
        selesai
    selesai

    kerja ramal(X_baru)
        ada ramalan = []
        untuk titik dalam X_baru
            ada jarak_min = 1000000000.0
            ada pusat_terpilih = 0
            ada c = 0
            selagi c < diri.k
                ada dist = jarakEuclidean(titik, diri.pusat[c])
                jika dist < jarak_min
                    jarak_min = dist
                    pusat_terpilih = c
                selesai
                c = c + 1
            selesai
            ramalan.tambah(pusat_terpilih)
        selesai
        pulang ramalan
    selesai
selesai
"#;
