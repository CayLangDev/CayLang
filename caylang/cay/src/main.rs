use cay::cmd::{build, Cmd};

#[cfg(test)]
pub mod cay_tests;

fn main() {
    match Cmd::default() {
        Cmd::Build(args) => build::exec(args),
    }
}
