use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
use crate::tree::{Tree, Node, NodeIdx, root_idx};

pub type FileIden = PathBuf;
pub type FileNode = Node;
pub type FileSysTree = Tree;

pub fn load_full_tree(root: FileIden) -> FileSysTree {
    let mut tree: FileSysTree = Tree::new();
    // let mut root: NodeIdx = root_idx();
    
    for entry in WalkDir::new(root).sort(true) {
		let entry = entry.unwrap();

        println!("{}", entry.path().display());

        // let e: entry.path();
        // let child_idx = tree.add_child(root, e, "");
        // ...
        // need to determine if child_idx is our new root
        // entry.file_type.is_file() and entry.depth() will likely help
        //
	}

	return tree;
}

// validation-loading ideas
// the naive approach is to just full load the tree and then validate
// if the tree is massive and obviously fucked early this is dumb
// better idea?
// validate current tree every k nodes, doubling k each time

// need to edit to take a closure (tree) -> bool
pub fn load_tree_val(root: FileIden) -> FileSysTree {
    let mut tree: FileSysTree = Tree::new();

    return tree;
}
// super simple approach, a better one would use a struct with a validation trait that statefully tracks what has been validified, and pass difference from last time
// but this would be sophisticated enough for now
