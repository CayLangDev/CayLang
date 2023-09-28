use caylang_parser::{syntax};
use lalrpop_util::{ParseError};

use std::fmt::Debug;
use std::fs;

#[derive(clap::Args)]
/// Builds a Cay script. Currently, this only parses the script and prints the AST.
pub struct Args {
    /// The Cay script to build.
    cay_file: String,

    /// Whether to print the AST or not.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    run: bool
}

fn pretty_print<T: Debug>(input: &T) -> String {
    format!("{:#?}", input)
}

pub fn exec(
    Args {
        cay_file,
        verbose,
        run,
    }: Args
) {
    let cay_script = match fs::read_to_string(&cay_file) {
        Ok(script) => script,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            return;
        }
    };

    // Create a new parser instance and parse the script
    let parser = syntax::MainParser::new();
    let result = parser.parse(&cay_script);

    // Handle the parsing result
    match result {
        Ok(program) => {
            // Pretty-print the parsed program (AST)
            println!("Successfully parsed {}!", &cay_file);

            if verbose {
                println!("Parsed program:\n{}", pretty_print(&program));
            } 
            if run {
                println!("Trying to run");
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            match e {
                ParseError::InvalidToken { location } => {
                    println!("char: {:?}", String::from(&cay_script[location..location + 1]));
                }
                _ => (),
            }
        }
    }
}
