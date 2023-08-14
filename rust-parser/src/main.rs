pub mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub fold_syntax);

fn main() {
    let input = r#"
        fold dir/ {
            *grades_2018.csv => grades/2018.csv
            grades_2019.csv => grades/2019.csv
        }
    "#;

    println!("Parsing: {}", input);

    let parser = fold_syntax::MainParser::new();
    let result = parser.parse(input);

    match result {
        Ok(program) => {
            for rule in program {
                println!("Operation: {:?}", rule.operation);
                match rule.path {
                    ast::Path::Exact(path) => println!("Directory Path: {}", path),
                    ast::Path::Glob(path) => println!("Directory Glob Pattern: {}", path),
                }
                for entry in rule.entries {
                    match entry.pattern {
                        ast::Path::Exact(path) => println!("Path: {}", path),
                        ast::Path::Glob(path) => println!("Glob Pattern: {}", path),
                    }
                    println!("Destination: {}", entry.destination.unwrap());
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
