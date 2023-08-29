use std::env;
pub mod tree;
use std::path::PathBuf;
pub mod filesys;
use crate::tree::{Tree, NodeIdx, root_idx};
use crate::filesys::{load_full_tree, write_full_tree};

fn dfs(tree: &Tree, current_idx: NodeIdx) {
	let root = &tree.nodes[current_idx];
	println!("{:?}", root.data.path);
	for child in tree.get_children(current_idx) {
		dfs(&tree, child);
	}
}

fn copy_fold(tree: &Tree) -> Tree {
	let from_paths = (&tree.nodes).into_iter().map(|x| {
		x.data.path.clone()
	}).collect();

	let to_paths = (&tree.nodes).into_iter().map(|x| {
		x.data.path.clone()
	}).collect();

	return Tree::from_fold(&tree, from_paths, to_paths);
}

// TODO for Gleb
fn flatten_fold(tree: &Tree) -> Tree {
	let from_paths = (&tree.nodes).into_iter().map(|x| {
		x.data.path.clone()
	}).collect();

	let to_paths = (&tree.nodes).into_iter().map(|x| {
		let string = x.data.path.to_str().unwrap();
		PathBuf::from(str::replace(string, "/", "_"))
	}).collect();

	return Tree::from_fold(&tree, from_paths, to_paths);
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


	let copied_tree = copy_fold(&tree);
	let flattened_tree = flatten_fold(&tree);

	// println!("BFS");
	
	// bfs(&tree, root_idx());

	println!("DFS");

	dfs(&tree, root_idx());

	println!("DFS Copy Fold");

	dfs(&copied_tree, root_idx());

	println!("DFS Flatten Fold");

	dfs(&flattened_tree, root_idx());

	write_full_tree(tree, flattened_tree);
	
	return Ok(());
}
