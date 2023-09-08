use regex::{Regex};
use caylang_io::tree::NodeData;

pub struct NodePrototype {
	regex: Regex
}

impl NodePrototype {
	pub fn new(regex: &str) -> Self {
		return Self {regex: Regex::new(format!(r"^{}$", regex)).unwrap() };
	}

	pub fn matches(&self, node: NodeData) -> bool {
		return self.regex.is_match(node.path);
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
    fn to_interp_object(&self) -> InterpObject;
}
