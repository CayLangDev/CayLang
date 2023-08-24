use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
use crate::tree::{Tree, Node, NodeData, NodeIdx, root_idx};

pub type FileIden = PathBuf;
pub type FileNode = Node;
pub type FileSysTree = Tree;

pub fn load_full_tree(root: FileIden) -> FileSysTree {
    let mut tree: FileSysTree = Tree::new();
    // let mut root: NodeIdx = root_idx();
    
    for entry in WalkDir::new(&root).sort(true) {
		let entry = entry.unwrap();

        let mut parent_path = entry.path().clone();
        parent_path.pop();
        // Path sorting means we can expect the parent to exist
        let parent_idx = tree.path_map.get(&parent_path);

        match parent_idx {
            Some (idx) => {
                tree.add_child(*idx, NodeData { path: entry.path() });
            }
            None => {
                tree.add_child(0, NodeData { path: entry.path() });
            }
        }
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
