use std::{collections::HashMap, string};

use caylang_io::tree::NodeData;
use regex::Regex;

pub struct NodePrototype {
    regex: Regex,
}

impl NodePrototype {
    pub fn new(regex: &str) -> Self {
        return Self {
            regex: Regex::new(format!(r"^{}$", regex).as_str()).unwrap(),
        };
    }

    pub fn matches(&self, node: &NodeData) -> bool {
        let p = node.path.as_os_str().to_str();
        if let Some(s) = p {
            return self.regex.is_match(s);
        } else {
            return false;
        }
    }
}

pub struct TreePrototype {
    pub layers: Vec<NodePrototype>,
    pub edges: Vec<NodePrototype>,
}

pub type Rename = Vec<usize>;

pub struct FoldOperation {
    pub options: Vec<NodePrototype>,
    pub targets: Vec<Rename>,
}

pub enum Prototype {
    Node(NodePrototype),
    Tree(TreePrototype),
}

pub struct Declaration {
    name: String,
    prototype: Prototype,
}

pub struct OperationApplication {
    from: String,
    operation: FoldOperation,
}

pub enum InterpObject {
    declaration(Declaration),
    Application(OperationApplication),
}

trait toInterpObject {
    fn to_interp_object(&self) -> Option<InterpObject>;
}

use caylang_parser::ast::{
    Clause, ClauseType, Destination, Expr, Field, FoldExpr, Function, Ident, LabelledList,
    UnlabelledList, Literal,
};

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

impl toInterpObject for FoldExpr {
    fn to_interp_object(&self) -> Option<InterpObject> {
        let mut depths: Vec<Option<String>> = vec![];
        let mut rename_template: Vec<Vec<usize>> = vec![];
        let mut variable_depth_map: HashMap<String, usize> = HashMap::new();

        // DFS function to process clauses.
        fn dfs(
            clause: &Clause,
            depth: usize,
            depths: &mut Vec<Option<String>>,
            variable_depth_map: &mut HashMap<String, usize>,
            rename_template: &mut Vec<Vec<usize>>,
        ) {
            if let Some(fields) = &clause.destructured_type.fields {
                let name_var = fields
                    .iter()
                    .find(|field| field.name == Ident::Variable("name".to_string()))
                    .and_then(|field| field.alias.as_ref().or(Some(&field.name)))
                    .map(|ident| ident.to_string());

                depths.push(name_var.clone());
                if let Some(name) = name_var {
                    variable_depth_map.insert(name, depth);
                }
            } else {
                depths.push(None);
            }
            match &clause.child {
                ClauseType::SubClause(subclauses) => {
                    for subclause in subclauses {
                        dfs(subclause, depth + 1, depths, variable_depth_map,rename_template);
                    }
                }
                ClauseType::FileRead(_, dest) => {
                    //todo
					if let Destination::Move(Literal::FString(path)) = dest {
						let elems = path.split('/').map(|s| {
							// Remove the "{" and "}"
							let mut chars = s.chars();
							chars.next();
							chars.next_back();
							let t = chars.as_str();
							variable_depth_map.get(t).unwrap().clone()
						} ).collect();
						rename_template.push(elems);
					}
                }
                _ => {}
            }
            let last_name_var = depths.pop().unwrap();
        }

        for clause in &self.clauses {
            dfs(clause, 0, &mut depths, &mut variable_depth_map, &mut rename_template);
        }

		// TODO: Once NodePrototypes are properly parsed, can put in all options here.
        let options: Vec<NodePrototype> = vec![NodePrototype { regex: Regex::new(r".*").unwrap()}];

        let operation = FoldOperation {
            options,
            targets: vec![], 
        };

        Some(InterpObject::Application(OperationApplication {
            from: self.directory.clone(),
            operation,
        }))
    }
}

// impl toInterpObject for Expr {
//
// }
//
// impl toInterpObject for Vec<Expr> {
//
// }
