pub mod ast;
use lalrpop_util::{lalrpop_mod};
lalrpop_mod!(pub syntax);

#[cfg(test)]
mod tests;