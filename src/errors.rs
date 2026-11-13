use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Analyzer,
    Runtime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MelayuError {
    pub kind: ErrorKind,
    pub title: String,              // e.g. "Woy!", "Alamak!", "Aduh!", "Eh!"
    pub message: String,            // Penjelasan masalah
    pub suggestion: Option<String>, // Cadangan cara betulkan
    pub line: usize,
    pub column: usize,
    pub source_line: Option<String>,
}

impl MelayuError {
    pub fn new(title: &str, message: &str, line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: title.to_string(),
            message: message.to_string(),
            suggestion: None,
            line,
            column,
            source_line: None,
        }
    }

    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    pub fn with_source(mut self, source_line: &str) -> Self {
        self.source_line = Some(source_line.to_string());
        self
    }

    // Generator error ramah khas Melayu
    pub fn typo_keyword(word: &str, suggestion: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Lexer,
            title: "Eh!".to_string(),
            message: format!("\"{}\" tu bukan keyword Melayu yang sah.", word),
            suggestion: Some(format!(
                "Mungkin maksud kau \"{}\"? Cuba betulkan dulu.",
                suggestion
            )),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn pembolehubah_tak_wujud(nama: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Woy!".to_string(),
            message: format!("Pembolehubah '{}' belum diisytihar (tidak wujud).", nama),
            suggestion: Some(format!(
                "Cuba tambah 'ada {} = ...' dulu sebelum guna.",
                nama
            )),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn pembolehubah_dah_ada(nama: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Alamak!".to_string(),
            message: format!(
                "Pembolehubah '{}' dah wujud, tak boleh dideklarasikan dua kali.",
                nama
            ),
            suggestion: Some(format!(
                "Guna nama lain, atau buang 'ada' kalau kau cuma nak tukar nilai '{} = ...'.",
                nama
            )),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn ubah_pembolehubah_tetap(nama: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Alamak!".to_string(),
            message: format!(
                "'{}' adalah pembolehubah tetap (konstanta), nilainya tak boleh diubah!",
                nama
            ),
            suggestion: Some(format!(
                "Kalau nilainya perlu berubah, isytihar guna 'ada {} = ...' dan bukan 'tetap'.",
                nama
            )),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn bahagi_kosong(line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Aduh!".to_string(),
            message: "Pembahagian dengan kosong tak boleh lah bro.".to_string(),
            suggestion: Some("Cuba semak pembahagi kau, pastikan nilainya bukan 0.".to_string()),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn blok_tak_tutup(blok: &str, buka_line: usize, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Parser,
            title: "Woy!".to_string(),
            message: format!(
                "Blok '{}' bermula di baris {} belum ditutup.",
                blok, buka_line
            ),
            suggestion: Some("Tambah 'selesai' di hujung blok untuk menutupnya.".to_string()),
            line,
            column: col,
            source_line: None,
        }
    }

    pub fn jenis_salah(jangka: &str, dapat: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Eh!".to_string(),
            message: format!(
                "Operasi perlukan jenis '{}', tapi dapat '{}'.",
                jangka, dapat
            ),
            suggestion: Some(
                "Cuba semak dan pastikan nilai yang kau masukkan berpadanan.".to_string(),
            ),
            line,
            column: col,
            source_line: None,
        }
    }
}

impl fmt::Display for MelayuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} Ada ralat di baris {}, lajur {}:",
            self.title, self.line, self.column
        )?;
        if let Some(src) = &self.source_line {
            writeln!(f, "  {}", src)?;
            let pointer = if self.column > 1 {
                format!("{:>width$}^", "", width = self.column - 1)
            } else {
                "^".to_string()
            };
            writeln!(f, "  {}", pointer)?;
        }
        writeln!(f, "  {}", self.message)?;
        if let Some(sug) = &self.suggestion {
            writeln!(f, "  -> {}", sug)?;
        }
        Ok(())
    }
}

impl std::error::Error for MelayuError {}
