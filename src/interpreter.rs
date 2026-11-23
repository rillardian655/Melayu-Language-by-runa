use crate::ast::*;
use crate::builtins::{daftar_fungsi_bawaan, lakukan_baca_input, semak_laluan_selamat};
use crate::errors::{ErrorKind, MelayuError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Kosong,
    Betul,
    Salah,
    Nombor(f64),
    Teks(String),
    Senarai(Rc<RefCell<Vec<Value>>>),
    Kamus(Rc<RefCell<HashMap<String, Value>>>),
    Fungsi {
        nama: String,
        parameter: Vec<(String, Option<String>)>,
        badan: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    Bawaan(String, fn(&[Value], LineCol) -> Result<Value, MelayuError>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Kosong, Value::Kosong) => true,
            (Value::Betul, Value::Betul) => true,
            (Value::Salah, Value::Salah) => true,
            (Value::Nombor(a), Value::Nombor(b)) => a == b,
            (Value::Teks(a), Value::Teks(b)) => a == b,
            (Value::Senarai(a), Value::Senarai(b)) => *a.borrow() == *b.borrow(),
            (Value::Kamus(a), Value::Kamus(b)) => *a.borrow() == *b.borrow(),
            (Value::Fungsi { nama: a, .. }, Value::Fungsi { nama: b, .. }) => a == b,
            (Value::Bawaan(a, _), Value::Bawaan(b, _)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    pub fn jenis(&self) -> &'static str {
        match self {
            Value::Kosong => "kosong",
            Value::Betul | Value::Salah => "betul/salah",
            Value::Nombor(_) => "nombor",
            Value::Teks(_) => "teks",
            Value::Senarai(_) => "senarai",
            Value::Kamus(_) => "kamus",
            Value::Fungsi { .. } => "kerja",
            Value::Bawaan(..) => "kerja terbina-dalam",
        }
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Salah | Value::Kosong)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Kosong => write!(f, "kosong"),
            Value::Betul => write!(f, "betul"),
            Value::Salah => write!(f, "salah"),
            Value::Nombor(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Teks(s) => write!(f, "{}", s),
            Value::Senarai(list) => {
                write!(f, "[")?;
                let borrowed = list.borrow();
                for (i, v) in borrowed.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Value::Teks(t) = v {
                        write!(f, "\"{}\"", t)?;
                    } else {
                        write!(f, "{}", v)?;
                    }
                }
                write!(f, "]")
            }
            Value::Kamus(map) => {
                write!(f, "kamus {{")?;
                let borrowed = map.borrow();
                let mut first = true;
                for (k, v) in borrowed.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "\"{}\": ", k)?;
                    if let Value::Teks(t) = v {
                        write!(f, "\"{}\"", t)?;
                    } else {
                        write!(f, "{}", v)?;
                    }
                }
                write!(f, "}}")
            }
            Value::Fungsi { nama, .. } => write!(f, "<kerja: {}>", nama),
            Value::Bawaan(nama, _) => write!(f, "<kerja terbina-dalam: {}>", nama),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub nilai: Value,
    pub tetap: bool,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Variable>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: &str, nilai: Value, tetap: bool) {
        self.values
            .insert(name.to_string(), Variable { nilai, tetap });
    }

    pub fn assign(&mut self, name: &str, nilai: Value, pos: LineCol) -> Result<(), MelayuError> {
        if let Some(var) = self.values.get_mut(name) {
            if var.tetap {
                return Err(MelayuError::ubah_pembolehubah_tetap(
                    name, pos.line, pos.col,
                ));
            }
            var.nilai = nilai;
            return Ok(());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow_mut().assign(name, nilai, pos);
        }

        Err(MelayuError::pembolehubah_tak_wujud(name, pos.line, pos.col))
    }

    pub fn get(&self, name: &str, pos: LineCol) -> Result<Value, MelayuError> {
        if let Some(var) = self.values.get(name) {
            return Ok(var.nilai.clone());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name, pos);
        }

        Err(MelayuError::pembolehubah_tak_wujud(name, pos.line, pos.col))
    }
}

pub enum Flow {
    Lanjut,
    Pulang(Value),
    Hentikan,
    Teruskan,
}

// -----------------------------------------------------------------------------
// HAD KESELAMATAN & SANDBOX PENGHURAI MELAYU
// -----------------------------------------------------------------------------
pub const HAD_REKURSI_LALAI: usize = 1_000;
pub const HAD_LANGKAH_LALAI: u64 = 10_000_000;
pub const HAD_SAIZ_TEKS_MAKSIMUM: usize = 16 * 1024 * 1024; // 16 MB
pub const HAD_ELEMEN_SENARAI_MAKSIMUM: usize = 1_000_000; // 1 juta elemen

