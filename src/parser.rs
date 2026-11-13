use crate::ast::*;
use crate::errors::{ErrorKind, MelayuError};
use crate::tokens::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, MelayuError> {
        let mut stmts = Vec::new();
        self.skip_newlines();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
            self.skip_newlines();
        }

        Ok(Program { pernyataan: stmts })
    }

    // ----------------------------------------------------
    // Penghuraian Pernyataan (Statements)
    // ----------------------------------------------------

    fn parse_statement(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.peek();
        let pos = LineCol::new(tok.line, tok.column);

        match &tok.kind {
            TokenKind::Ada => self.parse_var_decl(false),
            TokenKind::Tetap => self.parse_var_decl(true),
            TokenKind::Tunjuk => self.parse_tunjuk(),
            TokenKind::Jika => self.parse_jika(),
            TokenKind::Selagi => self.parse_selagi(),
            TokenKind::Untuk => self.parse_untuk(),
            TokenKind::Kerja => self.parse_kerja(),
            TokenKind::Pulang => self.parse_pulang(),
            TokenKind::Hentikan => {
                self.advance();
                self.expect_stmt_delimiter()?;
                Ok(Stmt::Hentikan(pos))
            }
            TokenKind::Teruskan => {
                self.advance();
                self.expect_stmt_delimiter()?;
                Ok(Stmt::Teruskan(pos))
            }
            TokenKind::Cuba => self.parse_cuba(),
            TokenKind::Baling => self.parse_baling(),
            TokenKind::Horas => {
                self.advance();
                self.expect_stmt_delimiter()?;
                Ok(Stmt::Horas(pos))
            }
            TokenKind::Mangan => {
                self.advance();
                self.expect_stmt_delimiter()?;
                Ok(Stmt::Mangan(pos))
            }
            _ => self.parse_assignment_or_expr_stmt(),
        }
    }

    fn parse_ident_token(&mut self) -> Option<String> {
        match self.peek().kind {
            TokenKind::Pengenal(ref n) => {
                let name = n.clone();
                self.advance();
                Some(name)
            }
            TokenKind::TipeNombor => {
                self.advance();
                Some("nombor".to_string())
            }
            TokenKind::TipeTeks => {
                self.advance();
                Some("teks".to_string())
            }
            TokenKind::Senarai => {
                self.advance();
                Some("senarai".to_string())
            }
            _ => None,
        }
    }

    fn parse_var_decl(&mut self, tetap: bool) -> Result<Stmt, MelayuError> {
        let start_tok = self.advance(); // lewati 'ada' atau 'tetap'
        let pos = LineCol::new(start_tok.line, start_tok.column);

        let nama = match self.parse_ident_token() {
            Some(n) => n,
            None => {
                let cur = self.peek();
                return Err(MelayuError::new(
                    "Alamak!",
                    &format!(
                        "Diharapkan nama pembolehubah selepas '{}', tetapi dapat '{}'.",
                        start_tok.lexeme, cur.lexeme
                    ),
                    cur.line,
                    cur.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion("Contoh: ada nama = \"Budi\" atau tetap PI = 3.14"));
            }
        };

        // Semak anotasi jenis pilihan (: jenis)
        let mut jenis_anotasi = None;
        if self.match_token(TokenKind::TitikDua) {
            let type_tok = self.advance();
            jenis_anotasi = Some(type_tok.lexeme.clone());
        }

        // Mesti ada '='
        if !self.match_token(TokenKind::SamaDenganSatu) {
            let cur = self.peek();
            return Err(MelayuError::new(
                "Alamak!",
                &format!(
                    "Nilai pemula diperlukan untuk pembolehubah '{}'. Diharapkan '='.",
                    nama
                ),
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion(&format!("Tulis: {} {} = <nilai>", start_tok.lexeme, nama)));
        }

        let nilai = self.parse_expression()?;
        self.expect_stmt_delimiter()?;

        Ok(Stmt::IsytiharPembolehubah {
            nama,
            tetap,
            jenis_anotasi,
            nilai,
            pos,
        })
    }

    fn parse_tunjuk(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance();
        let pos = LineCol::new(tok.line, tok.column);

        let mut args = Vec::new();

        // Tunjuk boleh tanpa argumen (cetak baris kosong)
        if !self.check_stmt_delimiter() && !self.is_at_end() {
            args.push(self.parse_expression()?);
            while self.match_token(TokenKind::Koma) {
                args.push(self.parse_expression()?);
            }
        }

        self.expect_stmt_delimiter()?;
        Ok(Stmt::Tunjuk(args, pos))
    }

    fn parse_jika(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // lewati 'jika'
        let pos = LineCol::new(tok.line, tok.column);
        let start_line = tok.line;

        let syarat = self.parse_expression()?;
        self.expect_stmt_delimiter()?;

        let mut maka = Vec::new();
        let mut jika_tidak_jika = Vec::new();
        let mut jika_tidak = None;

        while !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::Selesai)
                || self.check(TokenKind::JikaTidakJika)
                || self.check(TokenKind::JikaTidak)
            {
                break;
            }
            maka.push(self.parse_statement()?);
        }

        while self.match_token(TokenKind::JikaTidakJika) {
            let jtj_syarat = self.parse_expression()?;
            self.expect_stmt_delimiter()?;
            let mut jtj_badan = Vec::new();
            while !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::Selesai)
                    || self.check(TokenKind::JikaTidakJika)
                    || self.check(TokenKind::JikaTidak)
                {
                    break;
                }
                jtj_badan.push(self.parse_statement()?);
            }
            jika_tidak_jika.push((jtj_syarat, jtj_badan));
        }

        if self.match_token(TokenKind::JikaTidak) {
            self.expect_stmt_delimiter()?;
            let mut jt_badan = Vec::new();
            while !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::Selesai) {
                    break;
                }
                jt_badan.push(self.parse_statement()?);
            }
            jika_tidak = Some(jt_badan);
        }

        if !self.match_token(TokenKind::Selesai) {
            let cur = self.peek();
            return Err(MelayuError::blok_tak_tutup(
                "jika", start_line, cur.line, cur.column,
            ));
        }

        self.expect_stmt_delimiter()?;

        Ok(Stmt::Jika {
            syarat,
            maka,
            jika_tidak_jika,
            jika_tidak,
            pos,
        })
    }

    fn parse_selagi(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'selagi'
        let pos = LineCol::new(tok.line, tok.column);
        let start_line = tok.line;

        let syarat = self.parse_expression()?;
        self.expect_stmt_delimiter()?;

        let mut badan = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::Selesai) {
                break;
            }
            badan.push(self.parse_statement()?);
        }

        if !self.match_token(TokenKind::Selesai) {
            let cur = self.peek();
            return Err(MelayuError::blok_tak_tutup(
                "selagi", start_line, cur.line, cur.column,
            ));
        }

        self.expect_stmt_delimiter()?;
        Ok(Stmt::Selagi { syarat, badan, pos })
    }

    fn parse_untuk(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'untuk'
        let pos = LineCol::new(tok.line, tok.column);
        let start_line = tok.line;

        let var_name = match self.parse_ident_token() {
            Some(n) => n,
            None => {
                let cur = self.peek();
                return Err(MelayuError::new(
                    "Alamak!",
                    "Diharapkan nama pembolehubah selepas 'untuk'.",
                    cur.line,
                    cur.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion(
                    "Contoh: untuk i dari 1 hingga 10 atau untuk item dalam senarai",
                ));
            }
        };

        // Semak sama ada 'dalam' atau 'dari'
        if self.match_token(TokenKind::Dalam) {
            let senarai = self.parse_expression()?;
            self.expect_stmt_delimiter()?;

            let mut badan = Vec::new();
            while !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::Selesai) {
                    break;
                }
                badan.push(self.parse_statement()?);
            }

            if !self.match_token(TokenKind::Selesai) {
                let cur = self.peek();
                return Err(MelayuError::blok_tak_tutup(
                    "untuk", start_line, cur.line, cur.column,
                ));
            }
            self.expect_stmt_delimiter()?;

            Ok(Stmt::UntukDalam {
                pengenal: var_name,
                senarai,
                badan,
                pos,
            })
        } else if self.match_token(TokenKind::Dari) {
            let mula = self.parse_expression()?;

            if !self.match_token(TokenKind::Hingga) {
                let cur = self.peek();
                return Err(MelayuError::new(
                    "Alamak!",
                    "Diharapkan 'hingga' selepas nilai awal julat.",
                    cur.line,
                    cur.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion("Contoh: untuk i dari 0 hingga 10"));
            }

            let hingga = self.parse_expression()?;

            let mut langkah = None;
            if self.match_token(TokenKind::Langkah) {
                langkah = Some(self.parse_expression()?);
            }

            self.expect_stmt_delimiter()?;

            let mut badan = Vec::new();
            while !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::Selesai) {
                    break;
                }
                badan.push(self.parse_statement()?);
            }

            if !self.match_token(TokenKind::Selesai) {
                let cur = self.peek();
                return Err(MelayuError::blok_tak_tutup(
                    "untuk", start_line, cur.line, cur.column,
                ));
            }
            self.expect_stmt_delimiter()?;

            Ok(Stmt::UntukJulat {
                pengenal: var_name,
                mula,
                hingga,
                langkah,
                badan,
                pos,
            })
        } else {
            let cur = self.peek();
            Err(MelayuError::new(
                "Alamak!",
                &format!("Diharapkan 'dalam' atau 'dari' selepas pembolehubah 'untuk', tetapi dapat '{}'.", cur.lexeme),
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion("Gunakan: 'untuk x dalam senarai' atau 'untuk i dari 1 hingga 10'"))
        }
    }

    fn parse_kerja(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'kerja'
        let pos = LineCol::new(tok.line, tok.column);
        let start_line = tok.line;

        let nama = match self.parse_ident_token() {
            Some(n) => n,
            None => {
                let cur = self.peek();
                return Err(MelayuError::new(
                    "Alamak!",
                    "Diharapkan nama fungsi selepas 'kerja'.",
                    cur.line,
                    cur.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion("Contoh: kerja kiraGaji(jam, kadar) ... selesai"));
            }
        };

        if !self.match_token(TokenKind::KurungBuka) {
            let cur = self.peek();
            return Err(MelayuError::new(
                "Alamak!",
                &format!("Diharapkan '(' selepas nama fungsi '{}'.", nama),
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion("Contoh: kerja nama(parameter1, parameter2)"));
        }

        let mut parameter = Vec::new();
        if !self.check(TokenKind::KurungTutup) {
            loop {
                let p_name = match self.parse_ident_token() {
                    Some(n) => n,
                    None => {
                        let cur = self.peek();
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Nama parameter fungsi tidak sah.",
                            cur.line,
                            cur.column,
                        )
                        .with_kind(ErrorKind::Parser));
                    }
                };

                let mut p_type = None;
                if self.match_token(TokenKind::TitikDua) {
                    let type_tok = self.advance();
                    p_type = Some(type_tok.lexeme.clone());
                }

                parameter.push((p_name, p_type));

                if !self.match_token(TokenKind::Koma) {
                    break;
                }
            }
        }

        if !self.match_token(TokenKind::KurungTutup) {
            let cur = self.peek();
            return Err(MelayuError::new(
                "Alamak!",
                "Diharapkan ')' untuk menutup senarai parameter fungsi.",
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser));
        }

        // Semak anotasi jenis pulangan pilihan: pulang <jenis>
        let mut jenis_pulangan = None;
        if self.match_token(TokenKind::Pulang) {
            let ret_type_tok = self.advance();
            jenis_pulangan = Some(ret_type_tok.lexeme.clone());
        }

        self.expect_stmt_delimiter()?;

        let mut badan = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::Selesai) {
                break;
            }
            badan.push(self.parse_statement()?);
        }

        if !self.match_token(TokenKind::Selesai) {
            let cur = self.peek();
            return Err(MelayuError::blok_tak_tutup(
                &format!("fungsi '{}'", nama),
                start_line,
                cur.line,
                cur.column,
            ));
        }
        self.expect_stmt_delimiter()?;

        Ok(Stmt::DeklarasiFungsi {
            nama,
            parameter,
            jenis_pulangan,
            badan,
            pos,
        })
    }

    fn parse_pulang(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'pulang'
        let pos = LineCol::new(tok.line, tok.column);

        let val = if self.check_stmt_delimiter() || self.is_at_end() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_stmt_delimiter()?;
        Ok(Stmt::Pulang(val, pos))
    }

    fn parse_cuba(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'cuba'
        let pos = LineCol::new(tok.line, tok.column);
        let start_line = tok.line;
        self.expect_stmt_delimiter()?;

        let mut badan_cuba = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::Tangkap) {
                break;
            }
            badan_cuba.push(self.parse_statement()?);
        }

        if !self.match_token(TokenKind::Tangkap) {
            let cur = self.peek();
            return Err(MelayuError::new(
                "Alamak!",
                "Blok 'cuba' memerlukan bahagian 'tangkap <pembolehubah>'.",
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion("Contoh: cuba ... tangkap e ... selesai"));
        }

        let var_tok = self.advance();
        let ralat_var = match var_tok.kind {
            TokenKind::Pengenal(n) => n,
            _ => {
                return Err(MelayuError::new(
                    "Alamak!",
                    "Diharapkan nama pembolehubah ralat selepas 'tangkap'.",
                    var_tok.line,
                    var_tok.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion("Contoh: tangkap e"));
            }
        };

        self.expect_stmt_delimiter()?;

        let mut badan_tangkap = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::Akhirnya) || self.check(TokenKind::Selesai) {
                break;
            }
            badan_tangkap.push(self.parse_statement()?);
        }

        let mut badan_akhirnya = None;
        if self.match_token(TokenKind::Akhirnya) {
            self.expect_stmt_delimiter()?;
            let mut akh = Vec::new();
            while !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::Selesai) {
                    break;
                }
                akh.push(self.parse_statement()?);
            }
            badan_akhirnya = Some(akh);
        }

        if !self.match_token(TokenKind::Selesai) {
            let cur = self.peek();
            return Err(MelayuError::blok_tak_tutup(
                "cuba-tangkap",
                start_line,
                cur.line,
                cur.column,
            ));
        }
        self.expect_stmt_delimiter()?;

        Ok(Stmt::Cuba {
            badan_cuba,
            ralat_var,
            badan_tangkap,
            badan_akhirnya,
            pos,
        })
    }

    fn parse_baling(&mut self) -> Result<Stmt, MelayuError> {
        let tok = self.advance(); // 'baling'
        let pos = LineCol::new(tok.line, tok.column);
        let expr = self.parse_expression()?;
        self.expect_stmt_delimiter()?;
        Ok(Stmt::Baling(expr, pos))
    }

    fn parse_assignment_or_expr_stmt(&mut self) -> Result<Stmt, MelayuError> {
        let expr = self.parse_expression()?;
        let pos = expr.pos();

        if self.match_token(TokenKind::SamaDenganSatu) {
            let nilai = self.parse_expression()?;
            self.expect_stmt_delimiter()?;

            match expr {
                Expr::Pengenal(nama, _) => Ok(Stmt::Umpukan {
                    sasaran: TargetAssign::Pembolehubah(nama),
                    nilai,
                    pos,
                }),
                Expr::AksesIndeks { sasaran, indeks, .. } => Ok(Stmt::Umpukan {
                    sasaran: TargetAssign::Indeks { sasaran, indeks },
                    nilai,
                    pos,
                }),
                _ => Err(MelayuError::new(
                    "Alamak!",
                    "Sasaran umpukan tidak sah. Hanya pembolehubah atau indeks yang boleh diumpukkan.",
                    pos.line,
                    pos.col,
                )
                .with_kind(ErrorKind::Parser)),
            }
        } else {
            self.expect_stmt_delimiter()?;
            Ok(Stmt::Ungkapan(expr, pos))
        }
    }

    // ----------------------------------------------------
    // Penghuraian Ungkapan (Expressions)
    // ----------------------------------------------------

    pub fn parse_expression(&mut self) -> Result<Expr, MelayuError> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_logic_and()?;

        while self.check(TokenKind::Atau) {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let right = self.parse_logic_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Atau,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_logic_and(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_equality()?;

        while self.check(TokenKind::Dan) {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Dan,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_comparison()?;

        while self.check(TokenKind::SamaDengan) || self.check(TokenKind::TidakSama) {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let op = if op_tok.kind == TokenKind::SamaDengan {
                BinaryOp::SamaDengan
            } else {
                BinaryOp::TidakSama
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_term()?;

        while self.check(TokenKind::Lebih)
            || self.check(TokenKind::Kurang)
            || self.check(TokenKind::LebihSama)
            || self.check(TokenKind::KurangSama)
        {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let op = match op_tok.kind {
                TokenKind::Lebih => BinaryOp::Lebih,
                TokenKind::Kurang => BinaryOp::Kurang,
                TokenKind::LebihSama => BinaryOp::LebihSama,
                TokenKind::KurangSama => BinaryOp::KurangSama,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_factor()?;

        while self.check(TokenKind::Tambah) || self.check(TokenKind::Tolak) {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let op = if op_tok.kind == TokenKind::Tambah {
                BinaryOp::Tambah
            } else {
                BinaryOp::Tolak
            };
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, MelayuError> {
        let mut left = self.parse_unary()?;

        while self.check(TokenKind::Kali)
            || self.check(TokenKind::Bahagi)
            || self.check(TokenKind::Modulo)
        {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let op = match op_tok.kind {
                TokenKind::Kali => BinaryOp::Kali,
                TokenKind::Bahagi => BinaryOp::Bahagi,
                TokenKind::Modulo => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos,
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, MelayuError> {
        if self.check(TokenKind::Tolak) || self.check(TokenKind::Bukan) {
            let op_tok = self.advance();
            let pos = LineCol::new(op_tok.line, op_tok.column);
            let op = if op_tok.kind == TokenKind::Tolak {
                UnaryOp::Negatif
            } else {
                UnaryOp::Bukan
            };
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
                pos,
            });
        }

        self.parse_call_and_index()
    }

    fn parse_call_and_index(&mut self) -> Result<Expr, MelayuError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(TokenKind::KurungBuka) {
                // Panggilan fungsi
                let pos = expr.pos();
                let mut args = Vec::new();
                if !self.check(TokenKind::KurungTutup) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(TokenKind::Koma) {
                            break;
                        }
                    }
                }
                if !self.match_token(TokenKind::KurungTutup) {
                    let cur = self.peek();
                    return Err(MelayuError::new(
                        "Alamak!",
                        "Diharapkan ')' untuk menutup panggilan fungsi.",
                        cur.line,
                        cur.column,
                    )
                    .with_kind(ErrorKind::Parser));
                }
                expr = Expr::Panggil {
                    fungsi: Box::new(expr),
                    argumen: args,
                    pos,
                };
            } else if self.match_token(TokenKind::KurungSikuBuka) {
                // Akses indeks
                let pos = expr.pos();
                let indeks = self.parse_expression()?;
                if !self.match_token(TokenKind::KurungSikuTutup) {
                    let cur = self.peek();
                    return Err(MelayuError::new(
                        "Alamak!",
                        "Diharapkan ']' untuk menutup akses indeks.",
                        cur.line,
                        cur.column,
                    )
                    .with_kind(ErrorKind::Parser));
                }
                expr = Expr::AksesIndeks {
                    sasaran: Box::new(expr),
                    indeks: Box::new(indeks),
                    pos,
                };
            } else if self.match_token(TokenKind::Titik) {
                // Akses kaedah atau sifat: .tambah(6) atau .kunci()
                let pos = expr.pos();
                let name_tok = self.advance();
                let name = match name_tok.kind {
                    TokenKind::Pengenal(n) => n,
                    TokenKind::Tambah => "tambah".to_string(),
                    TokenKind::Tolak => "tolak".to_string(),
                    _ => {
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Diharapkan nama kaedah selepas titik ('.').",
                            name_tok.line,
                            name_tok.column,
                        )
                        .with_kind(ErrorKind::Parser)
                        .with_suggestion("Contoh: senarai.tambah(6) atau kamus.kunci()"));
                    }
                };

                if self.match_token(TokenKind::KurungBuka) {
                    let mut args = Vec::new();
                    if !self.check(TokenKind::KurungTutup) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(TokenKind::Koma) {
                                break;
                            }
                        }
                    }
                    if !self.match_token(TokenKind::KurungTutup) {
                        let cur = self.peek();
                        return Err(MelayuError::new(
                            "Alamak!",
                            "Diharapkan ')' untuk menutup hujah kaedah.",
                            cur.line,
                            cur.column,
                        )
                        .with_kind(ErrorKind::Parser));
                    }
                    expr = Expr::PanggilKaedah {
                        sasaran: Box::new(expr),
                        kaedah: name,
                        argumen: args,
                        pos,
                    };
                } else {
                    expr = Expr::AksesSifat {
                        sasaran: Box::new(expr),
                        sifat: name,
                        pos,
                    };
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, MelayuError> {
        let tok = self.advance();
        let pos = LineCol::new(tok.line, tok.column);

        match tok.kind {
            TokenKind::Nombor(n) => Ok(Expr::Literal(LiteralValue::Nombor(n), pos)),
            TokenKind::Teks(s) => Ok(Expr::Literal(LiteralValue::Teks(s), pos)),
            TokenKind::Betul => Ok(Expr::Literal(LiteralValue::Betul, pos)),
            TokenKind::Salah => Ok(Expr::Literal(LiteralValue::Salah, pos)),
            TokenKind::Kosong => Ok(Expr::Literal(LiteralValue::Kosong, pos)),
            TokenKind::Pengenal(name) => Ok(Expr::Pengenal(name, pos)),
            TokenKind::TipeNombor => Ok(Expr::Pengenal("nombor".to_string(), pos)),
            TokenKind::TipeTeks => Ok(Expr::Pengenal("teks".to_string(), pos)),
            TokenKind::Senarai => Ok(Expr::Pengenal("senarai".to_string(), pos)),

            // Senarai literal: [e1, e2, ...]
            TokenKind::KurungSikuBuka => {
                let mut items = Vec::new();
                if !self.check(TokenKind::KurungSikuTutup) {
                    loop {
                        items.push(self.parse_expression()?);
                        if !self.match_token(TokenKind::Koma) {
                            break;
                        }
                    }
                }
                if !self.match_token(TokenKind::KurungSikuTutup) {
                    let cur = self.peek();
                    return Err(MelayuError::new(
                        "Alamak!",
                        "Diharapkan ']' untuk menutup senarai.",
                        cur.line,
                        cur.column,
                    )
                    .with_kind(ErrorKind::Parser));
                }
                Ok(Expr::Senarai(items, pos))
            }

            // Kamus literal: kamus { ... } atau { ... }
            TokenKind::Kamus => {
                if !self.match_token(TokenKind::KurungKurawalBuka) {
                    let cur = self.peek();
                    return Err(MelayuError::new(
                        "Alamak!",
                        "Diharapkan '{' selepas kata kunci 'kamus'.",
                        cur.line,
                        cur.column,
                    )
                    .with_kind(ErrorKind::Parser)
                    .with_suggestion("Contoh: ada pelajar = kamus { \"nama\": \"Siti\" }"));
                }
                self.parse_kamus_body(pos)
            }
            TokenKind::KurungKurawalBuka => self.parse_kamus_body(pos),

            // Kurung kumpul (expr)
            TokenKind::KurungBuka => {
                let expr = self.parse_expression()?;
                if !self.match_token(TokenKind::KurungTutup) {
                    let cur = self.peek();
                    return Err(MelayuError::new(
                        "Alamak!",
                        "Diharapkan ')' untuk menutup kurungan.",
                        cur.line,
                        cur.column,
                    )
                    .with_kind(ErrorKind::Parser));
                }
                Ok(expr)
            }

            // Input I/O: baca() dan baca nombor()
            TokenKind::Baca => {
                if self.match_token(TokenKind::KurungBuka) {
                    let _ = self.match_token(TokenKind::KurungTutup);
                }
                Ok(Expr::BacaInput(pos))
            }
            TokenKind::BacaNombor => {
                if self.match_token(TokenKind::KurungBuka) {
                    let _ = self.match_token(TokenKind::KurungTutup);
                }
                Ok(Expr::BacaNomborInput(pos))
            }

            _ => Err(MelayuError::new(
                "Eh!",
                &format!("Sintaks tidak dijangka ditemui: '{}'.", tok.lexeme),
                tok.line,
                tok.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion("Periksa semula sintaks kod pada baris ini.")),
        }
    }

    fn parse_kamus_body(&mut self, pos: LineCol) -> Result<Expr, MelayuError> {
        let mut pasangan = Vec::new();
        self.skip_newlines();

        while !self.check(TokenKind::KurungKurawalTutup) && !self.is_at_end() {
            let kunci = self.parse_expression()?;
            if !self.match_token(TokenKind::TitikDua) {
                let cur = self.peek();
                return Err(MelayuError::new(
                    "Alamak!",
                    "Diharapkan ':' antara kunci dan nilai kamus.",
                    cur.line,
                    cur.column,
                )
                .with_kind(ErrorKind::Parser)
                .with_suggestion("Contoh: \"nama\": \"Siti\""));
            }
            let nilai = self.parse_expression()?;
            pasangan.push((kunci, nilai));

            self.skip_newlines();
            if !self.match_token(TokenKind::Koma) {
                self.skip_newlines();
                break;
            }
            self.skip_newlines();
        }

        if !self.match_token(TokenKind::KurungKurawalTutup) {
            let cur = self.peek();
            return Err(MelayuError::new(
                "Alamak!",
                "Diharapkan '}' untuk menutup takrifan kamus.",
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser));
        }

        Ok(Expr::Kamus(pasangan, pos))
    }

    // ----------------------------------------------------
    // Pembantu (Helpers)
    // ----------------------------------------------------

    fn skip_newlines(&mut self) {
        while self.check(TokenKind::BarisBaru) {
            self.advance();
        }
    }

    fn check_stmt_delimiter(&self) -> bool {
        self.check(TokenKind::BarisBaru) || self.check(TokenKind::Tamat)
    }

    fn expect_stmt_delimiter(&mut self) -> Result<(), MelayuError> {
        if self.is_at_end() {
            return Ok(());
        }
        if self.check(TokenKind::BarisBaru) {
            self.advance();
            self.skip_newlines();
            Ok(())
        } else if self.check(TokenKind::Selesai)
            || self.check(TokenKind::JikaTidak)
            || self.check(TokenKind::JikaTidakJika)
            || self.check(TokenKind::Tangkap)
            || self.check(TokenKind::Akhirnya)
        {
            Ok(())
        } else {
            let cur = self.peek();
            Err(MelayuError::new(
                "Woy!",
                &format!(
                    "Diharapkan baris baharu atau penamat pernyataan, tetapi dapat '{}'.",
                    cur.lexeme
                ),
                cur.line,
                cur.column,
            )
            .with_kind(ErrorKind::Parser)
            .with_suggestion("Pastikan setiap arahan diletakkan pada baris berasingan."))
        }
    }

    fn peek(&self) -> &Token {
        if self.cursor >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.cursor]
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            let tok = self.tokens[self.cursor].clone();
            self.cursor += 1;
            tok
        } else {
            self.tokens[self.tokens.len() - 1].clone()
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len() || self.peek().kind == TokenKind::Tamat
    }
}
