mod ast;
mod tests;

fn main() {
    println!("Hello, world!");
}

// use lalrpop_util::{lalrpop_mod, ParseError};
// lalrpop_mod!(pub fold_syntax);

// fn test(input: &str) {
//     println!("Parsing: {}", input);

//     let parser = fold_syntax::MainParser::new();
//     let result = parser.parse(input);

//     match result {
//         Ok(program) => {
//             println!("{:?}", program);
//         }
//         Err(e) => {
//             println!("Error: {:?}", e);
//             match e {
//                 ParseError::InvalidToken { location } => {
//                     println!("char: {:?}", String::from(&input[location..location + 1]));
//                 }
//                 _ => (),
//             }
//         }
//     }
// }

// // args
// // vector of argument types

// // argument
// // ident -> matches ident
// // rigidll<l> -> matches labelled list with labels matching l
// // ll -> matches a labelled list, converts it to a hashmap
// // expr -> matches some evaluated expression
// // if an argument is an identifier of a defined function its evaluated based on said functions defined arguments

// // return
// // some object type or None, either a string/stringvar, numeric, or a prototype

// // define construct
// // fn deffunc(name, args, return)
// // defines a constructor of name name, with argument types args

// fn main() {
//     // let input1 = r#"
//     //     DirectorySet SmallNumDir {
//     //         Names: r"[123]",
//     //         Tags: {
//     //             num: asint name
//     //         }
//     //     }
//     // "#;

//     let input1 = r#"DirectorySet SmallNumDir { Names: r"[123]", Tags: { num: asint name } }"#;

//     let input2 = r#"print 1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello"#;

//     let input3 = r#"print {1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello}"#;

//     // a/3 b/1
//     let input4 = r#"a 1 b 2"#; // evaluates to  (a 1 (b 2)); throws error on not enough arguments for a (in interpreter not parser)
//     let input5 = r#"a 1 (b) 2"#; // evaluate to (a 1 (b) 2); no error

//     test(input1);
//     test(input2);
//     test(input3);
//     test(input4);
//     test(input5);
// }