/// Memastikan nombor indeks sah: terhingga, bukan perpuluhan, dan dalam julat yang dibenarkan.
pub fn sahkan_indeks(n: f64, len: usize, pos: LineCol) -> Result<usize, MelayuError> {
    if !n.is_finite() {
        return Err(MelayuError::indeks_tidak_sah(
            "Indeks mestilah nombor terhingga (bukan NaN atau Infiniti).",
            pos.line,
            pos.col,
        ));
    }

    if n.fract() != 0.0 {
        return Err(MelayuError::indeks_tidak_sah(
            &format!(
                "Indeks mestilah nombor bulat (integer), bukan pecahan atau perpuluhan ({}).",
                n
            ),
            pos.line,
            pos.col,
        ));
    }

    let idx = n as isize;
    let actual_idx = if idx < 0 {
        let abs_idx = idx.unsigned_abs();
        if abs_idx > len {
            return Err(MelayuError::indeks_tidak_sah(
                &format!(
                    "Indeks negatif {} di luar julat saiz semasa ({}).",
                    idx, len
                ),
                pos.line,
                pos.col,
            ));
        }
        len - abs_idx
    } else {
        idx as usize
    };

    if actual_idx >= len {
        return Err(MelayuError::indeks_tidak_sah(
            &format!(
                "Indeks {} di luar julat yang sah (saiz sebenar: {}).",
                n, len
            ),
            pos.line,
            pos.col,
        ));
    }

    Ok(actual_idx)
}

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
    pub kedalaman_panggilan: usize,
    pub had_rekursi: usize,
    pub jumlah_langkah: u64,
    pub had_langkah: Option<u64>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));

        // Muat fungsi terbina-dalam ke dalam skop global
        for (nama, func) in daftar_fungsi_bawaan() {
            env.borrow_mut()
                .define(nama, Value::Bawaan(nama.to_string(), func), true);
        }

        Self {
            env,
            kedalaman_panggilan: 0,
            had_rekursi: HAD_REKURSI_LALAI,
            jumlah_langkah: 0,
            had_langkah: Some(HAD_LANGKAH_LALAI),
        }
    }

    pub fn with_step_limit(mut self, limit: Option<u64>) -> Self {
        self.had_langkah = limit;
        self
    }

    pub fn with_recursion_limit(mut self, limit: usize) -> Self {
        self.had_rekursi = limit;
        self
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, MelayuError> {
        let env_rc = Rc::clone(&self.env);
        for stmt in &program.pernyataan {
            match self.execute_stmt(stmt, &env_rc)? {
                Flow::Pulang(v) => return Ok(v),
                Flow::Hentikan => {
                    return Err(MelayuError::new(
                        "Woy!",
                        "'hentikan' tidak boleh berada di luar gelung.",
                        1,
                        1,
                    )
                    .with_kind(ErrorKind::Runtime));
                }
                Flow::Teruskan => {
                    return Err(MelayuError::new(
                        "Woy!",
                        "'teruskan' tidak boleh berada di luar gelung.",
                        1,
                        1,
                    )
                    .with_kind(ErrorKind::Runtime));
                }
                Flow::Lanjut => {}
            }
        }
        Ok(Value::Kosong)
    }

    pub fn panggil_nilai(
        &mut self,
        func_val: &Value,
        evaluated_args: &[Value],
        pos: LineCol,
    ) -> Result<Value, MelayuError> {
        match func_val {
            Value::Bawaan(_, func) => func(evaluated_args, pos),
            Value::Fungsi {
                nama,
                parameter,
                badan,
                closure,
            } => {
                if parameter.len() != evaluated_args.len() {
                    return Err(MelayuError::new(
                        "Alamak!",
                        &format!(
                            "Fungsi '{}' memerlukan {} hujah, tetapi menerima {}.",
                            nama,
                            parameter.len(),
                            evaluated_args.len()
                        ),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime));
                }

                if self.kedalaman_panggilan >= self.had_rekursi {
                    return Err(MelayuError::rekursi_berlebihan(
                        self.had_rekursi,
                        pos.line,
                        pos.col,
                    ));
                }

                let call_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(closure))));
                for (i, (p_name, _)) in parameter.iter().enumerate() {
                    call_env
                        .borrow_mut()
                        .define(p_name, evaluated_args[i].clone(), false);
                }

                self.kedalaman_panggilan += 1;
                let hasil = (|| -> Result<Value, MelayuError> {
                    for s in badan {
                        match self.execute_stmt(s, &call_env)? {
                            Flow::Lanjut => {}
                            Flow::Pulang(v) => return Ok(v),
                            Flow::Hentikan | Flow::Teruskan => {
                                return Err(MelayuError::new(
                                    "Woy!",
                                    "'hentikan' atau 'teruskan' tidak sah di luar gelung.",
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                        }
                    }
                    Ok(Value::Kosong)
                })();
                self.kedalaman_panggilan -= 1;
                hasil
            }
            other => Err(MelayuError::new(
                "Eh!",
                &format!("'{}' bukan fungsi yang boleh dipanggil.", other.jenis()),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime)),
        }
    }

    pub fn panggil_kaedah(
        &mut self,
        target_val: Value,
        kaedah: &str,
        evaluated_args: &[Value],
        pos: LineCol,
    ) -> Result<Value, MelayuError> {
        match target_val {
            Value::Senarai(list) => match kaedah {
                "tambah" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.tambah()' memerlukan 1 elemen untuk dimasukkan.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let mut borrowed = list.borrow_mut();
                    if borrowed.len() >= HAD_ELEMEN_SENARAI_MAKSIMUM {
                        return Err(MelayuError::had_memori_terlampau(
                            "Bilangan elemen senarai melebihi had keselamatan maksimum (1,000,000)",
                            pos.line,
                            pos.col,
                        ));
                    }
                    borrowed.push(evaluated_args[0].clone());
                    Ok(Value::Kosong)
                }
                "buang" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.buang(indeks)' memerlukan 1 hujah nombor indeks.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    match evaluated_args[0] {
                        Value::Nombor(n) => {
                            let mut borrowed = list.borrow_mut();
                            let actual_idx = sahkan_indeks(n, borrowed.len(), pos)?;
                            Ok(borrowed.remove(actual_idx))
                        }
                        ref other => {
                            Err(MelayuError::jenis_salah(
                                "nombor",
                                other.jenis(),
                                pos.line,
                                pos.col,
                            ))
                        }
                    }
                }
                "panjang" => Ok(Value::Nombor(list.borrow().len() as f64)),
                "berisi" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.berisi()' memerlukan 1 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let borrowed = list.borrow();
                    let found = borrowed.contains(&evaluated_args[0]);
                    Ok(if found { Value::Betul } else { Value::Salah })
                }
                "isih" => {
                    let mut borrowed = list.borrow_mut();
                    borrowed.sort_by(|a, b| match (a, b) {
                        (Value::Nombor(x), Value::Nombor(y)) => {
                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (Value::Teks(x), Value::Teks(y)) => x.cmp(y),
                        _ => std::cmp::Ordering::Equal,
                    });
                    Ok(Value::Senarai(Rc::clone(&list)))
                }
                "balik" => {
                    list.borrow_mut().reverse();
                    Ok(Value::Senarai(Rc::clone(&list)))
                }
                "cantum" => {
                    let pemisah = if evaluated_args.is_empty() {
                        "".to_string()
                    } else {
                        match &evaluated_args[0] {
                            Value::Teks(s) => s.clone(),
                            other => other.to_string(),
                        }
                    };
                    let borrowed = list.borrow();
                    let parts: Vec<String> =
                        borrowed.iter().map(|v| v.to_string()).collect();
                    let result = parts.join(&pemisah);
                    if result.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                        return Err(MelayuError::had_memori_terlampau(
                            "Saiz teks cantuman melebihi 16 MB",
                            pos.line,
                            pos.col,
                        ));
                    }
                    Ok(Value::Teks(result))
                }
                "petakan" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.petakan(fungsi)' memerlukan 1 hujah fungsi penukar.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let func_val = &evaluated_args[0];
                    let items = list.borrow().clone();
                    let mut mapped = Vec::with_capacity(items.len());
                    for item in items {
                        let res = self.panggil_nilai(func_val, &[item], pos)?;
                        mapped.push(res);
                    }
                    if mapped.len() > HAD_ELEMEN_SENARAI_MAKSIMUM {
                        return Err(MelayuError::had_memori_terlampau(
                            "Bilangan elemen senarai petakan melebihi had keselamatan (1,000,000)",
                            pos.line,
                            pos.col,
                        ));
                    }
                    Ok(Value::Senarai(Rc::new(RefCell::new(mapped))))
                }
                "tapis" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.tapis(fungsi)' memerlukan 1 hujah fungsi penapis.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let func_val = &evaluated_args[0];
                    let items = list.borrow().clone();
                    let mut filtered = Vec::new();
                    for item in items {
                        let res =
                            self.panggil_nilai(func_val, std::slice::from_ref(&item), pos)?;
                        if res.is_truthy() {
                            filtered.push(item);
                        }
                    }
                    Ok(Value::Senarai(Rc::new(RefCell::new(filtered))))
                }
                "kiraSemua" => {
                    if evaluated_args.len() != 2 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.kiraSemua(fungsi, awal)' memerlukan 2 hujah (fungsi pengumpul, nilai awal).",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let func_val = &evaluated_args[0];
                    let mut terkumpul = evaluated_args[1].clone();
                    let items = list.borrow().clone();
                    for item in items {
                        terkumpul = self.panggil_nilai(func_val, &[terkumpul, item], pos)?;
                    }
                    Ok(terkumpul)
                }
                "cari" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.cari(fungsi)' memerlukan 1 hujah fungsi carian.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let func_val = &evaluated_args[0];
                    let items = list.borrow().clone();
                    for item in items {
                        let res =
                            self.panggil_nilai(func_val, std::slice::from_ref(&item), pos)?;
                        if res.is_truthy() {
                            return Ok(item);
                        }
                    }
                    Ok(Value::Kosong)
                }
                "indeks" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.indeks(item)' memerlukan 1 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let target = &evaluated_args[0];
                    let borrowed = list.borrow();
                    let idx = borrowed.iter().position(|v| v == target);
                    match idx {
                        Some(i) => Ok(Value::Nombor(i as f64)),
                        None => Ok(Value::Nombor(-1.0)),
                    }
                }
                "setiapSatu" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.setiapSatu(fungsi)' memerlukan 1 hujah fungsi callback.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let func_val = &evaluated_args[0];
                    let items = list.borrow().clone();
                    for item in items {
                        self.panggil_nilai(func_val, &[item], pos)?;
                    }
                    Ok(Value::Kosong)
                }
                other => Err(MelayuError::new(
                    "Eh!",
                    &format!("Kaedah '.{}()' tidak wujud untuk senarai.", other),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)
                .with_suggestion("Kaedah senarai yang ada: .tambah(item), .buang(indeks), .panjang(), .berisi(item), .isih(), .balik(), .cantum(pemisah), .petakan(fungsi), .tapis(fungsi), .kiraSemua(fungsi, awal), .cari(fungsi), .indeks(item), .setiapSatu(fungsi)")),
            },
            Value::Kamus(map) => match kaedah {
                "kunci" => {
                    let borrowed = map.borrow();
                    let keys: Vec<Value> =
                        borrowed.keys().cloned().map(Value::Teks).collect();
                    Ok(Value::Senarai(Rc::new(RefCell::new(keys))))
                }
                "nilai" => {
                    let borrowed = map.borrow();
                    let values: Vec<Value> = borrowed.values().cloned().collect();
                    Ok(Value::Senarai(Rc::new(RefCell::new(values))))
                }
                "panjang" => Ok(Value::Nombor(map.borrow().len() as f64)),
                "adaKunci" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.adaKunci(kunci)' memerlukan 1 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let key = match &evaluated_args[0] {
                        Value::Teks(s) => s.clone(),
                        other => other.to_string(),
                    };
                    let exists = map.borrow().contains_key(&key);
                    Ok(if exists { Value::Betul } else { Value::Salah })
                }
                "buang" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.buang(kunci)' memerlukan 1 hujah nama kunci.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let key = match &evaluated_args[0] {
                        Value::Teks(s) => s.clone(),
                        other => other.to_string(),
                    };
                    let removed = map.borrow_mut().remove(&key);
                    Ok(removed.unwrap_or(Value::Kosong))
                }
                other => Err(MelayuError::new(
                    "Eh!",
                    &format!("Kaedah '.{}()' tidak wujud untuk kamus.", other),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)
                .with_suggestion("Kaedah kamus yang ada: .kunci(), .nilai(), .panjang(), .adaKunci(kunci), .buang(kunci)")),
            },
            Value::Teks(s) => match kaedah {
                "hurufBesar" => Ok(Value::Teks(s.to_uppercase())),
                "hurufKecil" => Ok(Value::Teks(s.to_lowercase())),
                "hurufAwalBesar" => {
                    let mut c = s.chars();
                    let res = match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    };
                    Ok(Value::Teks(res))
                }
                "panjang" => Ok(Value::Nombor(s.chars().count() as f64)),
                "potong" => Ok(Value::Teks(s.trim().to_string())),
                "pecah" => {
                    let pemisah = if evaluated_args.is_empty() {
                        "".to_string()
                    } else {
                        match &evaluated_args[0] {
                            Value::Teks(p) => p.clone(),
                            other => other.to_string(),
                        }
                    };
                    let items: Vec<Value> = if pemisah.is_empty() {
                        s.chars().map(|c| Value::Teks(c.to_string())).collect()
                    } else {
                        s.split(&pemisah)
                            .map(|part| Value::Teks(part.to_string()))
                            .collect()
                    };
                    if items.len() > HAD_ELEMEN_SENARAI_MAKSIMUM {
                        return Err(MelayuError::had_memori_terlampau(
                            "Bilangan elemen hasil pecahan teks melebihi 1,000,000",
                            pos.line,
                            pos.col,
                        ));
                    }
                    Ok(Value::Senarai(Rc::new(RefCell::new(items))))
                }
                "ganti" => {
                    if evaluated_args.len() != 2 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.ganti(lama, baru)' memerlukan 2 hujah teks.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let lama = evaluated_args[0].to_string();
                    let baru = evaluated_args[1].to_string();
                    let replaced = s.replace(&lama, &baru);
                    if replaced.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                        return Err(MelayuError::had_memori_terlampau(
                            "Saiz teks ganti melebihi 16 MB",
                            pos.line,
                            pos.col,
                        ));
                    }
                    Ok(Value::Teks(replaced))
                }
                "ulang" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.ulang(bilangan)' memerlukan 1 hujah nombor.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    match evaluated_args[0] {
                        Value::Nombor(n) => {
                            if n < 0.0 || n.fract() != 0.0 {
                                return Err(MelayuError::new(
                                    "Aduh!",
                                    "Bilangan ulangan mestilah integer positif (0 atau lebih).",
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                            let count = n as usize;
                            let saiz = s.len().saturating_mul(count);
                            if saiz > HAD_SAIZ_TEKS_MAKSIMUM {
                                return Err(MelayuError::had_memori_terlampau(
                                    "Saiz teks hasil ulangan melebihi had 16 MB.",
                                    pos.line,
                                    pos.col,
                                ));
                            }
                            Ok(Value::Teks(s.repeat(count)))
                        }
                        ref other => Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        )),
                    }
                }
                "selitDepan" => {
                    if evaluated_args.is_empty() || evaluated_args.len() > 2 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.selitDepan(panjang, aksara?)' memerlukan 1 atau 2 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let sasaran = match evaluated_args[0] {
                        Value::Nombor(n) => {
                            if n < 0.0 {
                                0
                            } else {
                                n as usize
                            }
                        }
                        ref other => {
                            return Err(MelayuError::jenis_salah(
                                "nombor",
                                other.jenis(),
                                pos.line,
                                pos.col,
                            ));
                        }
                    };
                    let pad_str = if evaluated_args.len() == 2 {
                        match &evaluated_args[1] {
                            Value::Teks(t) => {
                                if t.is_empty() {
                                    " ".to_string()
                                } else {
                                    t.clone()
                                }
                            }
                            other => other.to_string(),
                        }
                    } else {
                        " ".to_string()
                    };
                    let char_count = s.chars().count();
                    if char_count >= sasaran {
                        Ok(Value::Teks(s))
                    } else {
                        let mut hasil = String::new();
                        let pad_needed = sasaran - char_count;
                        let pad_chars: Vec<char> = pad_str.chars().collect();
                        for i in 0..pad_needed {
                            hasil.push(pad_chars[i % pad_chars.len()]);
                        }
                        hasil.push_str(&s);
                        if hasil.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                            return Err(MelayuError::had_memori_terlampau(
                                "Saiz teks selit melebihi had 16 MB.",
                                pos.line,
                                pos.col,
                            ));
                        }
                        Ok(Value::Teks(hasil))
                    }
                }
                "selitBelakang" => {
                    if evaluated_args.is_empty() || evaluated_args.len() > 2 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.selitBelakang(panjang, aksara?)' memerlukan 1 atau 2 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let sasaran = match evaluated_args[0] {
                        Value::Nombor(n) => {
                            if n < 0.0 {
                                0
                            } else {
                                n as usize
                            }
                        }
                        ref other => {
                            return Err(MelayuError::jenis_salah(
                                "nombor",
                                other.jenis(),
                                pos.line,
                                pos.col,
                            ));
                        }
                    };
                    let pad_str = if evaluated_args.len() == 2 {
                        match &evaluated_args[1] {
                            Value::Teks(t) => {
                                if t.is_empty() {
                                    " ".to_string()
                                } else {
                                    t.clone()
                                }
                            }
                            other => other.to_string(),
                        }
                    } else {
                        " ".to_string()
                    };
                    let char_count = s.chars().count();
                    if char_count >= sasaran {
                        Ok(Value::Teks(s))
                    } else {
                        let mut hasil = s;
                        let pad_needed = sasaran - char_count;
                        let pad_chars: Vec<char> = pad_str.chars().collect();
                        for i in 0..pad_needed {
                            hasil.push(pad_chars[i % pad_chars.len()]);
                        }
                        if hasil.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                            return Err(MelayuError::had_memori_terlampau(
                                "Saiz teks selit melebihi had 16 MB.",
                                pos.line,
                                pos.col,
                            ));
                        }
                        Ok(Value::Teks(hasil))
                    }
                }
                "mulaDengan" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.mulaDengan(awalan)' memerlukan 1 hujah teks.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let awalan = evaluated_args[0].to_string();
                    Ok(if s.starts_with(&awalan) {
                        Value::Betul
                    } else {
                        Value::Salah
                    })
                }
                "tamatDengan" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.tamatDengan(akhiran)' memerlukan 1 hujah teks.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let akhiran = evaluated_args[0].to_string();
                    Ok(if s.ends_with(&akhiran) {
                        Value::Betul
                    } else {
                        Value::Salah
                    })
                }
                "mengandungi" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.mengandungi(subteks)' memerlukan 1 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let subteks = evaluated_args[0].to_string();
                    Ok(if s.contains(&subteks) {
                        Value::Betul
                    } else {
                        Value::Salah
                    })
                }
                other => Err(MelayuError::new(
                    "Eh!",
                    &format!("Kaedah '.{}()' tidak wujud untuk teks.", other),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)
                .with_suggestion("Kaedah teks yang ada: .hurufBesar(), .hurufKecil(), .hurufAwalBesar(), .panjang(), .potong(), .pecah(pemisah), .ganti(lama, baru), .ulang(n), .selitDepan(panjang, aksara?), .selitBelakang(panjang, aksara?), .mulaDengan(awalan), .tamatDengan(akhiran), .mengandungi(subteks)")),
            },
            other => Err(MelayuError::new(
                "Eh!",
                &format!("Jenis '{}' tidak mempunyai kaedah '.{}()'.", other.jenis(), kaedah),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime)),
        }
    }

    pub fn execute_stmt(
        &mut self,
        stmt: &Stmt,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Flow, MelayuError> {
        self.jumlah_langkah += 1;
        if let Some(had) = self.had_langkah {
            if self.jumlah_langkah > had {
                let pos = stmt.pos();
                return Err(MelayuError::had_langkah_terlampau(had, pos.line, pos.col));
            }
        }

        match stmt {
            Stmt::IsytiharPembolehubah {
                nama, tetap, nilai, ..
            } => {
                let val = self.evaluate_expr(nilai, env)?;
                env.borrow_mut().define(nama, val, *tetap);
                Ok(Flow::Lanjut)
            }
            Stmt::Umpukan {
                sasaran,
                nilai,
                pos,
            } => {
                let val = self.evaluate_expr(nilai, env)?;
                match sasaran {
                    TargetAssign::Pembolehubah(nama) => {
                        env.borrow_mut().assign(nama, val, *pos)?;
                    }
                    TargetAssign::Indeks { sasaran, indeks } => {
                        let target_val = self.evaluate_expr(sasaran, env)?;
                        let idx_val = self.evaluate_expr(indeks, env)?;
                        match target_val {
                            Value::Senarai(list) => match idx_val {
                                Value::Nombor(n) => {
                                    let mut borrowed = list.borrow_mut();
                                    let actual_idx = sahkan_indeks(n, borrowed.len(), *pos)?;
                                    borrowed[actual_idx] = val;
                                }
                                other => {
                                    return Err(MelayuError::jenis_salah(
                                        "nombor",
                                        other.jenis(),
                                        pos.line,
                                        pos.col,
                                    ));
                                }
                            },
                            Value::Kamus(map) => {
                                let key_str = match idx_val {
                                    Value::Teks(s) => s,
                                    other => other.to_string(),
                                };
                                map.borrow_mut().insert(key_str, val);
                            }
                            other => {
                                return Err(MelayuError::new(
                                    "Eh!",
                                    &format!(
                                        "Hanya senarai atau kamus yang boleh diindeks, bukan '{}'.",
                                        other.jenis()
                                    ),
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                        }
                    }
                }
                Ok(Flow::Lanjut)
            }
            Stmt::Tunjuk(args, _) => {
                let mut parts = Vec::new();
                for arg in args {
                    let val = self.evaluate_expr(arg, env)?;
                    parts.push(val.to_string());
                }
                println!("{}", parts.join(" "));
                Ok(Flow::Lanjut)
            }
            Stmt::Jika {
                syarat,
                maka,
                jika_tidak_jika,
                jika_tidak,
                ..
            } => {
                let cond_val = self.evaluate_expr(syarat, env)?;
                if cond_val.is_truthy() {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    for s in maka {
                        match self.execute_stmt(s, &local_env)? {
                            Flow::Lanjut => {}
                            flow => return Ok(flow),
                        }
                    }
                    return Ok(Flow::Lanjut);
                }

                for (jtj_syarat, jtj_badan) in jika_tidak_jika {
                    let jtj_cond = self.evaluate_expr(jtj_syarat, env)?;
                    if jtj_cond.is_truthy() {
                        let local_env =
                            Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                        for s in jtj_badan {
                            match self.execute_stmt(s, &local_env)? {
                                Flow::Lanjut => {}
                                flow => return Ok(flow),
                            }
                        }
                        return Ok(Flow::Lanjut);
                    }
                }

                if let Some(jt_badan) = jika_tidak {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    for s in jt_badan {
                        match self.execute_stmt(s, &local_env)? {
                            Flow::Lanjut => {}
                            flow => return Ok(flow),
                        }
                    }
                }

                Ok(Flow::Lanjut)
            }
            Stmt::Selagi { syarat, badan, .. } => {
                while self.evaluate_expr(syarat, env)?.is_truthy() {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    let mut should_break = false;
                    for s in badan {
                        match self.execute_stmt(s, &local_env)? {
                            Flow::Lanjut => {}
                            Flow::Teruskan => break,
                            Flow::Hentikan => {
                                should_break = true;
                                break;
                            }
                            Flow::Pulang(v) => return Ok(Flow::Pulang(v)),
                        }
                    }
                    if should_break {
                        break;
                    }
                }
                Ok(Flow::Lanjut)
            }
            Stmt::UntukDalam {
                pengenal,
                senarai,
                badan,
                pos,
            } => {
                let target_val = self.evaluate_expr(senarai, env)?;
                let items: Vec<Value> = match target_val {
                    Value::Senarai(list) => list.borrow().clone(),
                    Value::Teks(s) => s.chars().map(|c| Value::Teks(c.to_string())).collect(),
                    other => {
                        return Err(MelayuError::jenis_salah(
                            "senarai atau teks",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        ));
                    }
                };

                for item in items {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    local_env.borrow_mut().define(pengenal, item, false);
                    let mut should_break = false;
                    for s in badan {
                        match self.execute_stmt(s, &local_env)? {
                            Flow::Lanjut => {}
                            Flow::Teruskan => break,
                            Flow::Hentikan => {
                                should_break = true;
                                break;
                            }
                            Flow::Pulang(v) => return Ok(Flow::Pulang(v)),
                        }
                    }
                    if should_break {
                        break;
                    }
                }
                Ok(Flow::Lanjut)
            }
            Stmt::UntukJulat {
                pengenal,
                mula,
                hingga,
                langkah,
                badan,
                pos,
            } => {
                let start_val = self.evaluate_expr(mula, env)?;
                let end_val = self.evaluate_expr(hingga, env)?;
                let step_val = if let Some(l) = langkah {
                    self.evaluate_expr(l, env)?
                } else {
                    Value::Nombor(1.0)
                };

                let start_num = match start_val {
                    Value::Nombor(n) => n,
                    other => {
                        return Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        ))
                    }
                };
                let end_num = match end_val {
                    Value::Nombor(n) => n,
                    other => {
                        return Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        ))
                    }
                };
                let step_num = match step_val {
                    Value::Nombor(n) => {
                        if n == 0.0 {
                            return Err(MelayuError::new(
                                "Aduh!",
                                "Langkah julat tidak boleh kosong (0).",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                        n
                    }
                    other => {
                        return Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        ))
                    }
                };

                let mut current = start_num;
                while (step_num > 0.0 && current <= end_num)
                    || (step_num < 0.0 && current >= end_num)
                {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    local_env
                        .borrow_mut()
                        .define(pengenal, Value::Nombor(current), false);
                    let mut should_break = false;
                    for s in badan {
                        match self.execute_stmt(s, &local_env)? {
                            Flow::Lanjut => {}
                            Flow::Teruskan => break,
                            Flow::Hentikan => {
                                should_break = true;
                                break;
                            }
                            Flow::Pulang(v) => return Ok(Flow::Pulang(v)),
                        }
                    }
                    if should_break {
                        break;
                    }
                    current += step_num;
                }
                Ok(Flow::Lanjut)
            }
            Stmt::DeklarasiFungsi {
                nama,
                parameter,
                badan,
                ..
            } => {
                let func_val = Value::Fungsi {
                    nama: nama.clone(),
                    parameter: parameter.clone(),
                    badan: badan.clone(),
                    closure: Rc::clone(env),
                };
                env.borrow_mut().define(nama, func_val, true);
                Ok(Flow::Lanjut)
            }
            Stmt::Pulang(expr_opt, _) => {
                let val = if let Some(e) = expr_opt {
                    self.evaluate_expr(e, env)?
                } else {
                    Value::Kosong
                };
                Ok(Flow::Pulang(val))
            }
            Stmt::Hentikan(_) => Ok(Flow::Hentikan),
            Stmt::Teruskan(_) => Ok(Flow::Teruskan),
            Stmt::Cuba {
                badan_cuba,
                ralat_var,
                badan_tangkap,
                badan_akhirnya,
                ..
            } => {
                let cuba_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                let mut result_flow = Flow::Lanjut;
                let mut error_caught = None;

                for s in badan_cuba {
                    match self.execute_stmt(s, &cuba_env) {
                        Ok(Flow::Lanjut) => {}
                        Ok(flow) => {
                            result_flow = flow;
                            break;
                        }
                        Err(err) => {
                            error_caught = Some(err);
                            break;
                        }
                    }
                }

                if let Some(err) = error_caught {
                    let tangkap_env =
                        Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    tangkap_env
                        .borrow_mut()
                        .define(ralat_var, Value::Teks(err.message), false);
                    for s in badan_tangkap {
                        match self.execute_stmt(s, &tangkap_env)? {
                            Flow::Lanjut => {}
                            flow => {
                                result_flow = flow;
                                break;
                            }
                        }
                    }
                }

                if let Some(akh) = badan_akhirnya {
                    let akh_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                    for s in akh {
                        match self.execute_stmt(s, &akh_env)? {
                            Flow::Lanjut => {}
                            flow => {
                                result_flow = flow;
                                break;
                            }
                        }
                    }
                }

                Ok(result_flow)
            }
            Stmt::Baling(expr, pos) => {
                let val = self.evaluate_expr(expr, env)?;
                Err(
                    MelayuError::new("Alamak!", &val.to_string(), pos.line, pos.col)
                        .with_kind(ErrorKind::Runtime),
                )
            }
            Stmt::Ungkapan(expr, _) => {
                self.evaluate_expr(expr, env)?;
                Ok(Flow::Lanjut)
            }
            Stmt::Ambil {
                sasaran,
                simbol,
                alias,
                pos,
            } => {
                let safe_path = semak_laluan_selamat(sasaran, false, *pos)?;
                let final_path = if !safe_path.exists() && !sasaran.ends_with(".ml") {
                    let with_ml = safe_path.with_extension("ml");
                    if with_ml.exists() {
                        with_ml
                    } else {
                        safe_path
                    }
                } else {
                    safe_path
                };

                if !final_path.exists() {
                    return Err(MelayuError::akses_fail_disekat(
                        sasaran,
                        "Fail modul tidak dijumpai.",
                        pos.line,
                        pos.col,
                    ));
                }

                let content = fs::read_to_string(&final_path).map_err(|e| {
                    MelayuError::akses_fail_disekat(
                        sasaran,
                        &format!("Gagal membaca fail modul: {}", e),
                        pos.line,
                        pos.col,
                    )
                })?;

                let mut lexer = crate::lexer::Lexer::new(&content);
                let tokens = lexer.tokenize()?;
                let mut parser = crate::parser::Parser::new(tokens);
                let program = parser.parse()?;

                let module_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));

                for s in &program.pernyataan {
                    match self.execute_stmt(s, &module_env)? {
                        Flow::Lanjut => {}
                        Flow::Pulang(_) => break,
                        Flow::Hentikan | Flow::Teruskan => {}
                    }
                }

                let module_vals = module_env.borrow().values.clone();

                if let Some(alias_name) = alias {
                    let mut map = HashMap::new();
                    for (k, v) in module_vals {
                        map.insert(k, v.nilai);
                    }
                    env.borrow_mut().define(
                        alias_name,
                        Value::Kamus(Rc::new(RefCell::new(map))),
                        true,
                    );
                } else if let Some(syms) = simbol {
                    for sym in syms {
                        if let Some(var) = module_vals.get(sym) {
                            env.borrow_mut().define(sym, var.nilai.clone(), var.tetap);
                        } else {
                            return Err(MelayuError::new(
                                "Alamak!",
                                &format!(
                                    "Simbol '{}' tidak dijumpai dalam modul '{}'.",
                                    sym, sasaran
                                ),
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    }
                } else {
                    for (k, v) in module_vals {
                        env.borrow_mut().define(&k, v.nilai, v.tetap);
                    }
                }

                Ok(Flow::Lanjut)
            }
            Stmt::Horas(_) | Stmt::Mangan(_) => Ok(Flow::Lanjut),
        }
    }

    pub fn evaluate_expr(
        &mut self,
        expr: &Expr,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, MelayuError> {
        match expr {
            Expr::Literal(lit, _) => match lit {
                LiteralValue::Nombor(n) => Ok(Value::Nombor(*n)),
                LiteralValue::Teks(s) => Ok(Value::Teks(s.clone())),
                LiteralValue::Betul => Ok(Value::Betul),
                LiteralValue::Salah => Ok(Value::Salah),
                LiteralValue::Kosong => Ok(Value::Kosong),
            },
            Expr::Pengenal(nama, pos) => env.borrow().get(nama, *pos),
            Expr::Senarai(items, _) => {
                let mut evaluated = Vec::new();
                for item in items {
                    evaluated.push(self.evaluate_expr(item, env)?);
                }
                Ok(Value::Senarai(Rc::new(RefCell::new(evaluated))))
            }
            Expr::Kamus(pasangan, _) => {
                let mut map = HashMap::new();
                for (k, v) in pasangan {
                    let key_val = self.evaluate_expr(k, env)?;
                    let key_str = match key_val {
                        Value::Teks(s) => s,
                        other => other.to_string(),
                    };
                    let val = self.evaluate_expr(v, env)?;
                    map.insert(key_str, val);
                }
                Ok(Value::Kamus(Rc::new(RefCell::new(map))))
            }
            Expr::Unary { op, expr, pos } => {
                let val = self.evaluate_expr(expr, env)?;
                match op {
                    UnaryOp::Negatif => match val {
                        Value::Nombor(n) => Ok(Value::Nombor(-n)),
                        other => Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        )),
                    },
                    UnaryOp::Bukan => {
                        if val.is_truthy() {
                            Ok(Value::Salah)
                        } else {
                            Ok(Value::Betul)
                        }
                    }
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                pos,
            } => {
                // Short-circuit untuk 'dan' dan 'atau'
                if *op == BinaryOp::Dan {
                    let left_val = self.evaluate_expr(left, env)?;
                    if !left_val.is_truthy() {
                        return Ok(Value::Salah);
                    }
                    let right_val = self.evaluate_expr(right, env)?;
                    return Ok(if right_val.is_truthy() {
                        Value::Betul
                    } else {
                        Value::Salah
                    });
                }
                if *op == BinaryOp::Atau {
                    let left_val = self.evaluate_expr(left, env)?;
                    if left_val.is_truthy() {
                        return Ok(Value::Betul);
                    }
                    let right_val = self.evaluate_expr(right, env)?;
                    return Ok(if right_val.is_truthy() {
                        Value::Betul
                    } else {
                        Value::Salah
                    });
                }

                let left_val = self.evaluate_expr(left, env)?;
                let right_val = self.evaluate_expr(right, env)?;

                match op {
                    BinaryOp::Tambah => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => Ok(Value::Nombor(a + b)),
                        (Value::Teks(a), b) => {
                            let b_str = b.to_string();
                            if a.len() + b_str.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                                return Err(MelayuError::had_memori_terlampau(
                                    "Saiz rentetan teks melebihi had keselamatan maksimum (16 MB)",
                                    pos.line,
                                    pos.col,
                                ));
                            }
                            Ok(Value::Teks(format!("{}{}", a, b_str)))
                        }
                        (a, Value::Teks(b)) => {
                            let a_str = a.to_string();
                            if a_str.len() + b.len() > HAD_SAIZ_TEKS_MAKSIMUM {
                                return Err(MelayuError::had_memori_terlampau(
                                    "Saiz rentetan teks melebihi had keselamatan maksimum (16 MB)",
                                    pos.line,
                                    pos.col,
                                ));
                            }
                            Ok(Value::Teks(format!("{}{}", a_str, b)))
                        }
                        (Value::Senarai(a), Value::Senarai(b)) => {
                            let a_len = a.borrow().len();
                            let b_len = b.borrow().len();
                            if a_len + b_len > HAD_ELEMEN_SENARAI_MAKSIMUM {
                                return Err(MelayuError::had_memori_terlampau(
                                    "Jumlah elemen senarai cantuman melebihi had keselamatan (1,000,000)",
                                    pos.line,
                                    pos.col,
                                ));
                            }
                            let mut combined = a.borrow().clone();
                            combined.extend(b.borrow().clone());
                            Ok(Value::Senarai(Rc::new(RefCell::new(combined))))
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor, teks, atau senarai",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Tolak => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => Ok(Value::Nombor(a - b)),
                        _ => Err(MelayuError::jenis_salah(
                            "nombor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Kali => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => Ok(Value::Nombor(a * b)),
                        _ => Err(MelayuError::jenis_salah(
                            "nombor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Bahagi => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            if *b == 0.0 {
                                return Err(MelayuError::bahagi_kosong(pos.line, pos.col));
                            }
                            Ok(Value::Nombor(a / b))
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Modulo => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            if *b == 0.0 {
                                return Err(MelayuError::bahagi_kosong(pos.line, pos.col));
                            }
                            Ok(Value::Nombor(a % b))
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::SamaDengan => Ok(if left_val == right_val {
                        Value::Betul
                    } else {
                        Value::Salah
                    }),
                    BinaryOp::TidakSama => Ok(if left_val != right_val {
                        Value::Betul
                    } else {
                        Value::Salah
                    }),
                    BinaryOp::Lebih => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            Ok(if a > b { Value::Betul } else { Value::Salah })
                        }
                        (Value::Teks(a), Value::Teks(b)) => {
                            Ok(if a > b { Value::Betul } else { Value::Salah })
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau teks",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Kurang => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            Ok(if a < b { Value::Betul } else { Value::Salah })
                        }
                        (Value::Teks(a), Value::Teks(b)) => {
                            Ok(if a < b { Value::Betul } else { Value::Salah })
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau teks",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::LebihSama => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            Ok(if a >= b { Value::Betul } else { Value::Salah })
                        }
                        (Value::Teks(a), Value::Teks(b)) => {
                            Ok(if a >= b { Value::Betul } else { Value::Salah })
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau teks",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::KurangSama => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => {
                            Ok(if a <= b { Value::Betul } else { Value::Salah })
                        }
                        (Value::Teks(a), Value::Teks(b)) => {
                            Ok(if a <= b { Value::Betul } else { Value::Salah })
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau teks",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Dan | BinaryOp::Atau => unreachable!(),
                }
            }
            Expr::Panggil {
                fungsi,
                argumen,
                pos,
            } => {
                let func_val = self.evaluate_expr(fungsi, env)?;
                let mut evaluated_args = Vec::new();
                for arg in argumen {
                    evaluated_args.push(self.evaluate_expr(arg, env)?);
                }
                self.panggil_nilai(&func_val, &evaluated_args, *pos)
            }
            Expr::AksesIndeks {
                sasaran,
                indeks,
                pos,
            } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                let idx_val = self.evaluate_expr(indeks, env)?;

                match target_val {
                    Value::Senarai(list) => match idx_val {
                        Value::Nombor(n) => {
                            let borrowed = list.borrow();
                            let actual_idx = sahkan_indeks(n, borrowed.len(), *pos)?;
                            Ok(borrowed[actual_idx].clone())
                        }
                        other => Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        )),
                    },
                    Value::Kamus(map) => {
                        let key_str = match idx_val {
                            Value::Teks(s) => s,
                            other => other.to_string(),
                        };
                        let borrowed = map.borrow();
                        Ok(borrowed.get(&key_str).cloned().unwrap_or(Value::Kosong))
                    }
                    Value::Teks(s) => match idx_val {
                        Value::Nombor(n) => {
                            let chars: Vec<char> = s.chars().collect();
                            let actual_idx = sahkan_indeks(n, chars.len(), *pos)?;
                            Ok(Value::Teks(chars[actual_idx].to_string()))
                        }
                        other => Err(MelayuError::jenis_salah(
                            "nombor",
                            other.jenis(),
                            pos.line,
                            pos.col,
                        )),
                    },
                    other => Err(MelayuError::new(
                        "Eh!",
                        &format!("Jenis '{}' tidak menyokong akses indeks.", other.jenis()),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime)),
                }
            }
            Expr::AksesSifat {
                sasaran,
                sifat,
                pos,
            } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                match (target_val, sifat.as_str()) {
                    (Value::Senarai(list), "panjang") => {
                        Ok(Value::Nombor(list.borrow().len() as f64))
                    }
                    (Value::Teks(s), "panjang") => Ok(Value::Nombor(s.chars().count() as f64)),
                    (Value::Kamus(map), "panjang") => Ok(Value::Nombor(map.borrow().len() as f64)),
                    (other, s) => Err(MelayuError::new(
                        "Eh!",
                        &format!(
                            "Sifat '{}' tidak dijumpai pada jenis '{}'.",
                            s,
                            other.jenis()
                        ),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime)),
                }
            }
            Expr::PanggilKaedah {
                sasaran,
                kaedah,
                argumen,
                pos,
            } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                let mut evaluated_args = Vec::new();
                for arg in argumen {
                    evaluated_args.push(self.evaluate_expr(arg, env)?);
                }
                self.panggil_kaedah(target_val, kaedah, &evaluated_args, *pos)
            }
            Expr::BacaInput(pos) => match lakukan_baca_input() {
                Ok(s) => Ok(Value::Teks(s)),
                Err(e) => Err(MelayuError::new(
                    "Alamak!",
                    &format!("Gagal membaca input dari terminal: {}", e),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)),
            },
            Expr::BacaNomborInput(pos) => match lakukan_baca_input() {
                Ok(s) => match s.trim().parse::<f64>() {
                    Ok(n) => Ok(Value::Nombor(n)),
                    Err(_) => Err(MelayuError::new(
                        "Aduh!",
                        &format!(
                            "'{}' bukan nombor yang sah! Sila masukkan angka seperti 42 atau 3.14.",
                            s
                        ),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime)),
                },
                Err(e) => Err(MelayuError::new(
                    "Alamak!",
                    &format!("Gagal membaca input angka: {}", e),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)),
            },
        }
    }
}
