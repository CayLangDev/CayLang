use std::env;
pub mod tree;
use std::path::PathBuf;
pub mod filesys;
use crate::tree::{Tree, NodeIdx, root_idx, NodeType, NodeData};
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

fn create_dataset() -> Tree {
	let mut dataset = Tree::new();
	dataset.add_child(root_idx(), NodeData::new(PathBuf::from("1"), NodeType::Directory));
	dataset.add_child(root_idx(), NodeData::new(PathBuf::from("2"), NodeType::Directory));
	dataset.add_child(root_idx(), NodeData::new(PathBuf::from("3"), NodeType::Directory));
	dataset.add_child(1, NodeData::new(PathBuf::from("1/file_a.txt"), NodeType::File));
	dataset.add_child(1, NodeData::new(PathBuf::from("1/file_b.txt"), NodeType::File));
	dataset.add_child(1, NodeData::new(PathBuf::from("1/file_c.txt"), NodeType::File));
	dataset.add_child(2, NodeData::new(PathBuf::from("2/file_d.txt"), NodeType::File));
	dataset.add_child(2, NodeData::new(PathBuf::from("2/file_e.txt"), NodeType::File));
	dataset.add_child(2, NodeData::new(PathBuf::from("2/file_f.txt"), NodeType::File));
	dataset.add_child(3, NodeData::new(PathBuf::from("3/file_g.txt"), NodeType::File));
	dataset.add_child(3, NodeData::new(PathBuf::from("3/file_h.txt"), NodeType::File));
	dataset.add_child(3, NodeData::new(PathBuf::from("3/file_i.txt"), NodeType::File));
	return dataset;
}

fn main() -> std::io::Result<()> {
	// let args: Vec<String> = env::args().collect();
	// if args.len() != 3 {
	// 	println!("Err: Expected two path arguments");
	// 	return Ok(());
	// }

	// let from_path = PathBuf::from(&args[1]);
	// let to_path = PathBuf::from(&args[2]);

	// let tree: Tree = load_full_tree((&args[1]).into());

	// let flattened_tree = Tree::from_fold_function(&tree, |x| {
	// 	let string = x.data.path.to_str().unwrap();
	// 	PathBuf::from(str::replace(string, "/", "_"))
	// });

	// // // println!("BFS");
	// // // bfs(&tree, root_idx());

	// println!("DFS");

	// dfs(&tree, root_idx());

	// println!("DFS Flatten Fold");
	// dfs(&flattened_tree, root_idx());

	// write_full_tree(&tree, &flattened_tree);
	
	// return Ok(());

	let dataset = create_dataset();

	dataset.bfs();

	println!("\n----------------------------------------------------------------POST-FLATTEN---------------------------------------------------------------------\n");
	
	let from_paths: Vec<PathBuf> = vec![PathBuf::from("1/file_a.txt"), PathBuf::from("1/file_b.txt"), PathBuf::from("1/file_c.txt"),
		PathBuf::from("2/file_d.txt"), PathBuf::from("2/file_e.txt"), PathBuf::from("2/file_f.txt"), PathBuf::from("3/file_g.txt"),
		PathBuf::from("3/file_h.txt"), PathBuf::from("3/file_i.txt")];

	let to_paths: Vec<PathBuf> = vec![PathBuf::from("1_file_a.txt"), PathBuf::from("1_file_b.txt"), PathBuf::from("1_file_c.txt"),
	PathBuf::from("2_file_d.txt"), PathBuf::from("2_file_e.txt"), PathBuf::from("2_file_f.txt"), PathBuf::from("3_file_g.txt"),
	PathBuf::from("3_file_h.txt"), PathBuf::from("3_file_i.txt")];
	
	let new_dataset = Tree::from_fold(&dataset, from_paths, to_paths);
	
	new_dataset.bfs();
	
	return Ok(());
}
