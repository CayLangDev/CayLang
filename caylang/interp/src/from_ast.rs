use caylang_io::tree::NodeData;
use regex::Regex;
use std::{collections::HashMap, string};
use caylang_parser::ast::{
    Clause, ClauseType, Destination, Expr, Field, FoldExpr, Function, Ident, LabelledList, Literal,
    UnlabelledList,
    Prototype,
    NodePrototype,
    TreePrototype,
    PrototypeDeclaration,
    NodeType
};

// use caylang_io::tree::NodeData;
// use regex::Regex;

// #[derive(Clone, Debug)]
// pub struct NodePrototype {
//     regex: Regex,
// }

pub trait Matches {
    fn matches(&self, node: &NodeData) -> bool;
}

impl Matches for NodePrototype {
    // pub fn new(regex: &str) -> Self {
    //     return Self {
    //         regex: Regex::new(format!(r"^{}$", regex).as_str()).unwrap(),
    //     };
    // }

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

// #[derive(Debug)]
// pub struct TreePrototype {
//     regex: Regex,
//     pub layers: Vec<NodePrototype>,
//     pub edges: Vec<NodePrototype>,
// }

pub type Rename = Vec<usize>;

#[derive(Debug)]
pub struct FoldOperation {
    pub options: Vec<(Ident, NodePrototype)>,
    pub targets: Vec<Rename>,
}

// #[derive(Debug)]
// pub enum Prototype {
//     Node(NodePrototype),
//     Tree(TreePrototype),
// }

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub prototype: Prototype,
}

#[derive(Debug)]
pub struct OperationApplication {
    from: String,
    operation: FoldOperation,
}

pub enum InterpObject {
    Declaration(Declaration),
    Application(OperationApplication),
}


pub trait toInterpObject {
    fn to_interp_object(&self) -> Option<InterpObject>;
}

pub trait intoInterpObject {
    fn to_interp_object(self) -> Option<InterpObject>;
}

//  Todo
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

// impl toInterpObject for FoldExpr {
// 	fn to_interp_object(&self) -> Option<InterpObject> {

// 		for clause in self.clauses {

// 		}

// 		return Some(InterpObject::Application( OperationApplication {from: self.directory, operation: ""}));

// 	}
// }

impl intoInterpObject for Expr {
    fn to_interp_object(self) -> Option<InterpObject> {
        match self {
            Expr::Fold(f) => f.to_interp_object(),
            Expr::PrototypeDeclaration(p) => p.to_interp_object(),
            _ => None
        }
    }
}

impl intoInterpObject for PrototypeDeclaration {
    fn to_interp_object(self) -> Option<InterpObject> {
        match self.name {
            Ident::Variable(s) => Some(InterpObject::Declaration(Declaration {name: s, prototype: self.prototype})),
            Ident::Ignored => None
        }
    }
}


// impl toInterpObject for Prototype {
//     fn to_interp_object(&self) -> Option<InterpObject> {
//         match self.name {
//             Ident::Variable(s) => Some(InterpObject::Declaration(Declaration {name: s, prototype: prototype})),
//             Ident::Ignored => None
//         }
//     }
// }


impl toInterpObject for FoldExpr {
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
        let options: Vec<(Ident, NodePrototype)> = vec![
            (
                Ident::Variable("SomePrototypeName".to_string()),
                NodePrototype {
                    regex: r".*".to_string(),
                    node_type: NodeType::File
                }
            );
            rename_template.len()
        ];

        Some(InterpObject::Application(OperationApplication {
            from: self.directory.clone(),
            operation: FoldOperation {
                options,
                targets: rename_template,
            },
        }))
    }
}
