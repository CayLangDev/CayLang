use jwalk::{WalkDir};
use std::fs;
use std::path::PathBuf;
use crate::tree::{Tree, Node, NodeData, NodeType};

pub fn load_full_tree(root: PathBuf) -> Tree {
    let mut tree: Tree = Tree::new();
    // let mut root: NodeIdx = root_idx();
    
    for entry in WalkDir::new(&root).sort(true) {
		let entry = entry.unwrap();

        let mut parent_path = entry.path().clone();
        parent_path.pop();
        // Path sorting means we can expect the parent to exist
        let parent_idx = tree.path_map.get(&parent_path);

        let node_type = match entry.file_type().is_file() {
            true => NodeType::File,
            false => NodeType::Directory,
        };

        match parent_idx {
            Some (idx) => {
                tree.add_child(*idx, NodeData::new(entry.path(), node_type));
            }
            None => {
                tree.add_child(0, NodeData::new(entry.path(), node_type));
            }
        }
	}

	return tree;
}

fn create_all_parents(path: &PathBuf)
{
    let prefix = path.as_path().parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}

pub fn write_full_tree(from_tree: &Tree, to_tree: &Tree)
{
    for node in to_tree.nodes.iter() {
        match node.data.node_type {
            NodeType::File => {
                // println!("{} -> {}", node.data.original_path.display(), node.data.path.display()); 
                create_all_parents(&node.data.path);
                fs::rename(&node.data.original_path, &node.data.path);
            }
            _ => ()
        }
    }

    for node in from_tree.nodes.iter() {

        match node.data.node_type {
            NodeType::Directory => {
                match to_tree.path_map.get(&node.data.path) {
                    None => {
                        fs::remove_dir_all(&node.data.path);
                        fs::remove_dir(&node.data.path);
                    }
                    Some(_) => ()
                }
                fs::rename(&node.data.original_path, &node.data.path);
            }
            _ => ()
        }
    }
}

// validation-loading ideas
// the naive approach is to just full load the tree and then validate
// if the tree is massive and obviously fucked early this is dumb
// better idea?
// validate current tree every k nodes, doubling k each time

// need to edit to take a closure (tree) -> bool

// pub fn load_tree_val(root: FileIden) -> FileSysTree {
//     let mut tree: FileSysTree = Tree::new();

//     return tree;
// }

// super simple approach, a better one would use a struct with a validation trait that statefully tracks what has been validified, and pass difference from last time
// but this would be sophisticated enough for now
