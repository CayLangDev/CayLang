pub mod ast;

use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(pub syntax);

fn test(input: &str) {
    println!("Parsing: {}", input);

    let parser = syntax::MainParser::new();
    let result = parser.parse(input);

    match result {
        Ok(program) => {
            println!("{:?}",program);
        }
        Err(e) => {
            println!("Error: {:?}", e);
            match e {
                ParseError::InvalidToken{location} => {
                    println!("char: {:?}", String::from(&input[location..location+1]));
                },
                _ => ()
            }
        }
    }
}

// args
// vector of argument types

// argument
// ident -> matches ident
// rigidll<l> -> matches labelled list with labels matching l
// ll -> matches a labelled list, converts it to a hashmap
// expr -> matches some evaluated expression
    // if an argument is an identifier of a defined function its evaluated based on said functions defined arguments

// return
// some object type or None, either a string/stringvar, numeric, or a prototype

// define construct
// fn deffunc(name, args, return)
// defines a constructor of name name, with argument types args


fn main() {
    // let input1 = r#"
    //     DirectorySet SmallNumDir {
    //         Names: r"[123]",
    //         Tags: {
    //             num: asint name
    //         }
    //     }
    // "#;

    let input1 = r#"DirectorySet SmallNumDir { Names: r"[123]", Tags: { num: asint name } }"#;

    let input2 = r#"print 1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello"#;

    let input3 = r#"print {1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello}"#;

    test(input1);
    test(input2);
    test(input3);

}
