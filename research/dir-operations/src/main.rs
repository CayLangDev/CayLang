use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
pub mod tree;
pub mod filesys;
use crate::tree::{Tree, Node, NodeData, NodeIdx, root_idx};
use crate::filesys::{load_full_tree, FileSysTree};

fn dfs(tree: &Tree, current_idx: NodeIdx) {
	let root = &tree.nodes[current_idx];
	println!("{:?}", root.data.path);
	for child in tree.get_children(current_idx) {
		dfs(&tree, child);
	}
}

fn bfs(tree: &Tree, current_idx: NodeIdx) {
	// ...
	// tbd
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Err: Expected one path argument");
		return Ok(());
	}

	let mut tree: FileSysTree = load_full_tree((&args[1]).into());

	tree.add_node(NodeData{ path: PathBuf::from("a/f/g/h.txt") });
	tree.add_node(NodeData{ path: PathBuf::from("b/f/g/h.txt") });

	println!("BFS");
	
	bfs(&tree, root_idx());

	println!("DFS");

	dfs(&tree, root_idx());
	
	return Ok(());
}
