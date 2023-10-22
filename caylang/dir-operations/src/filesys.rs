// use crate::filesys_builder::FileNode;
use crate::tree::{NodeData, NodeIdx, Tree, NodeType};
// use caylang_test::{NodeType};
use jwalk::WalkDir;
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};
extern crate fs_extra;
use fs_extra::dir;
use fs_extra::dir::copy;

use dir_diff::is_different;
use tempdir::TempDir;

pub fn load_full_tree(root: &PathBuf) -> Tree {
    let mut tree: Tree = Tree::new();
    // let mut root: NodeIdx = root_idx();
    for entry in WalkDir::new(&root).sort(true) {
        match entry {
            Ok(entry) => {
                let node_type = match entry.file_type().is_file() {
                    true => NodeType::File,
                    false => NodeType::Directory,
                };

                let relative_path =
                    PathBuf::from(PathBuf::from(entry.path()).strip_prefix(&root).unwrap());

                let mut parent_path = relative_path.clone();
                parent_path.pop();

                print!("{} {}\n", entry.path().display(), parent_path.display());
                // Path sorting means we can expect the parent to exist
                let parent_idx = tree.path_map.get(&parent_path);

                println!("root: {} path: {}", root.display(), relative_path.display());

                match parent_idx {
                    Some(idx) => {
                        tree.add_child(*idx, NodeData::new(relative_path, node_type));
                    }
                    None => {
                        tree.add_child(0, NodeData::new(relative_path, node_type));
                    }
                }
            }
            _ => println!("File doesn't exist"),
        }
    }

    return tree;
}

fn create_all_parents(path: &PathBuf) {
    let prefix = path.as_path().parent().unwrap();
    let _ = std::fs::create_dir_all(prefix);
}

pub fn write_full_tree(from_path: &PathBuf, to_path: &PathBuf, from_tree: &Tree, to_tree: &Tree) {
    for node in to_tree.nodes.iter() {
        let mut abs_path = to_path.clone();
        abs_path.push(&node.data.path);
        let mut abs_original_path = from_path.clone();
        abs_original_path.push(&node.data.original_path);

        match node.data.node_type {
            NodeType::File => {
                // println!("{} -> {}", node.data.original_path.display(), node.data.path.display());
                create_all_parents(&abs_path);
                let _ = fs::rename(&abs_original_path, &abs_path);
            }
            _ => (),
        }
    }

    for node in from_tree.nodes.iter() {
        let mut abs_path = to_path.clone();
        abs_path.push(&node.data.path);
        let mut abs_original_path = from_path.clone();
        abs_original_path.push(&node.data.original_path);

        println!("{} {}", node.data.path.display(), abs_path.display());

        match node.data.node_type {
            NodeType::Directory => match to_tree.path_map.get(&node.data.path) {
                None => {
                    let _ = fs::remove_dir_all(&abs_path);
                    let _ = fs::remove_dir(&abs_path);
                }
                Some(_) => (),
            },
            _ => (),
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
