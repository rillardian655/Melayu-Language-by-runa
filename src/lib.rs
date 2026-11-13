pub mod analyzer;
pub mod ast;
pub mod builtins;
pub mod errors;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
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

