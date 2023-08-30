// #[derive(Debug)]
// pub enum Operation {
//     Fold,
//     Reduce,
//     Find
// }
//
// #[derive(Debug)]
// pub enum Path {
//     Exact(String),
//     Glob(String),
// }
//
// impl Path {
//     pub fn unwrap(&self) -> &str {
//         match self {
//             Path::Exact(path) => path,
//             Path::Glob(path) => path,
//         }
//     }
// }
//
// pub struct Rule {
//     pub operation: Operation,
//     pub path: Path,
//     pub entries: Vec<RuleEntry>,
// }
//
// pub struct RuleEntry {
//     pub pattern: Path,
//     pub destination: Path,
// }

// comptime representation of a labelled list

#[derive(Debug)]
pub enum Expr {
    ExprList(Vec<Expr>),
    LabelledList(LabelledList),
    UnlabelledList(UnlabelledList),
    Ident(Ident),
    Literal(Literal)
}


pub type Ident = String;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Regex(String),
    Path(String),
    Numeric(f64)
}

pub fn stripstr(s: &str, i: usize) -> String {
    let s2 = String::from(s);
    return String::from(&s2[i..s2.len()-1]);
}

pub type LabelledList = Vec<Pair>;

pub type UnlabelledList = Vec<Expr>;

#[derive(Debug)]
pub struct Pair(pub Ident, pub Expr);
