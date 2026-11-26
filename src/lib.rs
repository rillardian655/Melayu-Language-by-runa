//! # MELAYU Core Library (`melayu_core`)
//!
//! Enjin penterjemah (*interpreter*) rasmi bagi bahasa pengaturcaraan **MELAYU**.
//!
//! Saluran paip pelaksanaan terdiri daripada fasa berikut:
//! 1. [`lexer`]: Mengimbas kod sumber rentetan aksara UTF-8 kepada aliran token laras Melayu.
//! 2. [`parser`]: Menghuraikan aliran token ke dalam Pokok Sintaks Abstrak ([`ast::Program`]).
//! 3. [`analyzer`]: Melakukan pemeriksaan skop, pemalar (`tetap`), dan semantik statik.
//! 4. [`interpreter`]: Melaksanakan atur cara dengan model persekitaran bertingkat dan keselamatan sandbox.
//!
//! ## Contoh Penggunaan Cepat
//!
//! ```rust
//! use melayu_core::jalankan;
//!
//! let kod = r#"
//! ada sapaan = "Salam Serumpun"
//! tunjuk sapaan
//! "#;
//!
//! let hasil = jalankan(kod);
//! assert!(hasil.is_ok());
//! ```

pub mod analyzer;
pub mod ast;
pub mod bekas;
pub mod builtins;
pub mod errors;
pub mod gpu;
pub mod interpreter;
pub mod kamus;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod tindanan;
pub mod tokens;

use analyzer::Analyzer;
use ast::Program;
use errors::MelayuError;
use interpreter::{Interpreter, Value};
use lexer::Lexer;
use parser::Parser;

/// Menghurai dan mengesahkan kod sumber Melayu ke dalam AST
pub fn huraikan(source: &str) -> Result<Program, MelayuError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let mut analyzer = Analyzer::new();
    analyzer.analyze(&program)?;
    Ok(program)
}

/// Menjalankan rentetan kod sumber Melayu secara terus
pub fn jalankan(source: &str) -> Result<Value, MelayuError> {
    let program = huraikan(source)?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}
