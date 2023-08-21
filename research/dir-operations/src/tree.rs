use core::ops::Index;
use core::ops::IndexMut;

#[derive(Debug)]
pub struct Node<T> {
	pub(crate) parent: usize,
	pub(crate) child_start: usize,
	pub(crate) leaf: bool,
	pub elem: T,
}

pub type Layer<T> = Vec<Node<T>>;
pub type Tree<T> = Vec<Layer<T>>;

#[derive(Debug, Copy, Clone)]
pub struct NodeIdx(pub usize, pub usize);

pub fn root_idx() -> NodeIdx {
    return NodeIdx(0, 0);
}

impl<T> Index<NodeIdx> for Tree<T> {
	type Output = Node<T>;
	fn index(&self, idx: NodeIdx) -> &Self::Output {
		&self[idx.0][idx.1]
	}
}

impl<T> IndexMut<NodeIdx> for Tree<T> {
	fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
		&mut self[idx.0][idx.1]
	}
}

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
