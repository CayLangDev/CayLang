use jwalk::{WalkDir};
use std::path::PathBuf;
use std::env;
use core::ops::Index;

#[derive(Debug)]
struct Node<T> {
	parent: usize,
	child_start: usize,
	leaf: bool,
	elem: T,
}

type Tree<T> = Vec<Vec<Node<T>>>;

#[derive(Debug, Copy, Clone)]
struct NodeIdx(usize, usize);

impl<T> Index<NodeIdx> for Tree<T> {
	type Output = Node<T>;

	fn index(&self, idx: NodeIdx) -> &Self::Output {
		&self[idx.0][idx.1]
	}
}

struct Children<'a, T> {
	tree: &'a Tree<T>, 
	parent_idx: NodeIdx,
	current: usize
}

fn Children<T>(tree: &Tree<T>, parent_idx: NodeIdx) -> Children<'_, T> {
	Children {tree, parent_idx, current: 0}
}

impl<T> Iterator for Children<'_, T> {
	type Item = NodeIdx;
	fn next(&mut self) -> Option<Self::Item> {
		let parent_node = &self.tree[self.parent_idx];
		if parent_node.leaf {
			return None;
		}
		
		let curr_idx = NodeIdx(self.parent_idx.0+1, parent_node.child_start + self.current);
		if curr_idx.1 >= self.tree[curr_idx.0].len() {
			return None;
		}
		
		let curr_node = &self.tree[curr_idx];
		if curr_node.parent == self.parent_idx.1 {
			self.current += 1;
			Some(curr_idx)
		}
		else {
			None
		}
	}
	
}


fn dfs<T: std::fmt::Debug>(tree: &Tree<T>, current_idx: NodeIdx) {
	// println!("{:?}", current_idx);
	let root = &tree[current_idx];
	println!("{:?}", root.elem);
	for child in Children(&tree, current_idx) {
		dfs(&tree, child);
	}
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Err: Expected one path argument");
		return Ok(());
	}

	println!("DFS");

	// let mut layers: Vec<Vec<Node<PathBuf>>> = Vec::new();
	let mut layers: Tree<PathBuf> = Vec::new();
	for entry in WalkDir::new(&args[1]).sort(true) {
		let entry = entry.unwrap();
	   	println!("{}", entry.path().display());

		let e: Node<PathBuf> = if entry.depth == 0 {
			Node::<PathBuf> {parent: 0, child_start: 0, elem: entry.path(), leaf: entry.file_type.is_file()}
		} else {
			let prior_layer = layers.get(entry.depth()-1).unwrap();
			if entry.file_type.is_file() {
				Node::<PathBuf> {parent: prior_layer.len()-1, 
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
				
				Node::<PathBuf> {parent: prior_layer.len()-1, child_start, elem: entry.path(), leaf: false}
			}
			
		};
	   	
	   	if let Some(layer) = layers.get_mut(entry.depth()) {
	   		layer.push(e);
	   	}
	   	else {
	   		layers.push(vec![e]);
	   	}
	}

	println!("BFS");
	
	for (i, layer) in ((&layers).into_iter()).enumerate() {
		println!("{i:}");
		for e in layer {
		   	println!("{:?}", e);
		}	
	}

	println!("DFS 2");

	dfs(&layers, NodeIdx(0, 0));
	
	return Ok(());
}
