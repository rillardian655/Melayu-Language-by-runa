use crate::ast::*;
use crate::builtins::{daftar_fungsi_bawaan, lakukan_baca_input};
use crate::errors::{ErrorKind, MelayuError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
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

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
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

        Self { env }
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

    pub fn execute_stmt(
        &mut self,
        stmt: &Stmt,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Flow, MelayuError> {
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
                                    let idx = n as isize;
                                    let actual_idx = if idx < 0 {
                                        (borrowed.len() as isize + idx) as usize
                                    } else {
                                        idx as usize
                                    };
                                    if actual_idx < borrowed.len() {
                                        borrowed[actual_idx] = val;
                                    } else {
                                        return Err(MelayuError::new(
                                            "Aduh!",
                                            &format!(
                                                "Indeks {} di luar julat senarai yang bersaiz {}.",
                                                n,
                                                borrowed.len()
                                            ),
                                            pos.line,
                                            pos.col,
                                        )
                                        .with_kind(ErrorKind::Runtime));
                                    }
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
                        (Value::Teks(a), b) => Ok(Value::Teks(format!("{}{}", a, b))),
                        (a, Value::Teks(b)) => Ok(Value::Teks(format!("{}{}", a, b))),
                        (Value::Senarai(a), Value::Senarai(b)) => {
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

                match func_val {
                    Value::Bawaan(_, func) => func(&evaluated_args, *pos),
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

                        let call_env =
                            Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&closure))));
                        for (i, (p_name, _)) in parameter.iter().enumerate() {
                            call_env
                                .borrow_mut()
                                .define(p_name, evaluated_args[i].clone(), false);
                        }

                        for s in &badan {
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
                            let idx = n as isize;
                            let actual_idx = if idx < 0 {
                                (borrowed.len() as isize + idx) as usize
                            } else {
                                idx as usize
                            };
                            if actual_idx < borrowed.len() {
                                Ok(borrowed[actual_idx].clone())
                            } else {
                                Err(MelayuError::new(
                                    "Aduh!",
                                    &format!(
                                        "Indeks {} di luar julat senarai bersaiz {}.",
                                        n,
                                        borrowed.len()
                                    ),
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime))
                            }
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
                            let idx = n as usize;
                            if idx < chars.len() {
                                Ok(Value::Teks(chars[idx].to_string()))
                            } else {
                                Err(MelayuError::new(
                                    "Aduh!",
                                    &format!(
                                        "Indeks rentetan {} di luar julat panjang {}.",
                                        n,
                                        chars.len()
                                    ),
                                    pos.line,
                                    pos.col,
                                )
                                .with_kind(ErrorKind::Runtime))
                            }
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

                match target_val {
                    Value::Senarai(list) => match kaedah.as_str() {
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
                            list.borrow_mut().push(evaluated_args[0].clone());
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
                                    let idx = n as usize;
                                    if idx < borrowed.len() {
                                        Ok(borrowed.remove(idx))
                                    } else {
                                        Err(MelayuError::new(
                                            "Aduh!",
                                            &format!("Indeks {} di luar julat senarai.", n),
                                            pos.line,
                                            pos.col,
                                        )
                                        .with_kind(ErrorKind::Runtime))
                                    }
                                }
                                ref other => {
                                    Err(MelayuError::jenis_salah("nombor", other.jenis(), pos.line, pos.col))
                                }
                            }
                        }
                        "panjang" => Ok(Value::Nombor(list.borrow().len() as f64)),
                        "berisi" => {
                            if evaluated_args.len() != 1 {
                                return Err(MelayuError::new("Alamak!", "Kaedah '.berisi()' memerlukan 1 hujah.", pos.line, pos.col)
                                    .with_kind(ErrorKind::Runtime));
                            }
                            let borrowed = list.borrow();
                            let found = borrowed.contains(&evaluated_args[0]);
                            Ok(if found { Value::Betul } else { Value::Salah })
                        }
                        other => Err(MelayuError::new(
                            "Eh!",
                            &format!("Kaedah '.{}()' tidak wujud untuk senarai.", other),
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime)
                        .with_suggestion("Kaedah senarai yang ada: .tambah(item), .buang(indeks), .panjang(), .berisi(item)")),
                    },
                    Value::Kamus(map) => match kaedah.as_str() {
                        "kunci" => {
                            let borrowed = map.borrow();
                            let keys: Vec<Value> = borrowed.keys().cloned().map(Value::Teks).collect();
                            Ok(Value::Senarai(Rc::new(RefCell::new(keys))))
                        }
                        "nilai" => {
                            let borrowed = map.borrow();
                            let values: Vec<Value> = borrowed.values().cloned().collect();
                            Ok(Value::Senarai(Rc::new(RefCell::new(values))))
                        }
                        "panjang" => Ok(Value::Nombor(map.borrow().len() as f64)),
                        other => Err(MelayuError::new(
                            "Eh!",
                            &format!("Kaedah '.{}()' tidak wujud untuk kamus.", other),
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime)
                        .with_suggestion("Kaedah kamus yang ada: .kunci(), .nilai(), .panjang()")),
                    },
                    Value::Teks(s) => match kaedah.as_str() {
                        "hurufBesar" => Ok(Value::Teks(s.to_uppercase())),
                        "hurufKecil" => Ok(Value::Teks(s.to_lowercase())),
                        "panjang" => Ok(Value::Nombor(s.chars().count() as f64)),
                        "potong" => Ok(Value::Teks(s.trim().to_string())),
                        other => Err(MelayuError::new(
                            "Eh!",
                            &format!("Kaedah '.{}()' tidak wujud untuk teks.", other),
                            pos.line,
                            pos.col,
                        )
                        .with_kind(ErrorKind::Runtime)
                        .with_suggestion("Kaedah teks yang ada: .hurufBesar(), .hurufKecil(), .panjang(), .potong()")),
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
