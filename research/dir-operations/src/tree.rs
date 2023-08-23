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
		Self { nodes: Vec::new(), path_map: HashMap::<PathBuf, NodeIdx>::new() }
	}

	fn add_child(&mut self, parent_idx: NodeIdx, child_data: NodeData) -> NodeIdx {
		let child = Node::new(child_data);
		let child_idx = self.nodes.len();

        let parent = &mut self.nodes[parent_idx];
		parent.children.push(child_idx);
		self.nodes.push(child);

		// TODO consider this type of code if we only want to pass 
		//   name rather than all data
		//
		// child.data.path = self.nodes[parent_idx].data.path.clone()
		// child.data.path.push(child_name)

		self.path_map.insert(self.nodes[child_idx].data.path.clone(), child_idx);

		return child_idx;
    }

	fn get_node_by_path(&self, path: PathBuf) -> Option<&Node> {
		match self.path_map.get(&path) {
			None => None,
			Some(nodeIdx) => return Some(&self.nodes[*nodeIdx]),
		}
    }

    fn get_children(&self, parent_idx: NodeIdx) -> Children<'_> {
        Children {tree: &self, parent_idx, current: 0}
    }

	fn get_child(&self, parent_idx: NodeIdx, file_name: String) -> Option<&NodeIdx> {
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

impl Children<'_> {
	fn next(&mut self) -> Option<NodeIdx> {
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
