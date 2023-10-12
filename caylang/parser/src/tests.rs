mod tests {
    use lalrpop_util::{lalrpop_mod, ParseError};
    lalrpop_mod!(pub syntax);

    fn test_parsing(input: &str) {
        let parser = syntax::MainParser::new();
        match parser.parse(input) {
            Err(err) => panic!("Failed to parse fold expression: {:?}", err),
            _ => (),
        }
    }

    fn show_parsing(input: &str) {
        let parser = syntax::MainParser::new();
        match parser.parse(input) {
            Err(err) => panic!("Failed to parse fold expression: {:?}", err),
            Ok(program) => {
                println!("{:?}", program);
            }
        }
    }

    #[test]
    fn test_valid_folds_1() {
        let cay =
                "fold \"~/dataset\": Dataset { name as school, .. } {
                    Year { name as year, .. } => {
                      Subject { name as subject, .. } => {
                        File { name, .. }
                          | matches name \"hi.txt\" => \"hello.txt\"
                          | matches name \"data*.txt\" && lengthGreaterThan name 10 => \"Flattened/{school}_{year}_{subject}_{name}\"
                      }
                    }
                    _ => .
                  }";
        test_parsing(cay);
    }

    #[test]
    fn test_valid_folds_2() {
        let parser = syntax::MainParser::new();

        let cay = "fold \"test\": MyDir {
            My => {
                Big => {
                    Cats => {
                        CsvFile { name, .. }
                          | lengthGreaterThan10 name => head 5 => reverse => \"~/Flattened/{name}\"
                    }
                }
            },
            _ => .
        }";
        match parser.parse(cay) {
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    panic!("char: {:?}", String::from(&cay[location..location + 1]));
                }
                _ => (),
            },
            _ => (),
        }
    }

    #[test]
    fn test_dirset() {
        let input = r#"DirectorySet SmallNumDir { Names: r"[123]", Tags: { num: asint name } }"#;
        test_parsing(input);
    }

    #[test]
    fn test_expr() {
        let input1 = r#"print 1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello"#;
        let input2 = r#"print {1 2.3 "hi" "" r"hi" r"" p"hi" p"" hello}"#;
        test_parsing(input1);
        test_parsing(input2);
    }

    #[test]
    fn test_invalid_folds() {
        let parser = syntax::MainParser::new();

        let test_folds = vec![
            "fold ~/dataset: Dataset", // Missing block
            "fold { =>",               // Missing source and type
        ];

        for input in test_folds {
            if parser.parse(input).is_ok() {
                panic!("Incorrectly parsed invalid fold expression");
            }
        }
    }

    #[test]
    fn test_fold_sugar() {
        let fold_sugar = r#"fold "~": LSDataSet {
            Subset: Dir {name as subset};
            Reader: ReaderDir {name as reader};
            Chapter: ChapterDir {name as chapter} => {
              Audio: File { name as audio } => `{root}/{subset}/{reader}/{chapter}/{audio}.flac`
              Transcript: File {name as transcipt} => `{root}/{subset}/{reader}/{chapter}/{transcipt}`
            }
          }"#;

        test_parsing(fold_sugar);
    }
}
