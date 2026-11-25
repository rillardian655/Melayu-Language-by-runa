use crate::ast::*;
use crate::builtins::{daftar_fungsi_bawaan, lakukan_baca_input, semak_laluan_selamat};
use crate::errors::{ErrorKind, MelayuError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;

pub type PetaNilai = crate::kamus::PetaTersusun<Value>;

#[derive(Debug, Clone)]
pub struct KaedahKelas {
    pub nama: String,
    pub parameter: Vec<(String, Option<String>)>,
    pub jenis_pulangan: Option<String>,
    pub badan: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl PartialEq for KaedahKelas {
    fn eq(&self, other: &Self) -> bool {
        self.nama == other.nama
            && self.parameter == other.parameter
            && self.jenis_pulangan == other.jenis_pulangan
            && self.badan == other.badan
            && Rc::ptr_eq(&self.closure, &other.closure)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KelasData {
    pub nama: String,
    pub induk: Option<String>,
    pub pembina: Option<KaedahKelas>,
    pub kaedah: HashMap<String, KaedahKelas>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjekData {
    pub nama_kelas: String,
    pub sifat: Rc<RefCell<PetaNilai>>,
    pub kaedah: HashMap<String, KaedahKelas>,
    pub induk: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndukData {
    pub nama_induk: String,
    pub instans_sifat: Rc<RefCell<PetaNilai>>,
    pub instans_kelas: String,
}

#[derive(Debug, Clone)]
pub enum Value {
    Kosong,
    Betul,
    Salah,
    Nombor(f64),
    Teks(String),
    Senarai(Rc<RefCell<Vec<Value>>>),
    Kamus(Rc<RefCell<PetaNilai>>),
    Fungsi {
        nama: String,
        parameter: Vec<(String, Option<String>)>,
        badan: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    Bawaan(String, fn(&[Value], LineCol) -> Result<Value, MelayuError>),
    Kelas(Rc<KelasData>),
    Objek(Rc<ObjekData>),
    Induk(Rc<IndukData>),
    Tensor {
        data: Vec<f64>,
        shape: Vec<usize>,
    },
}

thread_local! {
    static SEDANG_DIPAPAR: RefCell<std::collections::HashSet<usize>> = RefCell::new(std::collections::HashSet::new());
    static SEDANG_DIBANDING: RefCell<std::collections::HashSet<(usize, usize)>> = RefCell::new(std::collections::HashSet::new());
}

fn papar_bekas(
    f: &mut fmt::Formatter<'_>,
    alamat: usize,
    ganti: &str,
    papar: impl FnOnce(&mut fmt::Formatter<'_>) -> fmt::Result,
) -> fmt::Result {
    if !SEDANG_DIPAPAR.with(|s| s.borrow_mut().insert(alamat)) {
        return write!(f, "{}", ganti);
    }
    let hasil = papar(f);
    SEDANG_DIPAPAR.with(|s| s.borrow_mut().remove(&alamat));
    hasil
}

fn banding_bekas(a: usize, b: usize, banding: impl FnOnce() -> bool) -> bool {
    if a == b {
        return true;
    }
    if !SEDANG_DIBANDING.with(|s| s.borrow_mut().insert((a, b))) {
        return true;
    }
    let hasil = banding();
    SEDANG_DIBANDING.with(|s| s.borrow_mut().remove(&(a, b)));
    hasil
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Kosong, Value::Kosong) => true,
            (Value::Betul, Value::Betul) => true,
            (Value::Salah, Value::Salah) => true,
            (Value::Nombor(a), Value::Nombor(b)) => a == b,
            (Value::Teks(a), Value::Teks(b)) => a == b,
            (Value::Senarai(a), Value::Senarai(b)) => banding_bekas(
                Rc::as_ptr(a) as *const () as usize,
                Rc::as_ptr(b) as *const () as usize,
                || *a.borrow() == *b.borrow(),
            ),
            (Value::Kamus(a), Value::Kamus(b)) => banding_bekas(
                Rc::as_ptr(a) as *const () as usize,
                Rc::as_ptr(b) as *const () as usize,
                || *a.borrow() == *b.borrow(),
            ),
            (Value::Fungsi { nama: a, .. }, Value::Fungsi { nama: b, .. }) => a == b,
            (Value::Bawaan(a, _), Value::Bawaan(b, _)) => a == b,
            (Value::Kelas(a), Value::Kelas(b)) => a == b,
            (Value::Objek(a), Value::Objek(b)) => {
                a.nama_kelas == b.nama_kelas
                    && banding_bekas(
                        Rc::as_ptr(&a.sifat) as *const () as usize,
                        Rc::as_ptr(&b.sifat) as *const () as usize,
                        || *a.sifat.borrow() == *b.sifat.borrow(),
                    )
            }
            (Value::Induk(a), Value::Induk(b)) => a == b,
            (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                s1 == s2 && d1 == d2
            }
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
            Value::Kelas(_) => "kelas",
            Value::Objek(_) => "objek",
            Value::Induk(_) => "induk",
            Value::Tensor { .. } => "tensor",
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
            Value::Senarai(list) => papar_bekas(f, Rc::as_ptr(list) as *const () as usize, "[...]", |f| {
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
            }),
            Value::Kamus(map) => papar_bekas(f, Rc::as_ptr(map) as *const () as usize, "kamus {...}", |f| {
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
            }),
            Value::Fungsi { nama, .. } => write!(f, "<kerja: {}>", nama),
            Value::Bawaan(nama, _) => write!(f, "<kerja terbina-dalam: {}>", nama),
            Value::Kelas(k) => write!(f, "<kelas {}>", k.nama),
            Value::Objek(o) => papar_bekas(f, Rc::as_ptr(&o.sifat) as *const () as usize, "<objek ...>", |f| {
                write!(f, "<objek {}: {{", o.nama_kelas)?;
                let borrowed = o.sifat.borrow();
                let mut first = true;
                for (k, v) in borrowed.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}>")
            }),
            Value::Induk(i) => write!(f, "<induk: {}>", i.nama_induk),
            Value::Tensor { data, shape } => {
                if shape.len() == 2 {
                    let (rows, cols) = (shape[0], shape[1]);
                    write!(f, "tensor([")?;
                    for r in 0..rows {
                        if r > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "[")?;
                        for c in 0..cols {
                            if c > 0 {
                                write!(f, ", ")?;
                            }
                            let v = data[r * cols + c];
                            if v.fract() == 0.0 && v.is_finite() {
                                write!(f, "{:.0}", v)?;
                            } else {
                                write!(f, "{:.4}", v)?;
                            }
                        }
                        write!(f, "]")?;
                    }
                    write!(f, "])")
                } else {
                    write!(f, "tensor([")?;
                    for (i, v) in data.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        if v.fract() == 0.0 && v.is_finite() {
                            write!(f, "{:.0}", v)?;
                        } else {
                            write!(f, "{:.4}", v)?;
                        }
                    }
                    write!(f, "])")
                }
            }
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

pub fn had_langkah_dari_persekitaran() -> Option<u64> {
    match std::env::var("MELAYU_HAD_LANGKAH").ok().map(|v| v.trim().to_lowercase()) {
        Some(v) if v == "0" || v == "tiada" => None,
        Some(v) => Some(v.parse().unwrap_or(HAD_LANGKAH_LALAI)),
        None => Some(HAD_LANGKAH_LALAI),
    }
}

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

#[derive(Debug, Clone)]
pub struct KeputusanUjian {
    pub tajuk: String,
    pub lulus: bool,
    pub ralat: Option<String>,
    pub tempoh_ms: f64,
}

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
    pub kedalaman_panggilan: usize,
    pub had_rekursi: usize,
    pub jumlah_langkah: u64,
    pub had_langkah: Option<u64>,
    pub mod_ujian: bool,
    pub keputusan_ujian: Vec<KeputusanUjian>,
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
            had_langkah: had_langkah_dari_persekitaran(),
            mod_ujian: false,
            keputusan_ujian: Vec::new(),
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

    pub fn with_test_mode(mut self, enabled: bool) -> Self {
        self.mod_ujian = enabled;
        self
    }

    pub fn jalankan_ujian(&mut self, program: &Program) -> Result<Vec<KeputusanUjian>, MelayuError> {
        self.mod_ujian = true;
        self.interpret(program)?;
        Ok(self.keputusan_ujian.clone())
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
                let mut hasil = Ok(Value::Kosong);
                for s in badan {
                    match self.execute_stmt(s, &call_env) {
                        Ok(Flow::Lanjut) => {}
                        Ok(Flow::Pulang(v)) => {
                            hasil = Ok(v);
                            break;
                        }
                        Ok(Flow::Hentikan) | Ok(Flow::Teruskan) => {
                            hasil = Err(MelayuError::new(
                                "Woy!",
                                "'hentikan' atau 'teruskan' tidak sah di luar gelung.",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                            break;
                        }
                        Err(e) => {
                            hasil = Err(e);
                            break;
                        }
                    }
                }
                self.kedalaman_panggilan -= 1;
                hasil
            }
            Value::Kelas(ref k) => {
                let instans_sifat = Rc::new(RefCell::new(PetaNilai::new()));
                let instans = Value::Objek(Rc::new(ObjekData {
                    nama_kelas: k.nama.clone(),
                    sifat: Rc::clone(&instans_sifat),
                    kaedah: k.kaedah.clone(),
                    induk: k.induk.clone(),
                }));

                if let Some(ref ctor) = k.pembina {
                    if ctor.parameter.len() != evaluated_args.len() {
                        return Err(MelayuError::new(
                            "Alamak!",
                            &format!(
                                "Pembina 'bina' untuk kelas '{}' memerlukan {} hujah, tetapi menerima {}.",
                                k.nama,
                                ctor.parameter.len(),
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

                    let call_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(
                        &ctor.closure,
                    ))));
                    call_env.borrow_mut().define("diri", instans.clone(), false);
                    if let Some(ref induk_nama) = k.induk {
                        call_env.borrow_mut().define(
                            "induk",
                            Value::Induk(Rc::new(IndukData {
                                nama_induk: induk_nama.clone(),
                                instans_sifat: Rc::clone(&instans_sifat),
                                instans_kelas: k.nama.clone(),
                            })),
                            false,
                        );
                    }

                    for (i, (p_name, _)) in ctor.parameter.iter().enumerate() {
                        call_env
                            .borrow_mut()
                            .define(p_name, evaluated_args[i].clone(), false);
                    }

                    self.kedalaman_panggilan += 1;
                    let hasil = (|| -> Result<(), MelayuError> {
                        for s in &ctor.badan {
                            match self.execute_stmt(s, &call_env)? {
                                Flow::Lanjut => {}
                                Flow::Pulang(_) => return Ok(()),
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
                        Ok(())
                    })();
                    self.kedalaman_panggilan -= 1;
                    hasil?;
                } else if !evaluated_args.is_empty() {
                    return Err(MelayuError::new(
                        "Alamak!",
                        &format!(
                            "Kelas '{}' tidak mempunyai pembina 'bina' dan tidak menerima hujah.",
                            k.nama
                        ),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime));
                }

                Ok(instans)
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
            Value::Tensor { ref data, ref shape } => match kaedah {
                "bentuk" => {
                    let s: Vec<Value> = shape.iter().map(|&d| Value::Nombor(d as f64)).collect();
                    Ok(Value::Senarai(Rc::new(RefCell::new(s))))
                }
                "jumlah" => {
                    let sum: f64 = data.iter().sum();
                    Ok(Value::Nombor(sum))
                }
                "purata" => {
                    let sum: f64 = data.iter().sum();
                    let len = if data.is_empty() { 1.0 } else { data.len() as f64 };
                    Ok(Value::Nombor(sum / len))
                }
                "transposisi" => {
                    if shape.len() != 2 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.transposisi()' memerlukan tensor 2 dimensi.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let (rows, cols) = (shape[0], shape[1]);
                    let mut out = vec![0.0; rows * cols];
                    for r in 0..rows {
                        for c in 0..cols {
                            out[c * rows + r] = data[r * cols + c];
                        }
                    }
                    Ok(Value::Tensor {
                        data: out,
                        shape: vec![cols, rows],
                    })
                }
                "sigmoid" => {
                    let out: Vec<f64> = data.iter().map(|&x| 1.0 / (1.0 + (-x).exp())).collect();
                    Ok(Value::Tensor {
                        data: out,
                        shape: shape.clone(),
                    })
                }
                "relu" => {
                    let out: Vec<f64> = data.iter().map(|&x| if x > 0.0 { x } else { 0.0 }).collect();
                    Ok(Value::Tensor {
                        data: out,
                        shape: shape.clone(),
                    })
                }
                "ubahBentuk" => {
                    if evaluated_args.len() != 1 {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Kaedah '.ubahBentuk(senarai_bentuk)' memerlukan 1 hujah.",
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    let new_shape = match &evaluated_args[0] {
                        Value::Senarai(rc) => {
                            let mut dims = Vec::new();
                            for item in rc.borrow().iter() {
                                match item {
                                    Value::Nombor(n) if *n > 0.0 => dims.push(*n as usize),
                                    _ => {
                                        return Err(MelayuError::new(
                                            "Alamak!",
                                            "Dimensi bentuk mestilah nombor positif.",
                                            pos.line,
                                            pos.col,
                                        )
                                        .with_kind(ErrorKind::Runtime));
                                    }
                                }
                            }
                            dims
                        }
                        _ => {
                            return Err(MelayuError::new(
                                "Alamak!",
                                "Hujah '.ubahBentuk()' mestilah senarai dimensi.",
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    };
                    let prod: usize = new_shape.iter().product();
                    if prod != data.len() {
                        return Err(MelayuError::new(
                            "Alamak!",
                            &format!(
                                "Saiz bentuk baharu ({}) tidak sepadan dengan jumlah elemen tensor ({}).",
                                prod,
                                data.len()
                            ),
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime));
                    }
                    Ok(Value::Tensor {
                        data: data.clone(),
                        shape: new_shape,
                    })
                }
                "keSenarai" => {
                    if shape.len() == 2 {
                        let (rows, cols) = (shape[0], shape[1]);
                        let mut res = Vec::with_capacity(rows);
                        for r in 0..rows {
                            let row_vals: Vec<Value> = (0..cols)
                                .map(|c| Value::Nombor(data[r * cols + c]))
                                .collect();
                            res.push(Value::Senarai(Rc::new(RefCell::new(row_vals))));
                        }
                        Ok(Value::Senarai(Rc::new(RefCell::new(res))))
                    } else {
                        let vals: Vec<Value> = data.iter().map(|&v| Value::Nombor(v)).collect();
                        Ok(Value::Senarai(Rc::new(RefCell::new(vals))))
                    }
                }
                other => Err(MelayuError::new(
                    "Alamak!",
                    &format!("Kaedah '.{}()' tidak wujud untuk jenis 'tensor'.", other),
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Runtime)
                .with_suggestion("Kaedah tensor: .bentuk(), .transposisi(), .jumlah(), .purata(), .sigmoid(), .relu(), .ubahBentuk(bentuk), .keSenarai()")),
            },
            Value::Objek(ref o) => {
                if let Some(method) = o.kaedah.get(kaedah) {
                    if method.parameter.len() != evaluated_args.len() {
                        return Err(MelayuError::new(
                            "Alamak!",
                            &format!(
                                "Kaedah '{}' pada kelas '{}' memerlukan {} hujah, tetapi menerima {}.",
                                kaedah,
                                o.nama_kelas,
                                method.parameter.len(),
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

                    let call_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(
                        &method.closure,
                    ))));
                    let instans = Value::Objek(Rc::clone(o));
                    call_env.borrow_mut().define("diri", instans, false);
                    if let Some(ref induk_nama) = o.induk {
                        call_env.borrow_mut().define(
                            "induk",
                            Value::Induk(Rc::new(IndukData {
                                nama_induk: induk_nama.clone(),
                                instans_sifat: Rc::clone(&o.sifat),
                                instans_kelas: o.nama_kelas.clone(),
                            })),
                            false,
                        );
                    }

                    for (i, (p_name, _)) in method.parameter.iter().enumerate() {
                        call_env
                            .borrow_mut()
                            .define(p_name, evaluated_args[i].clone(), false);
                    }

                    self.kedalaman_panggilan += 1;
                    let hasil = (|| -> Result<Value, MelayuError> {
                        for s in &method.badan {
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
                } else if let Some(field_val) = o.sifat.borrow().get(kaedah).cloned() {
                    self.panggil_nilai(&field_val, evaluated_args, pos)
                } else {
                    Err(MelayuError::new(
                        "Eh!",
                        &format!(
                            "Kaedah '.{}()' tidak dijumpai pada objek kelas '{}'.",
                            kaedah, o.nama_kelas
                        ),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime))
                }
            }
            Value::Induk(ref i) => {
                let parent_class_val = self.env.borrow().get(&i.nama_induk, pos)?;
                match parent_class_val {
                    Value::Kelas(ref parent_k) => {
                        let target_method = if kaedah == "bina" {
                            parent_k.pembina.as_ref()
                        } else {
                            parent_k.kaedah.get(kaedah)
                        };

                        if let Some(method) = target_method {
                            if method.parameter.len() != evaluated_args.len() {
                                return Err(MelayuError::new(
                                    "Alamak!",
                                    &format!(
                                        "Kaedah induk '{}' memerlukan {} hujah, tetapi menerima {}.",
                                        kaedah,
                                        method.parameter.len(),
                                        evaluated_args.len()
                                    ),
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }

                            let call_env = Rc::new(RefCell::new(Environment::with_parent(
                                Rc::clone(&method.closure),
                            )));
                            let instans = Value::Objek(Rc::new(ObjekData {
                                nama_kelas: i.instans_kelas.clone(),
                                sifat: Rc::clone(&i.instans_sifat),
                                kaedah: parent_k.kaedah.clone(),
                                induk: parent_k.induk.clone(),
                            }));
                            call_env.borrow_mut().define("diri", instans, false);
                            if let Some(ref grand_nama) = parent_k.induk {
                                call_env.borrow_mut().define(
                                    "induk",
                                    Value::Induk(Rc::new(IndukData {
                                        nama_induk: grand_nama.clone(),
                                        instans_sifat: Rc::clone(&i.instans_sifat),
                                        instans_kelas: i.instans_kelas.clone(),
                                    })),
                                    false,
                                );
                            }

                            for (i, (p_name, _)) in method.parameter.iter().enumerate() {
                                call_env
                                    .borrow_mut()
                                    .define(p_name, evaluated_args[i].clone(), false);
                            }

                            self.kedalaman_panggilan += 1;
                            let hasil = (|| -> Result<Value, MelayuError> {
                                for s in &method.badan {
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
                        } else {
                            Err(MelayuError::new(
                                "Eh!",
                                &format!(
                                    "Kaedah '.{}()' tidak dijumpai pada kelas induk '{}'.",
                                    kaedah, i.nama_induk
                                ),
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime))
                        }
                    }
                    other => Err(MelayuError::new(
                        "Eh!",
                        &format!("'{}' bukan kelas yang sah.", other.jenis()),
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime)),
                }
            }
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
                            Value::Objek(ref o) => {
                                let key_str = match idx_val {
                                    Value::Teks(s) => s,
                                    other => other.to_string(),
                                };
                                o.sifat.borrow_mut().insert(key_str, val);
                            }
                            other => {
                                return Err(MelayuError::new(
                                    "Eh!",
                                    &format!(
                                        "Hanya senarai, kamus, atau objek yang boleh diindeks, bukan '{}'.",
                                        other.jenis()
                                    ),
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime));
                            }
                        }
                    }
                    TargetAssign::Sifat { sasaran, sifat } => {
                        let target_val = self.evaluate_expr(sasaran, env)?;
                        match target_val {
                            Value::Objek(ref o) => {
                                o.sifat.borrow_mut().insert(sifat.clone(), val);
                            }
                            Value::Kamus(map) => {
                                map.borrow_mut().insert(sifat.clone(), val);
                            }
                            other => {
                                return Err(MelayuError::new(
                                    "Eh!",
                                    &format!(
                                        "Hanya objek atau kamus yang boleh diumpukkan sifat, bukan '{}'.",
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

                let content = if !final_path.exists() {
                    if sasaran == "akal" || sasaran == "akal_ml" || sasaran == "akalml" || sasaran == "std/akal" || sasaran == "std/akal_ml" {
                        crate::builtins::AKAL_ML_STDLIB.to_string()
                    } else if sasaran == "selasih" || sasaran == "wau" || sasaran == "web" || sasaran == "std/selasih" || sasaran == "std/wau" {
                        crate::builtins::SELASIH_STDLIB.to_string()
                    } else {
                        let modul_path = std::path::Path::new("modul_melayu").join(format!("{}.ml", sasaran));
                        let modul_dir_mod = std::path::Path::new("modul_melayu").join(sasaran).join("mod.ml");
                        let modul_dir_utama = std::path::Path::new("modul_melayu").join(sasaran).join("utama.ml");

                        if modul_path.exists() {
                            fs::read_to_string(&modul_path).map_err(|e| {
                                MelayuError::akses_fail_disekat(sasaran, &format!("Gagal membaca fail modul: {}", e), pos.line, pos.col)
                            })?
                        } else if modul_dir_mod.exists() {
                            fs::read_to_string(&modul_dir_mod).map_err(|e| {
                                MelayuError::akses_fail_disekat(sasaran, &format!("Gagal membaca fail modul: {}", e), pos.line, pos.col)
                            })?
                        } else if modul_dir_utama.exists() {
                            fs::read_to_string(&modul_dir_utama).map_err(|e| {
                                MelayuError::akses_fail_disekat(sasaran, &format!("Gagal membaca fail modul: {}", e), pos.line, pos.col)
                            })?
                        } else {
                            return Err(MelayuError::akses_fail_disekat(
                                sasaran,
                                "Fail modul tidak dijumpai (termasuk dalam modul_melayu/).",
                                pos.line,
                                pos.col,
                            ));
                        }
                    }
                } else {
                    fs::read_to_string(&final_path).map_err(|e| {
                        MelayuError::akses_fail_disekat(
                            sasaran,
                            &format!("Gagal membaca fail modul: {}", e),
                            pos.line,
                            pos.col,
                        )
                    })?
                };

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
                    let mut map = PetaNilai::new();
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
            Stmt::DeklarasiKelas(ref d) => {
                let DeklarasiKelasAST {
                    ref nama,
                    ref induk,
                    ref pembina,
                    ref kaedah,
                    pos,
                } = **d;
                let mut methods_map: HashMap<String, KaedahKelas> = HashMap::new();
                let mut resolved_pembina: Option<KaedahKelas> = None;

                if let Some(ref nama_induk) = induk {
                    let parent_val = env.borrow().get(nama_induk, pos)?;
                    match parent_val {
                        Value::Kelas(ref parent_k) => {
                            for (k, v) in &parent_k.kaedah {
                                methods_map.insert(k.clone(), v.clone());
                            }
                            resolved_pembina = parent_k.pembina.clone();
                        }
                        _ => {
                            return Err(MelayuError::new(
                                "Eh!",
                                &format!("'{}' bukan kelas yang sah untuk diwarisi.", nama_induk),
                                pos.line,
                                pos.col,
                            )
                            .with_kind(ErrorKind::Runtime));
                        }
                    }
                }

                if let Some(ref pembina_ast) = pembina {
                    resolved_pembina = Some(KaedahKelas {
                        nama: "bina".to_string(),
                        parameter: pembina_ast.parameter.clone(),
                        jenis_pulangan: None,
                        badan: pembina_ast.badan.clone(),
                        closure: Rc::clone(env),
                    });
                }

                for k_ast in kaedah {
                    methods_map.insert(
                        k_ast.nama.clone(),
                        KaedahKelas {
                            nama: k_ast.nama.clone(),
                            parameter: k_ast.parameter.clone(),
                            jenis_pulangan: k_ast.jenis_pulangan.clone(),
                            badan: k_ast.badan.clone(),
                            closure: Rc::clone(env),
                        },
                    );
                }

                let class_val = Value::Kelas(Rc::new(KelasData {
                    nama: nama.clone(),
                    induk: induk.clone(),
                    pembina: resolved_pembina,
                    kaedah: methods_map,
                }));

                env.borrow_mut().define(nama, class_val, false);
                Ok(Flow::Lanjut)
            }
            Stmt::Horas(_) | Stmt::Mangan(_) => Ok(Flow::Lanjut),
            Stmt::Uji { nama, badan, .. } => {
                if !self.mod_ujian {
                    return Ok(Flow::Lanjut);
                }
                let mula = std::time::Instant::now();
                let test_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
                let mut ralat_ujian: Option<String> = None;
                for s in badan {
                    match self.execute_stmt(s, &test_env) {
                        Ok(Flow::Pulang(_)) | Ok(Flow::Hentikan) | Ok(Flow::Teruskan) | Ok(Flow::Lanjut) => {}
                        Err(e) => {
                            ralat_ujian = Some(format!("{}", e));
                            break;
                        }
                    }
                }
                let tempoh_ms = mula.elapsed().as_secs_f64() * 1000.0;
                self.keputusan_ujian.push(KeputusanUjian {
                    tajuk: nama.clone(),
                    lulus: ralat_ujian.is_none(),
                    ralat: ralat_ujian,
                    tempoh_ms,
                });
                Ok(Flow::Lanjut)
            }
            Stmt::Semak { syarat, mesej, pos } => {
                let cond_val = self.evaluate_expr(syarat, env)?;
                if !cond_val.is_truthy() {
                    let msg = if let Some(m) = mesej {
                        let m_val = self.evaluate_expr(m, env)?;
                        format!("{}", m_val)
                    } else {
                        "Semakan penegasan (semak) tidak benar!".to_string()
                    };
                    return Err(MelayuError::new(
                        "Semakan Gagal!",
                        &msg,
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Runtime));
                }
                Ok(Flow::Lanjut)
            }
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
                let mut map = PetaNilai::new();
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
            } => self.evaluate_binary(left, op, right, *pos, env),
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
                    Value::Objek(ref o) => {
                        let key_str = match idx_val {
                            Value::Teks(s) => s,
                            other => other.to_string(),
                        };
                        let borrowed = o.sifat.borrow();
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
                    (Value::Objek(ref o), "kelas") => Ok(Value::Teks(o.nama_kelas.clone())),
                    (Value::Objek(ref o), s) => {
                        let borrowed = o.sifat.borrow();
                        if let Some(val) = borrowed.get(s) {
                            Ok(val.clone())
                        } else {
                            Ok(Value::Kosong)
                        }
                    }
                    (Value::Senarai(list), "panjang") => {
                        Ok(Value::Nombor(list.borrow().len() as f64))
                    }
                    (Value::Teks(s), "panjang") => Ok(Value::Nombor(s.chars().count() as f64)),
                    (Value::Kamus(map), "panjang") => Ok(Value::Nombor(map.borrow().len() as f64)),
                    (Value::Kamus(map), s) => {
                        let borrowed = map.borrow();
                        Ok(borrowed.get(s).cloned().unwrap_or(Value::Kosong))
                    }
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
            Expr::SaluranPaip { kiri, kanan, pos } => {
                self.evaluate_saluran_paip(kiri, kanan, *pos, env)
            }
            Expr::NullCoalesce { kiri, kanan, .. } => {
                let val_kiri = self.evaluate_expr(kiri, env)?;
                if val_kiri != Value::Kosong {
                    Ok(val_kiri)
                } else {
                    self.evaluate_expr(kanan, env)
                }
            }
            Expr::AksesSelamatSifat { sasaran, sifat, .. } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                if target_val == Value::Kosong {
                    return Ok(Value::Kosong);
                }
                match (target_val, sifat.as_str()) {
                    (Value::Objek(ref o), "kelas") => Ok(Value::Teks(o.nama_kelas.clone())),
                    (Value::Objek(ref o), s) => {
                        let borrowed = o.sifat.borrow();
                        Ok(borrowed.get(s).cloned().unwrap_or(Value::Kosong))
                    }
                    (Value::Senarai(list), "panjang") => {
                        Ok(Value::Nombor(list.borrow().len() as f64))
                    }
                    (Value::Teks(s), "panjang") => Ok(Value::Nombor(s.chars().count() as f64)),
                    (Value::Kamus(map), "panjang") => {
                        Ok(Value::Nombor(map.borrow().len() as f64))
                    }
                    (Value::Kamus(map), s) => {
                        let borrowed = map.borrow();
                        Ok(borrowed.get(s).cloned().unwrap_or(Value::Kosong))
                    }
                    _ => Ok(Value::Kosong),
                }
            }
            Expr::PanggilSelamatKaedah {
                sasaran,
                kaedah,
                argumen,
                pos,
            } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                if target_val == Value::Kosong {
                    return Ok(Value::Kosong);
                }
                let mut evaluated_args = Vec::with_capacity(argumen.len());
                for arg in argumen {
                    evaluated_args.push(self.evaluate_expr(arg, env)?);
                }
                self.panggil_kaedah(target_val, kaedah, &evaluated_args, *pos)
            }
            Expr::Padan(p) => {
                self.evaluate_padan(p, env)
            }
        }
    }

    #[inline(never)]
    fn evaluate_saluran_paip(
        &mut self,
        kiri: &Expr,
        kanan: &Expr,
        pos: LineCol,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, MelayuError> {
        let val_kiri = self.evaluate_expr(kiri, env)?;
        match kanan {
            Expr::Panggil {
                fungsi,
                argumen,
                pos: p_pos,
            } => {
                let func_val = self.evaluate_expr(fungsi, env)?;
                let mut evaluated_args = Vec::with_capacity(argumen.len() + 1);
                evaluated_args.push(val_kiri);
                for arg in argumen {
                    evaluated_args.push(self.evaluate_expr(arg, env)?);
                }
                self.panggil_nilai(&func_val, &evaluated_args, *p_pos)
            }
            Expr::PanggilKaedah {
                sasaran,
                kaedah,
                argumen,
                pos: p_pos,
            } => {
                let target_val = self.evaluate_expr(sasaran, env)?;
                let mut evaluated_args = Vec::with_capacity(argumen.len() + 1);
                evaluated_args.push(val_kiri);
                for arg in argumen {
                    evaluated_args.push(self.evaluate_expr(arg, env)?);
                }
                self.panggil_kaedah(target_val, kaedah, &evaluated_args, *p_pos)
            }
            other => {
                let func_val = self.evaluate_expr(other, env)?;
                self.panggil_nilai(&func_val, &[val_kiri], pos)
            }
        }
    }

    #[inline(never)]
    fn evaluate_padan(
        &mut self,
        p: &PadanAST,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, MelayuError> {
        let target_val = self.evaluate_expr(&p.sasaran, env)?;
        for k in &p.kes {
            let case_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));
            let mut cocok = false;
            match &k.pola {
                PadanPola::Lain => {
                    cocok = true;
                }
                PadanPola::Nilai(val_expr) => {
                    if let Expr::Pengenal(id, _) = &**val_expr {
                        if id == "_" {
                            cocok = true;
                        } else if env.borrow().get(id, LineCol::new(1, 1)).is_ok() {
                            let p_val = self.evaluate_expr(val_expr, env)?;
                            if target_val == p_val {
                                cocok = true;
                            }
                        } else {
                            case_env.borrow_mut().define(id, target_val.clone(), false);
                            cocok = true;
                        }
                    } else {
                        let p_val = self.evaluate_expr(val_expr, env)?;
                        if target_val == p_val {
                            cocok = true;
                        }
                    }
                }
                PadanPola::Julat(mula_expr, tamat_expr) => {
                    let mula_val = self.evaluate_expr(mula_expr, env)?;
                    let tamat_val = self.evaluate_expr(tamat_expr, env)?;
                    if let (
                        Value::Nombor(n),
                        Value::Nombor(mula),
                        Value::Nombor(tamat),
                    ) = (&target_val, &mula_val, &tamat_val)
                    {
                        if *n >= *mula && *n <= *tamat {
                            cocok = true;
                        }
                    }
                }
            }

            if cocok {
                if let Some(ref guard) = k.syarat {
                    let g_val = self.evaluate_expr(guard, &case_env)?;
                    if !g_val.is_truthy() {
                        continue;
                    }
                }
                return self.evaluate_expr(&k.hasil, &case_env);
            }
        }

        if let Some(ref def_expr) = p.lain {
            self.evaluate_expr(def_expr, env)
        } else {
            Err(MelayuError::new(
                "Padanan Tidak Lengkap!",
                &format!(
                    "Tiada corak (pola) yang sepadan untuk nilai '{}'. Sila sertakan 'lain => ...'",
                    target_val
                ),
                p.pos.line,
                p.pos.col,
            )
            .with_kind(ErrorKind::Runtime))
        }
    }

    #[inline(never)]
    fn evaluate_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        pos: LineCol,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, MelayuError> {
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
                        (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                            tensor_binary_op(d1, s1, d2, s2, |a, b| a + b, pos)
                        }
                        (Value::Tensor { data: d, shape: s }, Value::Nombor(n))
                        | (Value::Nombor(n), Value::Tensor { data: d, shape: s }) => Ok(Value::Tensor {
                            data: d.iter().map(|&x| x + n).collect(),
                            shape: s.clone(),
                        }),
                        _ => Err(MelayuError::jenis_salah(
                            "nombor, teks, senarai, atau tensor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Tolak => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => Ok(Value::Nombor(a - b)),
                        (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                            tensor_binary_op(d1, s1, d2, s2, |a, b| a - b, pos)
                        }
                        (Value::Tensor { data: d, shape: s }, Value::Nombor(n)) => Ok(Value::Tensor {
                            data: d.iter().map(|&x| x - n).collect(),
                            shape: s.clone(),
                        }),
                        (Value::Nombor(n), Value::Tensor { data: d, shape: s }) => Ok(Value::Tensor {
                            data: d.iter().map(|&x| n - x).collect(),
                            shape: s.clone(),
                        }),
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau tensor",
                            &format!("{} dan {}", left_val.jenis(), right_val.jenis()),
                            pos.line,
                            pos.col,
                        )),
                    },
                    BinaryOp::Kali => match (&left_val, &right_val) {
                        (Value::Nombor(a), Value::Nombor(b)) => Ok(Value::Nombor(a * b)),
                        (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                            tensor_binary_op(d1, s1, d2, s2, |a, b| a * b, pos)
                        }
                        (Value::Tensor { data: d, shape: s }, Value::Nombor(n))
                        | (Value::Nombor(n), Value::Tensor { data: d, shape: s }) => Ok(Value::Tensor {
                            data: d.iter().map(|&x| x * n).collect(),
                            shape: s.clone(),
                        }),
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau tensor",
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
                        (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                            if d2.iter().any(|&x| x == 0.0) {
                                return Err(MelayuError::bahagi_kosong(pos.line, pos.col));
                            }
                            tensor_binary_op(d1, s1, d2, s2, |a, b| a / b, pos)
                        }
                        (Value::Tensor { data: d, shape: s }, Value::Nombor(n)) => {
                            if *n == 0.0 {
                                return Err(MelayuError::bahagi_kosong(pos.line, pos.col));
                            }
                            Ok(Value::Tensor {
                                data: d.iter().map(|&x| x / n).collect(),
                                shape: s.clone(),
                            })
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "nombor atau tensor",
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
                    BinaryOp::MatriksDarab => match (&left_val, &right_val) {
                        (Value::Tensor { data: d1, shape: s1 }, Value::Tensor { data: d2, shape: s2 }) => {
                            tensor_matmul(d1, s1, d2, s2, pos)
                        }
                        _ => Err(MelayuError::jenis_salah(
                            "tensor",
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
}

fn tensor_binary_op<F>(
    d1: &[f64],
    s1: &[usize],
    d2: &[f64],
    s2: &[usize],
    op: F,
    pos: LineCol,
) -> Result<Value, MelayuError>
where
    F: Fn(f64, f64) -> f64,
{
    if s1 == s2 {
        let out: Vec<f64> = d1.iter().zip(d2.iter()).map(|(&a, &b)| op(a, b)).collect();
        return Ok(Value::Tensor {
            data: out,
            shape: s1.to_vec(),
        });
    }
    // Broadcasting 1D [cols] ke atas 2D [rows, cols]
    if s1.len() == 2 && s2.len() == 1 && s1[1] == s2[0] {
        let (rows, cols) = (s1[0], s1[1]);
        let mut out = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                out.push(op(d1[r * cols + c], d2[c]));
            }
        }
        return Ok(Value::Tensor {
            data: out,
            shape: s1.to_vec(),
        });
    }
    // Broadcasting 2D [1, cols] ke atas 2D [rows, cols]
    if s1.len() == 2 && s2.len() == 2 && s2[0] == 1 && s1[1] == s2[1] {
        let (rows, cols) = (s1[0], s1[1]);
        let mut out = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                out.push(op(d1[r * cols + c], d2[c]));
            }
        }
        return Ok(Value::Tensor {
            data: out,
            shape: s1.to_vec(),
        });
    }
    Err(MelayuError::new(
        "Alamak!",
        &format!("Dimensi tensor tidak sepadan untuk operasi: {:?} dan {:?}", s1, s2),
        pos.line,
        pos.col,
    )
    .with_kind(ErrorKind::Runtime))
}

fn tensor_matmul(
    d1: &[f64],
    s1: &[usize],
    d2: &[f64],
    s2: &[usize],
    pos: LineCol,
) -> Result<Value, MelayuError> {
    if s1.len() == 2 && s2.len() == 2 {
        let (m, k1) = (s1[0], s1[1]);
        let (k2, n) = (s2[0], s2[1]);
        if k1 != k2 {
            return Err(MelayuError::new(
                "Alamak!",
                &format!("Dimensi darab matriks (@) tidak sepadan: {:?} @ {:?}", s1, s2),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime));
        }
        if m * k1 * n >= 262_144 {
            if let Some(gpu_out) = crate::gpu::gpu_matmul(d1, d2, m, k1, n) {
                return Ok(Value::Tensor {
                    data: gpu_out,
                    shape: vec![m, n],
                });
            }
        }

        let mut out = vec![0.0; m * n];
        let num_threads = std::thread::available_parallelism()
            .map(|t| t.get())
            .unwrap_or(1)
            .min(m);

        if m * k1 * n >= 32_768 && num_threads > 1 {
            let chunk_rows = m.div_ceil(num_threads);
            std::thread::scope(|scope| {
                for (t_idx, out_chunk) in out.chunks_mut(chunk_rows * n).enumerate() {
                    let row_start = t_idx * chunk_rows;
                    let rows_in_chunk = out_chunk.len() / n;
                    scope.spawn(move || {
                        for local_r in 0..rows_in_chunk {
                            let i = row_start + local_r;
                            let out_row = &mut out_chunk[local_r * n..(local_r + 1) * n];
                            for k in 0..k1 {
                                let a = d1[i * k1 + k];
                                let b_row = &d2[k * n..(k + 1) * n];
                                for j in 0..n {
                                    out_row[j] += a * b_row[j];
                                }
                            }
                        }
                    });
                }
            });
        } else {
            for i in 0..m {
                let out_row = &mut out[i * n..(i + 1) * n];
                for k in 0..k1 {
                    let a = d1[i * k1 + k];
                    let b_row = &d2[k * n..(k + 1) * n];
                    for j in 0..n {
                        out_row[j] += a * b_row[j];
                    }
                }
            }
        }
        Ok(Value::Tensor {
            data: out,
            shape: vec![m, n],
        })
    } else if s1.len() == 2 && s2.len() == 1 {
        let (m, k1) = (s1[0], s1[1]);
        let k2 = s2[0];
        if k1 != k2 {
            return Err(MelayuError::new(
                "Alamak!",
                &format!("Dimensi darab matriks (@) tidak sepadan: {:?} @ {:?}", s1, s2),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime));
        }
        let mut out = vec![0.0; m];
        for i in 0..m {
            let mut sum = 0.0;
            for k in 0..k1 {
                sum += d1[i * k1 + k] * d2[k];
            }
            out[i] = sum;
        }
        Ok(Value::Tensor {
            data: out,
            shape: vec![m],
        })
    } else if s1.len() == 1 && s2.len() == 1 {
        if s1[0] != s2[0] {
            return Err(MelayuError::new(
                "Alamak!",
                &format!("Dimensi titik darab vektor (@) tidak sepadan: {:?} @ {:?}", s1, s2),
                pos.line,
                pos.col,
            )
            .with_kind(ErrorKind::Runtime));
        }
        let sum: f64 = d1.iter().zip(d2.iter()).map(|(&a, &b)| a * b).sum();
        Ok(Value::Nombor(sum))
    } else {
        Err(MelayuError::new(
            "Alamak!",
            &format!("Operasi '@' menyokong tensor 1D atau 2D, dapat {:?} dan {:?}", s1, s2),
            pos.line,
            pos.col,
        )
        .with_kind(ErrorKind::Runtime))
    }
}
