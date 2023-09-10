use std::env;
pub mod tree;
use std::path::PathBuf;
pub mod filesys;
use crate::filesys::{load_full_tree, write_full_tree};
use crate::tree::{root_idx, NodeIdx, Tree};
mod tree_tests;

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

    let tree: Tree = load_full_tree((&args[1]).into());

    let flattened_tree = Tree::from_fold_function(&tree, |x| {
        let string = x.data.path.to_str().unwrap();
        let tmp_string = str::replace(string, "/", "_");
        PathBuf::from(&str::replacen(&tmp_string, "_", "/", 1))
    });

    println!("DFS");

    dfs(&tree, root_idx());

    println!("DFS Flatten Fold");
    dfs(&flattened_tree, root_idx());

    write_full_tree(&tree, &flattened_tree);

    return Ok(());
}
