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

}
