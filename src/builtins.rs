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
    Ok(Value::Teks(args[0].jenis().to_string()))
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
