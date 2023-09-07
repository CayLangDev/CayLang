// pointers
// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// https://doc.rust-lang.org/rust-by-example/fn/methods.html
// https://doc.rust-lang.org/rust-by-example/trait.html
use std::collections::HashMap;
use std::path::PathBuf;

pub type NodeIdx = usize;

#[derive(Debug)]
pub struct NodeData {
	pub path: PathBuf
}

#[derive(Debug)]
pub struct Node {
	pub(crate) parent: NodeIdx,
	pub(crate) children: Vec<NodeIdx>,
	pub data: NodeData,
}

impl Node {
	pub fn new(data: NodeData) -> Self { 
		Self{ parent: 0, children: vec![], data } 
	}
}

#[derive(Debug)]
pub struct Tree {
	pub nodes: Vec<Node>,
	pub path_map: HashMap<PathBuf, NodeIdx>,
}

pub fn root_idx() -> NodeIdx {
    return 0;
}

impl Tree {
	pub fn new() -> Self {
		let mut new_tree = Self { nodes: Vec::new(), path_map: HashMap::<PathBuf, NodeIdx>::new() };
		
		let mut root = Node::new(NodeData { path: "".into() });
		root.parent = 0;
		new_tree.nodes.push(root);

		return new_tree;
	}

	pub fn add_child(&mut self, parent_idx: NodeIdx, child_data: NodeData) -> NodeIdx {
		let mut child = Node::new(child_data);
		let child_idx = self.nodes.len();

        let parent = &mut self.nodes[parent_idx];
		parent.children.push(child_idx);

		child.parent = parent_idx;

		self.nodes.push(child);
		self.path_map.insert(self.nodes[child_idx].data.path.clone(), child_idx);

		return child_idx;
    }

	pub fn get_node_by_path(&self, path: PathBuf) -> Option<&Node> {
		match self.path_map.get(&path) {
			None => None,
			Some(nodeIdx) => return Some(&self.nodes[*nodeIdx]),
		}
    }

    pub fn get_children(&self, parent_idx: NodeIdx) -> Children<'_> {
        Children {tree: &self, parent_idx, current: 0}
    }

	pub fn get_child(&self, parent_idx: NodeIdx, file_name: String) -> Option<&NodeIdx> {
		let parent = &self.nodes[parent_idx];
		let mut child_path = parent.data.path.clone();
		child_path.push(file_name);

		return self.path_map.get(&child_path);
	}
}

// 'a is life-time shit
pub struct Children<'a> {
	tree: &'a Tree,
	parent_idx: NodeIdx,
	current: usize
}

impl Iterator for Children<'_> {
	type Item = NodeIdx;
	fn next(&mut self) -> Option<Self::Item> {
		let parent_node = &self.tree.nodes[self.parent_idx];

		if self.current >= parent_node.children.len() {
			return None;
		}

		let curr_idx = parent_node.children[self.current];
		self.current += 1;

		let curr_node = &self.tree.nodes[curr_idx];

		return Some(curr_idx);
	}
}

// 'a is life-time shit
pub struct Leaves<'a> {
	tree: &'a Tree,
	current: usize
}

impl Iterator for Leaves<'_> {
	type Item = NodeIdx;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current == tree.nodes.len() {
			return None
		}

		while tree.nodes[self.current].children.len() > 0 {
			self.current += 1;
			if self.current == tree.nodes.len() {
				return None
			}
		}

		let curr_idx = self.current;
		self.current += 1;

		return Some(curr_idx);
	}
}
