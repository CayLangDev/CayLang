use regex::{Regex};
use caylang_io::tree::NodeData;

pub struct NodePrototype {
	regex: Regex
}

impl NodePrototype {
	pub fn new(regex: &str) -> Self {
		return Self {regex: Regex::new(format!(r"^{}$", regex).as_str()).unwrap() };
	}

	pub fn matches(&self, node: &NodeData) -> bool {
		let p = node.path.as_os_str().to_str();
		if let Some(s) = p {
			return self.regex.is_match(s);
		}
		else {
			return false;
		}
	}
}

pub struct TreePrototype {
	pub layers: Vec<NodePrototype>,
	pub edges: Vec<NodePrototype>
}

pub type Rename = Vec<usize>;

pub struct FoldOperation {
	pub options: Vec<NodePrototype>,
	pub targets: Vec<Rename>
}

pub enum Prototype {
    Node(NodePrototype),
    Tree(TreePrototype),
}

pub struct Declaration {
    name: String,
    prototype: Prototype
}

pub struct OperationApplication {
    from: String,
    operation: FoldOperation
}

pub enum InterpObject {
    declaration(Declaration),
    operation_application(OperationApplication)
}

trait toInterpObject {
    fn to_interp_object(&self) -> Option<InterpObject>;
}

use caylang_parser::ast::{Expr, Ident, Function, FoldExpr, Field, ClauseType, Destination, LabelledList, UnlabelledList, Clause};

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
// 		// return OperationApplication {from: self.directory, }
// 	}
// }

// impl toInterpObject for Expr {
//
// }
//
// impl toInterpObject for Vec<Expr> {
//
// }

