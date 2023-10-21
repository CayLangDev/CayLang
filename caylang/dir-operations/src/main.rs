use std::env;
pub mod tree;
use std::path::{Path, PathBuf};
use std::process::Child;
pub mod filesys;
use crate::filesys::{load_full_tree, write_full_tree};
use crate::tree::{root_idx, NodeIdx, Tree};
pub mod filesys_builder;
mod filesys_tests;
mod tree_tests;

use crate::filesys_builder::FileNode;

fn dfs(tree: &Tree, current_idx: NodeIdx) {
    let root = &tree.nodes[current_idx];
    println!("{:?}", root.data.path);
    for child in tree.get_children(current_idx) {
        dfs(&tree, child);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Flatten operation failed!\nExpected:\ncargo run <path>");
        return Ok(());
    }

    let path = PathBuf::from(&args[1]);
    let to_path = PathBuf::from(&args[2]);
    let tree: Tree = load_full_tree(&path);

    let flattened_tree = Tree::from_fold_function(&tree, |x| {
        let string = x.data.path.to_str().unwrap();
        PathBuf::from(str::replace(string, "/", "_"))
    });

    println!("DFS");

    dfs(&tree, root_idx());

    println!("DFS Flatten Fold");
    dfs(&flattened_tree, root_idx());

    write_full_tree(&path, &to_path, &tree, &flattened_tree);

    return Ok(());
}
