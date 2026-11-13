use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literal
    Nombor(f64),
    Teks(String),
    Betul,
    Salah,
    Kosong,

    // Pengenal (Identifier)
    Pengenal(String),

    // Kata Kunci (Keywords)
    Ada,
    Tetap,
    Jika,
    JikaTidakJika,
    JikaTidak,
    Selagi,
    Untuk,
    Dalam,
    Dari,
    Hingga,
    Langkah,
    Hentikan,
    Teruskan,
    Kerja,
    Pulang,
    Selesai,
    Tunjuk,
    Baca,
    BacaNombor,
    Kamus,
    Senarai,
    Cuba,
    Tangkap,
    Baling,
    Akhirnya,
    Ambil,
    Daripada,
    Sebagai,
    Horas,
    Mangan,

    // Jenis (Types)
    TipeTeks,
    TipeNombor,
    TipeBetul,
    TipeSenarai,
    TipeKamus,
    TipeKosong,

    // Operator Aritmetik
    Tambah, // tambah / +
    Tolak,  // tolak / -
    Kali,   // kali / *
    Bahagi, // bahagi / /
    Modulo, // modulo / %

    // Operator Perbandingan
    SamaDengan, // sama dengan / ==
    TidakSama,  // tidak sama / !=
    LebihSama,  // lebih sama / >=
    KurangSama, // kurang sama / <=
    Lebih,      // lebih / >
    Kurang,     // kurang / <

    // Operator Logik
    Dan,   // dan / &&
    Atau,  // atau / ||
    Bukan, // bukan / !

    // Tanda Baca & Delimiter
    SamaDenganSatu,     // =
    KurungBuka,         // (
    KurungTutup,        // )
    KurungSikuBuka,     // [
    KurungSikuTutup,    // ]
    KurungKurawalBuka,  // {
    KurungKurawalTutup, // }
    Koma,               // ,
    Titik,              // .
    TitikDua,           // :
    BarisBaru,          // \n / statement boundary
    Tamat,              // EOF
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Nombor(n) => write!(f, "nombor({})", n),
            TokenKind::Teks(s) => write!(f, "teks(\"{}\")", s),
            TokenKind::Betul => write!(f, "betul"),
            TokenKind::Salah => write!(f, "salah"),
            TokenKind::Kosong => write!(f, "kosong"),
            TokenKind::Pengenal(s) => write!(f, "pengenal({})", s),
            TokenKind::Ada => write!(f, "ada"),
            TokenKind::Tetap => write!(f, "tetap"),
            TokenKind::Jika => write!(f, "jika"),
            TokenKind::JikaTidakJika => write!(f, "jika tidak jika"),
            TokenKind::JikaTidak => write!(f, "jika tidak"),
            TokenKind::Selagi => write!(f, "selagi"),
            TokenKind::Untuk => write!(f, "untuk"),
            TokenKind::Dalam => write!(f, "dalam"),
            TokenKind::Dari => write!(f, "dari"),
            TokenKind::Hingga => write!(f, "hingga"),
            TokenKind::Langkah => write!(f, "langkah"),
            TokenKind::Hentikan => write!(f, "hentikan"),
            TokenKind::Teruskan => write!(f, "teruskan"),
            TokenKind::Kerja => write!(f, "kerja"),
            TokenKind::Pulang => write!(f, "pulang"),
            TokenKind::Selesai => write!(f, "selesai"),
            TokenKind::Tunjuk => write!(f, "tunjuk"),
            TokenKind::Baca => write!(f, "baca"),
            TokenKind::BacaNombor => write!(f, "baca nombor"),
            TokenKind::Kamus => write!(f, "kamus"),
            TokenKind::Senarai => write!(f, "senarai"),
            TokenKind::Cuba => write!(f, "cuba"),
            TokenKind::Tangkap => write!(f, "tangkap"),
            TokenKind::Baling => write!(f, "baling"),
            TokenKind::Akhirnya => write!(f, "akhirnya"),
            TokenKind::Ambil => write!(f, "ambil"),
            TokenKind::Daripada => write!(f, "daripada"),
            TokenKind::Sebagai => write!(f, "sebagai"),
            TokenKind::Horas => write!(f, "horas"),
            TokenKind::Mangan => write!(f, "mangan"),
            TokenKind::TipeTeks => write!(f, "tipe:teks"),
            TokenKind::TipeNombor => write!(f, "tipe:nombor"),
            TokenKind::TipeBetul => write!(f, "tipe:betul"),
            TokenKind::TipeSenarai => write!(f, "tipe:senarai"),
            TokenKind::TipeKamus => write!(f, "tipe:kamus"),
            TokenKind::TipeKosong => write!(f, "tipe:kosong"),
            TokenKind::Tambah => write!(f, "tambah (+)"),
            TokenKind::Tolak => write!(f, "tolak (-)"),
            TokenKind::Kali => write!(f, "kali (*)"),
            TokenKind::Bahagi => write!(f, "bahagi (/)"),
            TokenKind::Modulo => write!(f, "modulo (%)"),
            TokenKind::SamaDengan => write!(f, "sama dengan (==)"),
            TokenKind::TidakSama => write!(f, "tidak sama (!=)"),
            TokenKind::LebihSama => write!(f, "lebih sama (>=)"),
            TokenKind::KurangSama => write!(f, "kurang sama (<=)"),
            TokenKind::Lebih => write!(f, "lebih (>)"),
            TokenKind::Kurang => write!(f, "kurang (<)"),
            TokenKind::Dan => write!(f, "dan (&&)"),
            TokenKind::Atau => write!(f, "atau (||)"),
            TokenKind::Bukan => write!(f, "bukan (!)"),
            TokenKind::SamaDenganSatu => write!(f, "="),
            TokenKind::KurungBuka => write!(f, "("),
            TokenKind::KurungTutup => write!(f, ")"),
            TokenKind::KurungSikuBuka => write!(f, "["),
            TokenKind::KurungSikuTutup => write!(f, "]"),
            TokenKind::KurungKurawalBuka => write!(f, "{{"),
            TokenKind::KurungKurawalTutup => write!(f, "}}"),
            TokenKind::Koma => write!(f, ","),
            TokenKind::Titik => write!(f, "."),
            TokenKind::TitikDua => write!(f, ":"),
            TokenKind::BarisBaru => write!(f, "\\n"),
            TokenKind::Tamat => write!(f, "TAMAT (EOF)"),
        }
    }
}
