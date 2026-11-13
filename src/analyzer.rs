use crate::ast::*;
use crate::builtins::daftar_fungsi_bawaan;
use crate::errors::{ErrorKind, MelayuError};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Symbol {
    tetap: bool,
    #[allow(dead_code)]
    pos: LineCol,
}

#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}

pub struct Analyzer {
    scopes: Vec<Scope>,
    builtin_names: HashSet<String>,
    loop_depth: usize,
    in_function: bool,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        let mut builtin_names = HashSet::new();
        for (name, _) in daftar_fungsi_bawaan() {
            builtin_names.insert(name.to_string());
        }

        Self {
            scopes: vec![Scope::new()],
            builtin_names,
            loop_depth: 0,
            in_function: false,
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), MelayuError> {
        for stmt in &program.pernyataan {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str, tetap: bool, pos: LineCol) -> Result<(), MelayuError> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.symbols.contains_key(name) {
                return Err(MelayuError::pembolehubah_dah_ada(name, pos.line, pos.col));
            }
            scope
                .symbols
                .insert(name.to_string(), Symbol { tetap, pos });
        }
        Ok(())
    }

    fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }
        None
    }

    fn is_builtin(&self, name: &str) -> bool {
        self.builtin_names.contains(name)
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), MelayuError> {
        match stmt {
            Stmt::IsytiharPembolehubah {
                nama,
                tetap,
                nilai,
                pos,
                ..
            } => {
                // Periksa ungkapan nilai dahulu sebelum deklarasi (mengelak penggunaan diri sendiri)
                self.analyze_expr(nilai)?;
                self.declare(nama, *tetap, *pos)?;
            }
            Stmt::Umpukan {
                sasaran,
                nilai,
                pos,
            } => {
                self.analyze_expr(nilai)?;
                match sasaran {
                    TargetAssign::Pembolehubah(nama) => {
                        if let Some(sym) = self.find_symbol(nama) {
                            if sym.tetap {
                                return Err(MelayuError::ubah_pembolehubah_tetap(
                                    nama, pos.line, pos.col,
                                ));
                            }
                        } else {
                            return Err(MelayuError::pembolehubah_tak_wujud(
                                nama, pos.line, pos.col,
                            ));
                        }
                    }
                    TargetAssign::Indeks { sasaran, indeks } => {
                        self.analyze_expr(sasaran)?;
                        self.analyze_expr(indeks)?;
                    }
                }
            }
            Stmt::Tunjuk(args, _) => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
            }
            Stmt::Jika {
                syarat,
                maka,
                jika_tidak_jika,
                jika_tidak,
                ..
            } => {
                self.analyze_expr(syarat)?;
                self.enter_scope();
                for s in maka {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();

                for (jtj_syarat, jtj_badan) in jika_tidak_jika {
                    self.analyze_expr(jtj_syarat)?;
                    self.enter_scope();
                    for s in jtj_badan {
                        self.analyze_stmt(s)?;
                    }
                    self.exit_scope();
                }

                if let Some(jt_badan) = jika_tidak {
                    self.enter_scope();
                    for s in jt_badan {
                        self.analyze_stmt(s)?;
                    }
                    self.exit_scope();
                }
            }
            Stmt::Selagi { syarat, badan, .. } => {
                self.analyze_expr(syarat)?;
                self.loop_depth += 1;
                self.enter_scope();
                for s in badan {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();
                self.loop_depth -= 1;
            }
            Stmt::UntukDalam {
                pengenal,
                senarai,
                badan,
                pos,
            } => {
                self.analyze_expr(senarai)?;
                self.loop_depth += 1;
                self.enter_scope();
                self.declare(pengenal, false, *pos)?;
                for s in badan {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();
                self.loop_depth -= 1;
            }
            Stmt::UntukJulat {
                pengenal,
                mula,
                hingga,
                langkah,
                badan,
                pos,
            } => {
                self.analyze_expr(mula)?;
                self.analyze_expr(hingga)?;
                if let Some(l) = langkah {
                    self.analyze_expr(l)?;
                }
                self.loop_depth += 1;
                self.enter_scope();
                self.declare(pengenal, false, *pos)?;
                for s in badan {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();
                self.loop_depth -= 1;
            }
            Stmt::DeklarasiFungsi {
                nama,
                parameter,
                badan,
                pos,
                ..
            } => {
                // Daftarkan nama fungsi di skop semasa dahulu supaya menyokong rekursi
                self.declare(nama, true, *pos)?;

                let prev_in_func = self.in_function;
                self.in_function = true;
                self.enter_scope();

                for (p_name, _) in parameter {
                    self.declare(p_name, false, *pos)?;
                }

                for s in badan {
                    self.analyze_stmt(s)?;
                }

                self.exit_scope();
                self.in_function = prev_in_func;
            }
            Stmt::Pulang(expr_opt, pos) => {
                if !self.in_function {
                    return Err(MelayuError::new(
                        "Woy!",
                        "'pulang' hanya boleh digunakan di dalam blok fungsi ('kerja').",
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Analyzer)
                    .with_suggestion("Padamkan 'pulang' atau balut arahan ini dalam fungsi."));
                }
                if let Some(expr) = expr_opt {
                    self.analyze_expr(expr)?;
                }
            }
            Stmt::Hentikan(pos) => {
                if self.loop_depth == 0 {
                    return Err(MelayuError::new(
                        "Woy!",
                        "'hentikan' hanya boleh digunakan di dalam gelung ('selagi' atau 'untuk').",
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Analyzer));
                }
            }
            Stmt::Teruskan(pos) => {
                if self.loop_depth == 0 {
                    return Err(MelayuError::new(
                        "Woy!",
                        "'teruskan' hanya boleh digunakan di dalam gelung ('selagi' atau 'untuk').",
                        pos.line,
                        pos.col,
                    )
                    .with_kind(ErrorKind::Analyzer));
                }
            }
            Stmt::Cuba {
                badan_cuba,
                ralat_var,
                badan_tangkap,
                badan_akhirnya,
                pos,
            } => {
                self.enter_scope();
                for s in badan_cuba {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();

                self.enter_scope();
                self.declare(ralat_var, false, *pos)?;
                for s in badan_tangkap {
                    self.analyze_stmt(s)?;
                }
                self.exit_scope();

                if let Some(akh) = badan_akhirnya {
                    self.enter_scope();
                    for s in akh {
                        self.analyze_stmt(s)?;
                    }
                    self.exit_scope();
                }
            }
            Stmt::Baling(expr, _) => {
                self.analyze_expr(expr)?;
            }
            Stmt::Ungkapan(expr, _) => {
                self.analyze_expr(expr)?;
            }
            Stmt::Horas(_) | Stmt::Mangan(_) => {}
        }
        Ok(())
    }

    fn analyze_expr(&self, expr: &Expr) -> Result<(), MelayuError> {
        match expr {
            Expr::Literal(..) => Ok(()),
            Expr::Pengenal(nama, pos) => {
                if self.find_symbol(nama).is_none() && !self.is_builtin(nama) {
                    return Err(MelayuError::pembolehubah_tak_wujud(nama, pos.line, pos.col));
                }
                Ok(())
            }
            Expr::Senarai(items, _) => {
                for item in items {
                    self.analyze_expr(item)?;
                }
                Ok(())
            }
            Expr::Kamus(pasangan, _) => {
                for (k, v) in pasangan {
                    self.analyze_expr(k)?;
                    self.analyze_expr(v)?;
                }
                Ok(())
            }
            Expr::Unary { expr, .. } => self.analyze_expr(expr),
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
                Ok(())
            }
            Expr::Panggil {
                fungsi, argumen, ..
            } => {
                self.analyze_expr(fungsi)?;
                for arg in argumen {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
            Expr::AksesIndeks {
                sasaran, indeks, ..
            } => {
                self.analyze_expr(sasaran)?;
                self.analyze_expr(indeks)?;
                Ok(())
            }
            Expr::AksesSifat { sasaran, .. } => {
                self.analyze_expr(sasaran)?;
                Ok(())
            }
            Expr::PanggilKaedah {
                sasaran, argumen, ..
            } => {
                self.analyze_expr(sasaran)?;
                for arg in argumen {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
            Expr::BacaInput(_) | Expr::BacaNomborInput(_) => Ok(()),
        }
    }
}
