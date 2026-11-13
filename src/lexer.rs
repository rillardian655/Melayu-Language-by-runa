use crate::errors::{ErrorKind, MelayuError};
use crate::tokens::{Token, TokenKind};

pub struct Lexer {
    #[allow(dead_code)]
    source: String,
    chars: Vec<char>,
    cursor: usize,
    line: usize,
    col: usize,
    paren_depth: usize,
}

impl Lexer {
    pub fn new(source: impl Into<String>) -> Self {
        let src = source.into();
        let chars = src.chars().collect();
        Self {
            source: src,
            chars,
            cursor: 0,
            line: 1,
            col: 1,
            paren_depth: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, MelayuError> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let start_line = self.line;
            let start_col = self.col;
            let c = self.peek();

            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    // Jika di dalam kurung / kurawal / siku, newline tidak memecah baris
                    if self.paren_depth == 0 {
                        // Jangan buat BarisBaru berturut-turut atau di awal
                        if let Some(last) = tokens.last() {
                            if last.kind != TokenKind::BarisBaru {
                                tokens.push(Token::new(
                                    TokenKind::BarisBaru,
                                    "\\n",
                                    start_line,
                                    start_col,
                                ));
                            }
                        }
                    }
                }
                ';' => {
                    // Titik koma diperlakukan sebagai pemisah baris untuk keramahan pemula
                    self.advance();
                    if let Some(last) = tokens.last() {
                        if last.kind != TokenKind::BarisBaru {
                            tokens.push(Token::new(
                                TokenKind::BarisBaru,
                                ";",
                                start_line,
                                start_col,
                            ));
                        }
                    }
                }
                '#' => {
                    // Komentar: semak sama ada multi-baris #[ ... ]#
                    if self.peek_offset(1) == Some('[') {
                        // Multi-baris
                        self.advance(); // lewati '#'
                        self.advance(); // lewati '['
                        let mut closed = false;
                        while !self.is_at_end() {
                            if self.peek() == ']' && self.peek_offset(1) == Some('#') {
                                self.advance(); // ']'
                                self.advance(); // '#'
                                closed = true;
                                break;
                            }
                            self.advance();
                        }
                        if !closed {
                            return Err(MelayuError::new(
                                "Woy!",
                                "Komentar multi-baris '#[' tidak ditutup dengan ']#' sehingga akhir fail.",
                                start_line,
                                start_col,
                            )
                            .with_kind(ErrorKind::Lexer)
                            .with_suggestion("Tambah ']#' untuk menutup komentar multi-baris ini."));
                        }
                    } else {
                        // Komentar satu baris sehingga akhir baris
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    }
                }
                '(' => {
                    self.advance();
                    self.paren_depth += 1;
                    tokens.push(Token::new(
                        TokenKind::KurungBuka,
                        "(",
                        start_line,
                        start_col,
                    ));
                }
                ')' => {
                    self.advance();
                    if self.paren_depth > 0 {
                        self.paren_depth -= 1;
                    }
                    tokens.push(Token::new(
                        TokenKind::KurungTutup,
                        ")",
                        start_line,
                        start_col,
                    ));
                }
                '[' => {
                    self.advance();
                    self.paren_depth += 1;
                    tokens.push(Token::new(
                        TokenKind::KurungSikuBuka,
                        "[",
                        start_line,
                        start_col,
                    ));
                }
                ']' => {
                    self.advance();
                    if self.paren_depth > 0 {
                        self.paren_depth -= 1;
                    }
                    tokens.push(Token::new(
                        TokenKind::KurungSikuTutup,
                        "]",
                        start_line,
                        start_col,
                    ));
                }
                '{' => {
                    self.advance();
                    self.paren_depth += 1;
                    tokens.push(Token::new(
                        TokenKind::KurungKurawalBuka,
                        "{",
                        start_line,
                        start_col,
                    ));
                }
                '}' => {
                    self.advance();
                    if self.paren_depth > 0 {
                        self.paren_depth -= 1;
                    }
                    tokens.push(Token::new(
                        TokenKind::KurungKurawalTutup,
                        "}",
                        start_line,
                        start_col,
                    ));
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Koma, ",", start_line, start_col));
                }
                '.' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Titik, ".", start_line, start_col));
                }
                ':' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::TitikDua, ":", start_line, start_col));
                }
                '+' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Tambah, "+", start_line, start_col));
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Tolak, "-", start_line, start_col));
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Kali, "*", start_line, start_col));
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Bahagi, "/", start_line, start_col));
                }
                '%' => {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Modulo, "%", start_line, start_col));
                }
                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenKind::SamaDengan,
                            "==",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(
                            TokenKind::SamaDenganSatu,
                            "=",
                            start_line,
                            start_col,
                        ));
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenKind::TidakSama,
                            "!=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(TokenKind::Bukan, "!", start_line, start_col));
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenKind::KurangSama,
                            "<=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(TokenKind::Kurang, "<", start_line, start_col));
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenKind::LebihSama,
                            ">=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(TokenKind::Lebih, ">", start_line, start_col));
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == '&' {
                        self.advance();
                        tokens.push(Token::new(TokenKind::Dan, "&&", start_line, start_col));
                    } else {
                        return Err(MelayuError::new(
                            "Eh!",
                            "Simbol '&' tunggal tidak dikenali. Gunakan '&&' atau kata 'dan'.",
                            start_line,
                            start_col,
                        )
                        .with_kind(ErrorKind::Lexer)
                        .with_suggestion("Gunakan perkataan 'dan' atau simbol '&&'."));
                    }
                }
                '|' => {
                    self.advance();
                    if self.peek() == '|' {
                        self.advance();
                        tokens.push(Token::new(TokenKind::Atau, "||", start_line, start_col));
                    } else {
                        return Err(MelayuError::new(
                            "Eh!",
                            "Simbol '|' tunggal tidak dikenali. Gunakan '||' atau kata 'atau'.",
                            start_line,
                            start_col,
                        )
                        .with_kind(ErrorKind::Lexer)
                        .with_suggestion("Gunakan perkataan 'atau' atau simbol '||'."));
                    }
                }
                '"' => {
                    let s = self.tokenize_string(start_line, start_col)?;
                    tokens.push(Token::new(
                        TokenKind::Teks(s.clone()),
                        format!("\"{}\"", s),
                        start_line,
                        start_col,
                    ));
                }
                c if c.is_ascii_digit() => {
                    let (num, lex) = self.tokenize_number();
                    tokens.push(Token::new(
                        TokenKind::Nombor(num),
                        lex,
                        start_line,
                        start_col,
                    ));
                }
                c if is_ident_start(c) => {
                    let token = self.tokenize_identifier_or_keyword(start_line, start_col)?;
                    tokens.push(token);
                }
                _ => {
                    let ch = self.peek();
                    self.advance();
                    return Err(MelayuError::new(
                        "Alamak!",
                        &format!("Aksara tidak sah ditemui: '{}'", ch),
                        start_line,
                        start_col,
                    )
                    .with_kind(ErrorKind::Lexer)
                    .with_suggestion(
                        "Sila padamkan aksara ini atau ganti dengan sintaks Melayu yang sah.",
                    ));
                }
            }
        }

        // Pastikan ada baris baru jika baris terakhir ada arahan
        if let Some(last) = tokens.last() {
            if last.kind != TokenKind::BarisBaru {
                tokens.push(Token::new(TokenKind::BarisBaru, "\\n", self.line, self.col));
            }
        }
        tokens.push(Token::new(TokenKind::Tamat, "", self.line, self.col));

        // Tambah teks baris kod ke setiap error jika berlaku nanti
        Ok(tokens)
    }

    fn tokenize_string(
        &mut self,
        start_line: usize,
        start_col: usize,
    ) -> Result<String, MelayuError> {
        self.advance(); // Lewati tanda petik awal '"'
        let mut s = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                // Rentetan boleh merentas baris atau baris baharu
                s.push('\n');
                self.advance();
            } else if self.peek() == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
                match self.peek() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    other => {
                        s.push('\\');
                        s.push(other);
                    }
                }
                self.advance();
            } else {
                s.push(self.peek());
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(MelayuError::new(
                "Woy!",
                "Rentetan teks tidak ditutup sehingga akhir baris/fail.",
                start_line,
                start_col,
            )
            .with_kind(ErrorKind::Lexer)
            .with_suggestion("Pastikan kau menutup rentetan dengan tanda petik dua (\")."));
        }

        self.advance(); // Lewati tanda petik penutup '"'
        Ok(s)
    }

    fn tokenize_number(&mut self) -> (f64, String) {
        let mut s = String::new();
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            s.push(self.peek());
            self.advance();
        }

        // Titik perpuluhan
        if !self.is_at_end()
            && self.peek() == '.'
            && self.peek_offset(1).is_some_and(|c| c.is_ascii_digit())
        {
            s.push(self.peek());
            self.advance();
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                s.push(self.peek());
                self.advance();
            }
        }

        let num: f64 = s.parse().unwrap_or(0.0);
        (num, s)
    }

    fn tokenize_identifier_or_keyword(
        &mut self,
        start_line: usize,
        start_col: usize,
    ) -> Result<Token, MelayuError> {
        let mut ident = String::new();
        while !self.is_at_end() && is_ident_part(self.peek()) {
            ident.push(self.peek());
            self.advance();
        }

        // Semak kata kunci gabungan seperti "jika tidak jika", "jika tidak", "sama dengan", "tidak sama", dll.
        // Semak "jika tidak jika" atau "jika tidak"
        if ident == "jika" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "tidak" {
                let after_tidak_cursor = self.cursor;
                let after_tidak_line = self.line;
                let after_tidak_col = self.col;

                self.skip_horizontal_whitespace();
                let mut third_word = String::new();
                while !self.is_at_end() && is_ident_part(self.peek()) {
                    third_word.push(self.peek());
                    self.advance();
                }

                if third_word == "jika" {
                    return Ok(Token::new(
                        TokenKind::JikaTidakJika,
                        "jika tidak jika",
                        start_line,
                        start_col,
                    ));
                } else {
                    // Pulihkan kursor ke selepas "tidak"
                    self.cursor = after_tidak_cursor;
                    self.line = after_tidak_line;
                    self.col = after_tidak_col;
                    return Ok(Token::new(
                        TokenKind::JikaTidak,
                        "jika tidak",
                        start_line,
                        start_col,
                    ));
                }
            } else {
                // Pulihkan kursor ke selepas "jika"
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
                return Ok(Token::new(TokenKind::Jika, "jika", start_line, start_col));
            }
        }

        // Semak "sama dengan"
        if ident == "sama" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "dengan" {
                return Ok(Token::new(
                    TokenKind::SamaDengan,
                    "sama dengan",
                    start_line,
                    start_col,
                ));
            } else {
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
            }
        }

        // Semak "tidak sama"
        if ident == "tidak" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "sama" {
                return Ok(Token::new(
                    TokenKind::TidakSama,
                    "tidak sama",
                    start_line,
                    start_col,
                ));
            } else {
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
                // "tidak" sendiri boleh dianggap sebagai operator NOT atau kata kunci
                return Ok(Token::new(TokenKind::Bukan, "tidak", start_line, start_col));
            }
        }

        // Semak "lebih sama"
        if ident == "lebih" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "sama" {
                return Ok(Token::new(
                    TokenKind::LebihSama,
                    "lebih sama",
                    start_line,
                    start_col,
                ));
            } else {
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
                return Ok(Token::new(TokenKind::Lebih, "lebih", start_line, start_col));
            }
        }

        // Semak "kurang sama"
        if ident == "kurang" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "sama" {
                return Ok(Token::new(
                    TokenKind::KurangSama,
                    "kurang sama",
                    start_line,
                    start_col,
                ));
            } else {
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
                return Ok(Token::new(
                    TokenKind::Kurang,
                    "kurang",
                    start_line,
                    start_col,
                ));
            }
        }

        // Semak "baca nombor"
        if ident == "baca" {
            let saved_cursor = self.cursor;
            let saved_line = self.line;
            let saved_col = self.col;

            self.skip_horizontal_whitespace();
            let mut next_word = String::new();
            while !self.is_at_end() && is_ident_part(self.peek()) {
                next_word.push(self.peek());
                self.advance();
            }

            if next_word == "nombor" {
                return Ok(Token::new(
                    TokenKind::BacaNombor,
                    "baca nombor",
                    start_line,
                    start_col,
                ));
            } else {
                self.cursor = saved_cursor;
                self.line = saved_line;
                self.col = saved_col;
                return Ok(Token::new(TokenKind::Baca, "baca", start_line, start_col));
            }
        }

        // Padanan kata kunci tunggal
        let kind = match ident.as_str() {
            "ada" => TokenKind::Ada,
            "tetap" => TokenKind::Tetap,
            "selagi" => TokenKind::Selagi,
            "untuk" => TokenKind::Untuk,
            "dalam" => TokenKind::Dalam,
            "dari" => TokenKind::Dari,
            "hingga" => TokenKind::Hingga,
            "langkah" => TokenKind::Langkah,
            "hentikan" => TokenKind::Hentikan,
            "teruskan" => TokenKind::Teruskan,
            "kerja" | "fungsi" => TokenKind::Kerja,
            "pulang" => TokenKind::Pulang,
            "selesai" => TokenKind::Selesai,
            "tunjuk" | "cetak" => TokenKind::Tunjuk,
            "kamus" => TokenKind::Kamus,
            "senarai" => TokenKind::Senarai,
            "cuba" => TokenKind::Cuba,
            "tangkap" => TokenKind::Tangkap,
            "baling" => TokenKind::Baling,
            "akhirnya" => TokenKind::Akhirnya,
            "ambil" => TokenKind::Ambil,
            "daripada" => TokenKind::Daripada,
            "sebagai" => TokenKind::Sebagai,
            "horas" => TokenKind::Horas,
            "mangan" => TokenKind::Mangan,

            // Jenis & Nilai Literal
            "teks" => TokenKind::TipeTeks,
            "nombor" => TokenKind::TipeNombor,
            "betul" => TokenKind::Betul,
            "salah" => TokenKind::Salah,
            "kosong" => TokenKind::Kosong,

            // Operator perkataan
            "tambah" => TokenKind::Tambah,
            "tolak" => TokenKind::Tolak,
            "kali" => TokenKind::Kali,
            "bahagi" => TokenKind::Bahagi,
            "modulo" => TokenKind::Modulo,
            "dan" => TokenKind::Dan,
            "atau" => TokenKind::Atau,
            "bukan" => TokenKind::Bukan,

            // Pengesan Typo / Kata Kunci Asing Biasa
            "tunjukkan" => {
                return Err(MelayuError::typo_keyword(
                    "tunjukkan",
                    "tunjuk",
                    start_line,
                    start_col,
                ));
            }
            "print" | "println" => {
                return Err(MelayuError::typo_keyword(
                    &ident, "tunjuk", start_line, start_col,
                ));
            }
            "if" => {
                return Err(MelayuError::typo_keyword(
                    "if", "jika", start_line, start_col,
                ));
            }
            "else" => {
                return Err(MelayuError::typo_keyword(
                    "else",
                    "jika tidak",
                    start_line,
                    start_col,
                ));
            }
            "while" => {
                return Err(MelayuError::typo_keyword(
                    "while", "selagi", start_line, start_col,
                ));
            }
            "for" => {
                return Err(MelayuError::typo_keyword(
                    "for", "untuk", start_line, start_col,
                ));
            }
            "def" | "fn" | "function" => {
                return Err(MelayuError::typo_keyword(
                    &ident, "kerja", start_line, start_col,
                ));
            }
            "return" => {
                return Err(MelayuError::typo_keyword(
                    "return", "pulang", start_line, start_col,
                ));
            }
            "let" | "var" => {
                return Err(MelayuError::typo_keyword(
                    &ident, "ada", start_line, start_col,
                ));
            }
            "const" => {
                return Err(MelayuError::typo_keyword(
                    "const", "tetap", start_line, start_col,
                ));
            }
            "true" => {
                return Err(MelayuError::typo_keyword(
                    "true", "betul", start_line, start_col,
                ));
            }
            "false" => {
                return Err(MelayuError::typo_keyword(
                    "false", "salah", start_line, start_col,
                ));
            }
            "null" | "nil" | "None" => {
                return Err(MelayuError::typo_keyword(
                    &ident, "kosong", start_line, start_col,
                ));
            }
            "break" => {
                return Err(MelayuError::typo_keyword(
                    "break", "hentikan", start_line, start_col,
                ));
            }
            "continue" => {
                return Err(MelayuError::typo_keyword(
                    "continue", "teruskan", start_line, start_col,
                ));
            }
            "try" => {
                return Err(MelayuError::typo_keyword(
                    "try", "cuba", start_line, start_col,
                ));
            }
            "catch" => {
                return Err(MelayuError::typo_keyword(
                    "catch", "tangkap", start_line, start_col,
                ));
            }

            // Bukan kata kunci: merupakan Pengenal (Identifier) biasa
            _ => TokenKind::Pengenal(ident.clone()),
        };

        Ok(Token::new(kind, ident, start_line, start_col))
    }

    fn skip_horizontal_whitespace(&mut self) {
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.cursor]
        }
    }

    fn peek_offset(&self, offset: usize) -> Option<char> {
        let idx = self.cursor + offset;
        if idx < self.chars.len() {
            Some(self.chars[idx])
        } else {
            None
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c = self.chars[self.cursor];
        self.cursor += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        c
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.chars.len()
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
