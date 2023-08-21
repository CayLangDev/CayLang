// pointers
// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// https://doc.rust-lang.org/rust-by-example/fn/methods.html
// https://doc.rust-lang.org/rust-by-example/trait.html

use std::collections::HashMap;

pub type NodeIdx = usize;
pub type Path = String;

#[derive(Debug)]
pub struct Node<T> {
	pub(crate) parent: NodeIdx,
	pub(crate) children: Vec<NodeIdx>,
	pub elem: T,
}

impl<T> Node<T> {
	pub fn new(elem: T) -> Self { 
		Self{ parent: 0, children: vec![], elem } 
	}
}

#[derive(Debug)]
pub struct Tree<T> {
	pub nodes: Vec<Node<T>>,
	pub path_map: HashMap<Path, NodeIdx>,
}

impl<T> Tree<T> {
	pub fn new() -> Self { 
		Self { nodes: Vec::new(), path_map: HashMap::<Path, NodeIdx>::new() } 
	}
}

pub fn root_idx() -> NodeIdx {
    return 0;
}

// 'a is life-time shit
pub struct Children<'a, T> {
	tree: &'a Tree<T>,
	parent_idx: NodeIdx,
	current: usize
}

pub trait GetChildren<T> {
    fn get_children(&self, parent_idx: NodeIdx) -> Children<'_, T>;
}

impl<T> GetChildren<T> for Tree<T> {
    fn get_children(&self, parent_idx: NodeIdx) -> Children<'_, T> {
        Children {tree: &self, parent_idx, current: 0}
    }
}

pub trait AddChild<T> {
	fn add_child(&self, parent_idx: NodeIdx, child_elem: T, child_path: Path) -> NodeIdx; 
}

impl<T> AddChild<T> for Tree<T> {
	fn add_child(&self, parent_idx: NodeIdx, child_elem: T, child_path: Path) -> NodeIdx {
        let child = Node::new(child_elem);
		let child_idx = self.nodes.len();

		self.nodes.push(child);

		// todo implement validation for parent directories, or just create them
		self.path_map.insert(child_path, child_idx);

		return child_idx;
    }
}

pub trait GetNode<T> {
	fn get_node(&self, child_path: Path) -> Option<&Node<T>>;
}

impl<T> GetNode<T> for Tree<T> {
	fn get_node(&self, child_path: Path) -> Option<&Node<T>>{
		match self.path_map.get(&child_path) {
			None => None,
			Some(nodeIdx) => return Some(&self.nodes[*nodeIdx]),
		}
    }
}

impl<T> Iterator for Children<'_, T> {
	type Item = NodeIdx;
	fn next(&mut self) -> Option<Self::Item> {
		let parent_node = &self.tree.nodes[self.parent_idx];

		if self.current >= parent_node.children.len() {
			return None;
		}

		let curr_idx = parent_node.children[self.current];

		let curr_node = &self.tree.nodes[curr_idx];

		return Some(curr_idx);
	}
}
