//! File relocation literal ("template") parser
//! Custom implementation of a LR(1) parser with integrated lexing because
//! LALRPOP doesn't allow whitespace without writing our own lexer.

pub mod ast;
pub mod parse;

#[cfg(test)]
mod test;
