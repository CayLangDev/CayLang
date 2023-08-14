#[derive(Debug)]
pub enum Operation {
    Fold,
    Reduce,
    Find
}

#[derive(Debug)]
pub enum Path {
    Exact(String),
    Glob(String),
}

impl Path {
    pub fn unwrap(&self) -> &str {
        match self {
            Path::Exact(path) => path,
            Path::Glob(path) => path,
        }
    }
}

pub struct Rule {
    pub operation: Operation,
    pub path: Path,
    pub entries: Vec<RuleEntry>,
}

pub struct RuleEntry {
    pub pattern: Path,
    pub destination: Path,
}