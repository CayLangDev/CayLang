use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
use crate::tree::{Tree, Node, GetChildren, NodeIdx, root_idx};

pub type FileIden = PathBuf;
pub type FileNode = Node<FileIden>;
pub type FileSysTree = Tree<FileIden>;


pub fn load_full_tree(root: FileIden) -> FileSysTree {
    let mut layers: FileSysTree = Vec::new();

    for entry in WalkDir::new(root).sort(true) {
		let entry = entry.unwrap();

		let e: FileNode = if entry.depth == 0 {
			FileNode {parent: 0, child_start: 0, elem: entry.path(), leaf: entry.file_type.is_file()}
		} else {
			let prior_layer = layers.get(entry.depth()-1).unwrap();
			if entry.file_type.is_file() {
				FileNode {parent: prior_layer.len()-1,
								 child_start: 0, elem: entry.path(),
								 leaf: true}
			}
			else {
				let child_start = if let Some(next_layer) = layers.get(entry.depth()+1) {
					next_layer.len()
				}
				else {
					0
				};
				FileNode {parent: prior_layer.len()-1, child_start, elem: entry.path(), leaf: false}
			}

		};

	   	if let Some(layer) = layers.get_mut(entry.depth()) {
	   		layer.push(e);
	   	}
	   	else {
	   		layers.push(vec![e]);
	   	}
	}

	return layers;

}
