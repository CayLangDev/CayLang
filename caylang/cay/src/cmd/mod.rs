use clap::Parser;

pub mod build;

#[derive(Parser)]
pub enum Cmd {
    Build(build::Args)
}

impl Default for Cmd {
    fn default() -> Self {
        Self::parse()
    }
}