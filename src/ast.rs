use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
}

impl LineCol {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Nombor(f64),
    Teks(String),
    Betul,
    Salah,
    Kosong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Tambah,
    Tolak,
    Kali,
    Bahagi,
    Modulo,
    MatriksDarab,
    SamaDengan,
    TidakSama,
    Lebih,
    Kurang,
    LebihSama,
    KurangSama,
    Dan,
    Atau,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Tambah => write!(f, "+"),
            BinaryOp::Tolak => write!(f, "-"),
            BinaryOp::Kali => write!(f, "*"),
            BinaryOp::Bahagi => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::MatriksDarab => write!(f, "@"),
            BinaryOp::SamaDengan => write!(f, "=="),
            BinaryOp::TidakSama => write!(f, "!="),
            BinaryOp::Lebih => write!(f, ">"),
            BinaryOp::Kurang => write!(f, "<"),
            BinaryOp::LebihSama => write!(f, ">="),
            BinaryOp::KurangSama => write!(f, "<="),
            BinaryOp::Dan => write!(f, "&&"),
            BinaryOp::Atau => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negatif,
    Bukan,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negatif => write!(f, "-"),
            UnaryOp::Bukan => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(LiteralValue, LineCol),
    Pengenal(String, LineCol),
    Senarai(Vec<Expr>, LineCol),
    Kamus(Vec<(Expr, Expr)>, LineCol),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        pos: LineCol,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        pos: LineCol,
    },
    Panggil {
        fungsi: Box<Expr>,
        argumen: Vec<Expr>,
        pos: LineCol,
    },
    AksesIndeks {
        sasaran: Box<Expr>,
        indeks: Box<Expr>,
        pos: LineCol,
    },
    AksesSifat {
        sasaran: Box<Expr>,
        sifat: String,
        pos: LineCol,
    },
    PanggilKaedah {
        sasaran: Box<Expr>,
        kaedah: String,
        argumen: Vec<Expr>,
        pos: LineCol,
    },
    BacaInput(LineCol),
    BacaNomborInput(LineCol),

    // Ungkapan Baru v0.4.0
    SaluranPaip {
        kiri: Box<Expr>,
        kanan: Box<Expr>,
        pos: LineCol,
    },
    NullCoalesce {
        kiri: Box<Expr>,
        kanan: Box<Expr>,
        pos: LineCol,
    },
    AksesSelamatSifat {
        sasaran: Box<Expr>,
        sifat: String,
        pos: LineCol,
    },
    PanggilSelamatKaedah {
        sasaran: Box<Expr>,
        kaedah: String,
        argumen: Vec<Expr>,
        pos: LineCol,
    },
    Padan(Box<PadanAST>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PadanAST {
    pub sasaran: Box<Expr>,
    pub kes: Vec<KesPadan>,
    pub lain: Option<Box<Expr>>,
    pub pos: LineCol,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PadanPola {
    Nilai(Box<Expr>),
    Julat(Box<Expr>, Box<Expr>), // mula hingga tamat
    Lain,                        // 'lain'
}

#[derive(Debug, Clone, PartialEq)]
pub struct KesPadan {
    pub pola: PadanPola,
    pub syarat: Option<Box<Expr>>,
    pub hasil: Box<Expr>,
    pub pos: LineCol,
}

impl Expr {
    pub fn pos(&self) -> LineCol {
        match self {
            Expr::Literal(_, pos) => *pos,
            Expr::Pengenal(_, pos) => *pos,
            Expr::Senarai(_, pos) => *pos,
            Expr::Kamus(_, pos) => *pos,
            Expr::Unary { pos, .. } => *pos,
            Expr::Binary { pos, .. } => *pos,
            Expr::Panggil { pos, .. } => *pos,
            Expr::AksesIndeks { pos, .. } => *pos,
            Expr::AksesSifat { pos, .. } => *pos,
            Expr::PanggilKaedah { pos, .. } => *pos,
            Expr::BacaInput(pos) => *pos,
            Expr::BacaNomborInput(pos) => *pos,
            Expr::SaluranPaip { pos, .. } => *pos,
            Expr::NullCoalesce { pos, .. } => *pos,
            Expr::AksesSelamatSifat { pos, .. } => *pos,
            Expr::PanggilSelamatKaedah { pos, .. } => *pos,
            Expr::Padan(p) => p.pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetAssign {
    Pembolehubah(String),
    Indeks {
        sasaran: Box<Expr>,
        indeks: Box<Expr>,
    },
    Sifat {
        sasaran: Box<Expr>,
        sifat: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct KaedahKelasAST {
    pub nama: String,
    pub parameter: Vec<(String, Option<String>)>,
    pub jenis_pulangan: Option<String>,
    pub badan: Vec<Stmt>,
    pub pos: LineCol,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    IsytiharPembolehubah {
        nama: String,
        tetap: bool,
        jenis_anotasi: Option<String>,
        nilai: Expr,
        pos: LineCol,
    },
    Umpukan {
        sasaran: TargetAssign,
        nilai: Expr,
        pos: LineCol,
    },
    Tunjuk(Vec<Expr>, LineCol),
    Jika {
        syarat: Expr,
        maka: Vec<Stmt>,
        jika_tidak_jika: Vec<(Expr, Vec<Stmt>)>,
        jika_tidak: Option<Vec<Stmt>>,
        pos: LineCol,
    },
    Selagi {
        syarat: Expr,
        badan: Vec<Stmt>,
        pos: LineCol,
    },
    UntukDalam {
        pengenal: String,
        senarai: Expr,
        badan: Vec<Stmt>,
        pos: LineCol,
    },
    UntukJulat {
        pengenal: String,
        mula: Expr,
        hingga: Expr,
        langkah: Option<Expr>,
        badan: Vec<Stmt>,
        pos: LineCol,
    },
    DeklarasiFungsi {
        nama: String,
        parameter: Vec<(String, Option<String>)>,
        jenis_pulangan: Option<String>,
        badan: Vec<Stmt>,
        pos: LineCol,
    },
    Pulang(Option<Expr>, LineCol),
    Hentikan(LineCol),
    Teruskan(LineCol),
    Cuba {
        badan_cuba: Vec<Stmt>,
        ralat_var: String,
        badan_tangkap: Vec<Stmt>,
        badan_akhirnya: Option<Vec<Stmt>>,
        pos: LineCol,
    },
    Baling(Expr, LineCol),
    Ungkapan(Expr, LineCol),
    Horas(LineCol),
    Mangan(LineCol),
    Ambil {
        sasaran: String,
        simbol: Option<Vec<String>>,
        alias: Option<String>,
        pos: LineCol,
    },
    DeklarasiKelas(Box<DeklarasiKelasAST>),

    // Pernyataan Baru v0.4.0
    Uji {
        nama: String,
        badan: Vec<Stmt>,
        pos: LineCol,
    },
    Semak {
        syarat: Expr,
        mesej: Option<Expr>,
        pos: LineCol,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeklarasiKelasAST {
    pub nama: String,
    pub induk: Option<String>,
    pub pembina: Option<KaedahKelasAST>,
    pub kaedah: Vec<KaedahKelasAST>,
    pub pos: LineCol,
}

impl Stmt {
    pub fn pos(&self) -> LineCol {
        match self {
            Stmt::IsytiharPembolehubah { pos, .. } => *pos,
            Stmt::Umpukan { pos, .. } => *pos,
            Stmt::Tunjuk(_, pos) => *pos,
            Stmt::Jika { pos, .. } => *pos,
            Stmt::Selagi { pos, .. } => *pos,
            Stmt::UntukDalam { pos, .. } => *pos,
            Stmt::UntukJulat { pos, .. } => *pos,
            Stmt::DeklarasiFungsi { pos, .. } => *pos,
            Stmt::DeklarasiKelas(d) => d.pos,
            Stmt::Pulang(_, pos) => *pos,
            Stmt::Hentikan(pos) => *pos,
            Stmt::Teruskan(pos) => *pos,
            Stmt::Cuba { pos, .. } => *pos,
            Stmt::Baling(_, pos) => *pos,
            Stmt::Ungkapan(_, pos) => *pos,
            Stmt::Horas(pos) => *pos,
            Stmt::Mangan(pos) => *pos,
            Stmt::Ambil { pos, .. } => *pos,
            Stmt::Uji { pos, .. } => *pos,
            Stmt::Semak { pos, .. } => *pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub pernyataan: Vec<Stmt>,
}
