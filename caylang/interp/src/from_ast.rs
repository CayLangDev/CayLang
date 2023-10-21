use caylang_io::tree::NodeData;
use regex::Regex;
use std::{collections::HashMap};
use caylang_parser::ast::{
    Clause, ClauseType, Destination, Expr, FoldExpr, Ident, Literal,
    Prototype, NodePrototype, PrototypeDeclaration, NodeType
};

pub trait Matches {
    fn matches(&self, node: &NodeData) -> bool;
}

impl Matches for NodePrototype {
    fn matches(&self, node: &NodeData) -> bool {
        let p = node.path.as_os_str().to_str();
        if let Some(s) = p {
            let r = Regex::new(format!(r"^{}$", self.regex).as_str()).unwrap();
            return r.is_match(s);
        } else {
            return false;
        }
    }
}

pub type Rename = Vec<usize>;

#[derive(Debug)]
pub struct FoldOperation {
    pub options: Vec<SuperIdent>,
    pub targets: Vec<Rename>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub prototype: Prototype,
}

#[derive(Debug)]
pub struct OperationApplication {
    pub from: String,
    pub operation: FoldOperation,
}

pub enum InterpObject {
    Declaration(Declaration),
    Application(OperationApplication),
}


pub trait ToInterpObject {
    fn to_interp_object(&self) -> Option<InterpObject>;
}

pub trait IntoInterpObject {
    fn to_interp_object(self) -> Option<InterpObject>;
}

// TODO
// dfs through fold expr clauses
// keep track of depth for each node, associate depth with variable assigned to name
// at file move clause create new rename template, but with depths instead of variable names
// template {
// 	dirs: Vec<DepthIdxs>
//  names: Vec<DepthIdxs>
// }
// when executing fold, for each leaf
// join each ancestor idx in dirs into a new ancestor path, in the stored order
// join each ancestor idx in names with the original name into a new name
// the new path is the new ancestor path with the new name added at the end.

impl IntoInterpObject for Expr {
    fn to_interp_object(self) -> Option<InterpObject> {
        match self {
            Expr::Fold(f) => f.to_interp_object(),
            Expr::PrototypeDeclaration(p) => p.to_interp_object(),
            _ => None
        }
    }
}

// tree stuff
impl IntoInterpObject for PrototypeDeclaration {
    fn to_interp_object(self) -> Option<InterpObject> {
        match self.name {
            SuperIdent::Ident::Variable(s) => Some(InterpObject::Declaration(Declaration {name: s, prototype: self.prototype})),
            SuperIdent::Ident::Ignored => None,
            SuperIdent::ParamIdent => None
        }
    }
}

// TODO
//

impl ToInterpObject for FoldExpr {
    fn to_interp_object(&self) -> Option<InterpObject> {
        let mut rename_template: Vec<Vec<usize>> = vec![];
        let mut variable_depth_map: HashMap<String, usize> = HashMap::new();

        // DFS function to process clauses.
        fn dfs(
            clause: &Clause,
            depth: usize,
            variable_depth_map: &mut HashMap<String, usize>,
            rename_template: &mut Vec<Vec<usize>>,
        ) {
            if let Some(fields) = &clause.destructured_type.fields {
                let name_var = fields
                    .iter()
                    .find(|field| field.name == Ident::Variable("name".to_string()))
                    .and_then(|field| field.alias.as_ref().or(Some(&field.name)))
                    .map(|ident| ident.to_string());

                if let Some(name) = name_var {
                    variable_depth_map.insert(name, depth);
                }
            }
            match &clause.child {
                ClauseType::SubClause(subclauses) => {
                    for subclause in subclauses {
                        dfs(subclause, depth + 1, variable_depth_map, rename_template);
                    }
                }
                ClauseType::FileRead(_, dest) => {
                    if let Destination::Move(Literal::FString(path)) = dest {
                        let elems = path
                            .split('/')
                            .map(|s| {
                                // Remove the "{" and "}"
                                let mut chars = s.chars();
                                chars.next();
                                chars.next_back();
                                let t = chars.as_str();
                                variable_depth_map.get(t).unwrap().clone()
                            })
                            .collect();
                        rename_template.push(elems);
                    }
                }
                _ => {}
            }
        }

        for clause in &self.clauses {
            dfs(clause, 0, &mut variable_depth_map, &mut rename_template);
        }

        // TODO: Once NodePrototypes are properly parsed, can do proper options here.
        let options: Vec<Ident> = vec![Ident::Variable("File".to_string()); rename_template.len()];

        Some(InterpObject::Application(OperationApplication {
            from: self.directory.clone(),
            operation: FoldOperation {
                options,
                targets: rename_template,
            },
        }))
    }
}
