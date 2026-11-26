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
    pub source_line: Option<Box<str>>,
    pub modul: Option<Box<String>>,
    pub asal_ditanda: bool,
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
            modul: None,
            asal_ditanda: false,
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
        self.source_line = Some(source_line.into());
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
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
            modul: None,
            asal_ditanda: false,
        }
    }

    pub fn tandakan_asal(mut self, modul: Option<&str>, kandungan: &str) -> Self {
        if self.asal_ditanda {
            return self;
        }
        self.asal_ditanda = true;
        if let Some(nama) = modul {
            self.modul = Some(Box::new(nama.to_string()));
            if self.source_line.is_none() {
                self.source_line = kandungan
                    .lines()
                    .nth(self.line.saturating_sub(1))
                    .map(|b| b.into());
            }
        }
        self
    }

    pub fn tindanan_penuh(line: usize, col: usize) -> Self {
        Self::new(
            "Alamak!",
            "Program terlalu dalam: ungkapan bersarang atau rekursi melebihi ruang tindanan yang selamat.",
            line,
            col,
        )
        .with_suggestion("Kurangkan kedalaman rekursi atau pecahkan ungkapan yang panjang.")
    }

    pub fn rekursi_berlebihan(had: usize, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Alamak!".to_string(),
            message: format!(
                "Had rekursi maksimum ({}) telah dicapai! Fungsi memanggil dirinya terlalu dalam.",
                had
            ),
            suggestion: Some(
                "Pastikan fungsi rekursif anda mempunyai kes asas (base case) untuk berhenti."
                    .to_string(),
            ),
            line,
            column: col,
            source_line: None,
            modul: None,
            asal_ditanda: false,
        }
    }

    pub fn had_langkah_terlampau(had: u64, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Woy!".to_string(),
            message: format!(
                "Had pelaksanaan selamat ({}) langkah telah terlampau! Program dihentikan untuk mengelakkan pembekuan (infinite loop).",
                had
            ),
            suggestion: Some(
                "Semak gelung 'selagi' atau 'untuk', pastikan syarat penamatan dapat dicapai."
                    .to_string(),
            ),
            line,
            column: col,
            source_line: None,
            modul: None,
            asal_ditanda: false,
        }
    }

    pub fn indeks_tidak_sah(mesej: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Aduh!".to_string(),
            message: mesej.to_string(),
            suggestion: Some(
                "Pastikan indeks adalah nombor bulat yang sah dalam julat yang betul.".to_string(),
            ),
            line,
            column: col,
            source_line: None,
            modul: None,
            asal_ditanda: false,
        }
    }

    pub fn had_memori_terlampau(mesej: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Alamak!".to_string(),
            message: format!("Had penggunaan memori selamat terlampau: {}.", mesej),
            suggestion: Some(
                "Elakkan penyambungan rentetan atau penambahan senarai yang terlalu besar dalam satu operasi."
                    .to_string(),
            ),
            line,
            column: col,
            source_line: None,
            modul: None,
            asal_ditanda: false,
        }
    }

    pub fn akses_fail_disekat(laluan: &str, sebab: &str, line: usize, col: usize) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            title: "Woy!".to_string(),
            message: format!("Akses fail keselamatan disekat untuk '{}': {}", laluan, sebab),
            suggestion: Some(
                "Operasi fail dihadkan kepada folder projek (folder yang mengandungi pakej.json, atau folder fail .ml) dan saiz maksimum 5 MB. Guna --sandbox <folder> untuk menukar sempadan ini."
                    .to_string(),
            ),
            line,
            column: col,
            source_line: None,
            modul: None,
            asal_ditanda: false,
        }
    }
}

impl fmt::Display for MelayuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.modul {
            Some(nama) => writeln!(
                f,
                "{} Ada ralat dalam modul '{}' di baris {}, lajur {}:",
                self.title, nama, self.line, self.column
            )?,
            None => writeln!(
                f,
                "{} Ada ralat di baris {}, lajur {}:",
                self.title, self.line, self.column
            )?,
        }
        if let Some(src) = &self.source_line {
            let aksara: Vec<char> = src.chars().collect();
            let lajur = self.column.max(1) - 1;
            let mula = lajur.saturating_sub(60);
            let tamat = (lajur + 60).min(aksara.len());
            let awalan = if mula > 0 { "..." } else { "" };
            let akhiran = if tamat < aksara.len() { "..." } else { "" };
            let petikan: String = aksara[mula.min(tamat)..tamat].iter().collect();
            writeln!(f, "  {}{}{}", awalan, petikan, akhiran)?;
            writeln!(f, "  {}^", " ".repeat(awalan.len() + lajur - mula))?;
        }
        writeln!(f, "  {}", self.message)?;
        if let Some(sug) = &self.suggestion {
            writeln!(f, "  -> {}", sug)?;
        }
        Ok(())
    }
}

impl std::error::Error for MelayuError {}
