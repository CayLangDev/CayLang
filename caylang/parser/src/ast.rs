#[derive(Debug)]
pub enum Expr {
    ExprList(Vec<Expr>),
    Fold(FoldExpr),
    LabelledList(LabelledList),
    UnlabelledList(UnlabelledList),
    Ident(Ident),
    Literal(Literal),
}

#[derive(Debug)]
pub struct FoldExpr {
    pub directory: String,
    pub dir_type: TypeDestructured,
    pub clauses: Vec<Clause>,
}

#[derive(Debug)]
pub enum ClauseType {
    Guarded(Vec<Guard>),
    // File ops, destination
    FileRead(Vec<Function>, Destination),
    SubClause(Vec<Clause>),
}

#[derive(Debug)]
pub struct Clause {
    pub label: Ident,
    pub destructured_type: TypeDestructured,
    pub child: ClauseType,
}

#[derive(Debug)]
pub struct Guard {
    pub conditions: Vec<Function>,
    pub child: ClauseType,
}

#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub alias: Option<Ident>,
}

#[derive(Debug)]
pub struct TypeDestructured {
    pub name: Ident,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug)]
pub enum Ident {
    Variable(String),
    Ignored,
}

#[derive(Debug)]
pub enum Destination {
    NoChange,
    Move(Literal)
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    FString(String),
    Regex(String),
    Path(String),
    Numeric(f64),
}

pub fn stripstr(s: &str, i: usize) -> String {
    let s2 = String::from(s);
    return String::from(&s2[i..s2.len() - 1]);
}

pub type LabelledList = Vec<Pair>;

pub type UnlabelledList = Vec<Expr>;

#[derive(Debug)]
pub struct Pair(pub Ident, pub Expr);
