use crate::tree::{NodeData, NodeIdx, NodeType, Tree};
use jwalk::WalkDir;
use std::fs;
use std::path::PathBuf;
extern crate fs_extra;
use fs_extra::dir;
use fs_extra::dir::copy;

use dir_diff::is_different;
use tempdir::TempDir;

fn dfs(tree: &Tree, current_idx: NodeIdx) {
    let root = &tree.nodes[current_idx];
    println!("{:?}", root.data.path);
    for child in tree.get_children(current_idx) {
        dfs(&tree, child);
    }
}

pub fn run_test(path: &str, f: fn(&Tree) -> Tree) {
    let test_path = PathBuf::from(path);
    let mut in_path = test_path.clone();
    in_path.push("in");
    let mut out_path = test_path.clone();
    out_path.push("out");

    let tmp_dir = TempDir::new("test").unwrap();
    let root_path = PathBuf::from(tmp_dir.path());

    let options = dir::CopyOptions::new();
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
    let _ = copy(&in_path, &root_path, &options);
    let mut tmp_in = root_path.clone();
    tmp_in.push("in");
    let mut tmp_out = root_path.clone();
    tmp_out.push("out");
    let _ = fs::rename(&tmp_in, &tmp_out);

    let tree = load_full_tree(&tmp_out);
    let new_tree = f(&tree);

    write_full_tree(&tree, &new_tree, &tmp_out);
    dfs(&tree, 0);
    dfs(&new_tree, 0);

    assert!(!is_different(out_path.to_str().unwrap(), tmp_out.to_str().unwrap()).unwrap());
}

pub fn load_full_tree(root: &PathBuf) -> Tree {
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

        let relative_path = PathBuf::from(PathBuf::from(entry.path()).strip_prefix(&root).unwrap());

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

    return tree;
}

fn create_all_parents(path: &PathBuf) {
    let prefix = path.as_path().parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}

pub fn write_full_tree(from_tree: &Tree, to_tree: &Tree, root: &PathBuf) {
    for node in to_tree.nodes.iter() {
        let mut abs_path = root.clone();
        abs_path.push(&node.data.path);
        let mut abs_original_path = root.clone();
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
        let mut abs_path = root.clone();
        abs_path.push(&node.data.path);
        let mut abs_original_path = root.clone();
        abs_original_path.push(&node.data.original_path);

        println!("{} {}", node.data.path.display(), abs_path.display());

        match node.data.node_type {
            NodeType::Directory => {
                match to_tree.path_map.get(&node.data.path) {
                    None => {
                        let _ = fs::remove_dir_all(&abs_path);
                        let _ = fs::remove_dir(&abs_path);
                    }
                    Some(_) => (),
                }
                let _ = fs::rename(&abs_original_path, &abs_path);
            }
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
