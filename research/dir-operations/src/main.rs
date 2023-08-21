use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
pub mod tree;
pub mod filesys;
use crate::tree::{Tree, Node, GetChildren, NodeIdx, root_idx};
use crate::filesys::{load_full_tree, FileSysTree};

fn dfs<T: std::fmt::Debug>(tree: &Tree<T>, current_idx: NodeIdx) {
	let root = &tree[current_idx];
	println!("{:?}", root.elem);
	for child in tree.get_children(current_idx) {
		dfs(&tree, child);
	}
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Err: Expected one path argument");
		return Ok(());
	}

	let tree: FileSysTree = load_full_tree((&args[1]).into());

	println!("BFS");
	
	for (i, layer) in ((&layers).into_iter()).enumerate() {
		println!("{i:}");
		for e in layer {
		   	println!("{:?}", e);
		}	
	}

	println!("DFS 2");

	dfs(&layers, root_idx());
	
	return Ok(());
}
