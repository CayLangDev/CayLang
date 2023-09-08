use std::env;
use std::path::PathBuf;
use caylang_io::tree::{Tree, Node, NodeData, NodeIdx, root_idx};
use caylang_io::filesys::{load_full_tree};
use std::iter::{zip};
use std::collections::{VecDeque};
use regex::{Regex};
use crate::from_ast::{TreePrototype, FoldOperation, Rename};
// use caylang_io::tree::NodeData;

fn validate(tree: &Tree, prototype: TreePrototype) -> bool {
	for (l, p_l) in zip(tree.data_iter(tree.layers()), prototype.layers) {
		if !p_l.matches(l) {
			return false;
		}
	}
	return true;
	// don't verify files idgaf'
}


pub fn to_fold(tree: &Tree, fold_desc: FoldOperation) -> (Vec<PathBuf>, Vec<PathBuf>) {
	let old_paths = vec![];
	let new_paths = vec![];
	for l in tree.data_iter(tree.leaves()) {
		for (o, t) in zip(fold_desc.options, fold_desc.targets) {
			if o.matches(l) {
				old_paths.push(l.path);
				new_paths.push(new_name(l.path, t));
			}
		}
	}
	return (old_paths, new_paths);
}

pub fn new_name(path: PathBuf, target: Rename) -> PathBuf {
	// let comps = path.components().collect();
	let target_q = VecDeque::from(target);
	let name_comps = vec![];
	for (i, comp) in path.components().enumerate() {
		let c = target_q.front();
		if let Some(j) = c {
			if &i == j {
				name_comps.push(comp);
				target_q.pop_front();
			}
		}
		else {
			break;
		}
	}
	return name_comps.join("/");
}
