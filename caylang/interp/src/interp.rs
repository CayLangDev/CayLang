use std::env;
use std::path::PathBuf;
use caylang_io::tree::{Tree, Node, NodeData, NodeIdx, root_idx};
use caylang_io::filesys::{load_full_tree, FileSysTree};
use std::iter::{zip};
use std::collections::{VecDeque};
use std::regex::{Regex};

fn validate(tree: &Tree, prototype: TreePrototype) -> bool {
	for l, p_l in zip(tree.layers(), prototype.layers) {
		if !p_l.matches(l) {
			return false;
		}
	}
	// don't verify files idgaf'
}

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
	layers: Vec<NodePrototype>,
	edges: Vec<NodePrototype>
}

pub type Rename = Vec<usize>;

pub struct FoldOperation {
	options: Vec<NodePrototype>,
	targets: Vec<Rename>
}

pub fn to_fold(tree: &Tree, fold_desc: FoldOperation) -> (Vec<PathBuf>, Vec<PathBuf>) {
	old_paths = vec![];
	new_paths = vec![];
	for l in tree.leaves() {
		for o, t in zip(fold_desc.options, fold_desc.targets) {
			if o.matches(l) {
				old_paths.push(l.path);
				new_paths.push(new_name(l.path, t));
			}
		}
	}
}

pub fn new_name(path: PathBuf, target: Rename) -> {
	// let comps = path.components().collect();
	let target_q = VecDeque(target);
	let name_comps = vec![];
	for i, comp in path.components().enumerate() {
		let c = target_q.pop_front();
		if i == c {
			name_comps.push(c);
		}
	}
	return name_comps.join("/");
}

fn main() -> std::io::Result<()> {
// 	let args: Vec<String> = env::args().collect();
// 	if args.len() != 2 {
// 		println!("Err: Expected one path argument");
// 		return Ok(());
// 	}
//
// 	let tree: FileSysTree = load_full_tree((&args[1]).into());
//
// 	println!("BFS");
//
// 	bfs(&tree, root_idx());
//
// 	println!("DFS");
//
// 	dfs(&tree, root_idx());
//
// 	return Ok(());
}
