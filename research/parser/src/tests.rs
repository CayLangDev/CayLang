#[cfg(test)]
mod tests {
    use lalrpop_util::{lalrpop_mod, ParseError};
    lalrpop_mod!(pub fold_syntax);

    #[test]
    fn test_valid_folds_1() {
        let parser = fold_syntax::MainParser::new();

        let cay =
                "fold \"~/dataset\": Dataset { name as school, .. } {
                    Year { name as year, .. } => {
                      Subject { name as subject, .. } => {
                        File { name, .. }
                          | matches name \"hi.txt\" => \"hello.txt\"
                          | matches name \"data*.txt\" => \"Flattened/{school}_{year}_{subject}_{name}\"
                      }
                    }
                    _ => .
                  }";
        match parser.parse(cay) {
            Err(err) => panic!("Failed to parse fold expression: {:?}", err),
            _ => (),
        }
    }

    #[test]
    fn test_valid_folds_2() {
        let parser = fold_syntax::MainParser::new();

        let cay = "fold \"test\": MyDir {
            My => {
                Big => {
                    Cats => {
                        CsvFile { name, .. }
                          | lengthGreaterThan10 name => head 5 => reverse => \"~/Flattened/{name}\"
                    }
                }
            }
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
    fn test_invalid_folds() {
        let parser = fold_syntax::MainParser::new();

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
}
