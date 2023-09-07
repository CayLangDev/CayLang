use cay::cmd::{build, Cmd};

fn main() {
    match Cmd::default() {
        Cmd::Build(args) => build::exec(args)
    }
}