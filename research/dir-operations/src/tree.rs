// pointers
// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// https://doc.rust-lang.org/rust-by-example/fn/methods.html
// https://doc.rust-lang.org/rust-by-example/trait.html


pub type NodeIdx = usize;

#[derive(Debug)]
pub struct Node<T> {
	pub(crate) parent: NodeIdx,
	pub(crate) children: Vec<NodeIdx>,
	pub elem: T,
}

impl<T> Node<T> {
	fn new(elem: T) {
		return Node<T> {parent: 0, children: vec![], elem};
	}
}

pub type Tree<T> = Vec<Node<T>>;

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

impl<T> AddChild<T> for Tree<T> {
	fn add_child(&self, parent_idx: NodeIdx, child_elem: T) -> NodeIdx {
        let child = Node<T>::new(child_elem);
		let child_idx = self.len();
		// ...
		return child_idx;
    }
}

impl<T> Iterator for Children<'_, T> {
	type Item = NodeIdx;
	fn next(&mut self) -> Option<Self::Item> {
		let parent_node = &self.tree[self.parent_idx];
		if self.current >= parent_node.children.len() {
			return None;
		}

		let curr_idx = parent_node.children[self.current];

		let curr_node = &self.tree[curr_idx];

		return Some(curr_idx);
	}

}
