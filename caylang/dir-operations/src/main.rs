use std::env;
pub mod tree;
use std::path::PathBuf;
pub mod filesys;
use crate::tree::{Tree, NodeIdx, root_idx, NodeType};
use crate::filesys::{load_full_tree, write_full_tree};
mod treeTests;

fn dfs(tree: &Tree, current_idx: NodeIdx) {
	let root = &tree.nodes[current_idx];
	println!("{:?}", root.data.path);
	for child in tree.get_children(current_idx) {
		dfs(&tree, child);
	}
}

// fn bfs(tree: &Tree, current_idx: NodeIdx) {
// 	// ...
// 	// tbd
// }

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		println!("Err: Expected two path arguments");
		return Ok(());
	}

	let from_path = PathBuf::from(&args[1]);
	let to_path = PathBuf::from(&args[2]);

	let tree: Tree = load_full_tree((&args[1]).into());

	let flattened_tree = Tree::from_fold_function(&tree, |x| {
		let string = x.data.path.to_str().unwrap();
		PathBuf::from(str::replace(string, "/", "_"))
	});

	// // println!("BFS");
	// // bfs(&tree, root_idx());

	println!("DFS");

	dfs(&tree, root_idx());

	println!("DFS Flatten Fold");
	dfs(&flattened_tree, root_idx());

	write_full_tree(&tree, &flattened_tree);
	
	return Ok(());
}
