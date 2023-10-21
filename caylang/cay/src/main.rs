use cay::cmd::{build, Cmd};
pub mod cay_tests;

fn main() {
    match Cmd::default() {
        Cmd::Build(args) => build::exec(args),
    }
}
