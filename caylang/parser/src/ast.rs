use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    ExprList(Vec<Expr>),
    Fold(FoldExpr),
    PrototypeDeclaration(PrototypeDeclaration),
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
    pub label: Option<Ident>,
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Ident {
    Variable(String),
    Ignored,
}

pub fn to_ident(s: &str) -> Ident {
    if s == "_" {
        return Ident::Ignored;
    } else {
        return Ident::Variable(s.to_string());
    }
}

#[derive(Debug)]
pub enum Destination {
    NoChange,
    Move(Literal),
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

pub trait GetValue {
    fn get_value<T: PartialEq<String>>(self, target: T) -> Option<Expr>;
    fn get_value_r<'a, T: PartialEq<&'a String>>(&'a self, target: T) -> Option<&'a Expr>;

    fn get_values<const N: usize>(self, targets: [Ident; N]) -> [Option<Expr>; N];
}

impl GetValue for LabelledList {
    fn get_value<T: PartialEq<String>>(self, target: T) -> Option<Expr> {
        for Pair(key, value) in self {
            if let Ident::Variable(key) = key {
                if target == key {
                    return Some(value);
                }
            }
        }
        return None;
    }

    fn get_values<const N: usize>(mut self, targets: [Ident; N]) -> [Option<Expr>; N] {
        let mut map: HashMap<Ident, Expr, RandomState> =
            HashMap::from_iter(self.into_iter().map(|Pair(k, v)| (k, v)));

        return targets.map(|x| map.remove(&x));
    }

    fn get_value_r<'a, T: PartialEq<&'a String>>(&'a self, target: T) -> Option<&'a Expr> {
        for Pair(key, value) in self {
            if let Ident::Variable(key) = key {
                if target == key {
                    return Some(value);
                }
            }
        }
        return None;
    }
}

// pub fn singular_expr<'a>(e: &'a Expr) -> Option<&'a Expr> {
//     if let Expr::ExprList(e) = e {
//         if e.len() == 1 {
//             let e = &e[0];
//             if let Expr::ExprList(_) = e {
//                 return None
//             }
//             else {
//                 return Some(e);
//             }
//         }
//         else {
//             return None
//         }
//     }
//     else {
//         return Some(&e)
//     }
// }

pub fn v_singular_expr(e: Vec<Expr>) -> Option<Expr> {
    if e.len() == 1 {
        // wow what an awesome language
        // I love rust so cool
        let mut e = e;
        return e.pop();
    } else {
        return None;
    }
}

pub fn singular_expr(e: Expr) -> Option<Expr> {
    if let Expr::ExprList(e) = e {
        return v_singular_expr(e);
    } else {
        return Some(e);
    }
}

pub fn as_regex(e: Expr) -> Option<String> {
    if let Some(e) = e {
        if let Some(Expr::Literal(Literal::Regex(r))) = singular_expr(e) {
            r.to_string();
        }
    }
    return None;
}

pub fn as_labelled_list(e: Expr) -> Option<LabelledList> {
    if let Some(e) = e {
        if let Some(Expr::LabelledList(l)) = singular_expr(e) {
            return l;
        }
    }
    return None;
}

pub fn as_ident(e: Expr) -> Option<Ident> {
    if let Some(e) = e {
        if let Some(Expr::Ident(i)) = singular_expr(e) {
            return i;
        }
    }
    return None;
}

pub fn as_structure_list(l: LabelledList) -> Option<StructureList> {
    return l.map(|Pair(i, e)| StructurePair(i, as_ident(e)?));
}

pub type UnlabelledList = Vec<Expr>;

#[derive(Debug)]
pub struct Pair(pub Ident, pub Expr);

#[derive(Debug)]
pub struct PrototypeDeclaration {
    pub name: Ident,
    pub prototype: Prototype,
}

#[derive(Debug)]
pub enum Prototype {
    NodePrototype(NodePrototype),
    TreePrototype(TreePrototype),
}

#[derive(Debug)]
pub struct TreePrototype {
    pub regex: String,
    pub layers: StructureList,
    pub edges: StructureList,
}

// .0 refers to prototype label, .1 refers to prototype identifier
// no expression prototypes rn
#[derive(Debug)]
pub struct StructurePair(pub Ident, pub Ident);

type StructureList = Vec<StructurePair>;

#[derive(Debug)]
pub enum NodeType {
    File,
    Dir,
}

#[derive(Debug)]
pub struct NodePrototype {
    pub regex: String,
    pub node_type: NodeType,
}

pub fn dodgy_or<T>(o: Option<T>, alt: T) {
    if let Some(r) = o {
        return r;
    }
    else {
        return alt;
    }
}
