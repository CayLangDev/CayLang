// pointers
// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// https://doc.rust-lang.org/rust-by-example/fn/methods.html
// https://doc.rust-lang.org/rust-by-example/trait.html
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub type NodeIdx = usize;

/// Whether the node is a file or directory is represented with NodeType
#[derive(Debug, Clone)]
pub enum NodeType {
    File,
    Directory,
}

/// The metadata of a node is represented with NodeData
#[derive(Debug, Clone)]
pub struct NodeData {
    /// The metadata contains the original path,
    pub original_path: PathBuf,
    /// the current path, and
    pub path: PathBuf,
    /// its type (file/directory)
    pub node_type: NodeType,
}

impl NodeData {
    /// Returns a NodeData object
    ///
    /// # Arguments
    ///
    /// * `original_path` - A PathBuf of the original path to the Node
    /// * `node_type` - A NodeType describing the Node as a file/directory
    pub fn new(original_path: PathBuf, node_type: NodeType) -> Self {
        NodeData {
            path: original_path.clone(),
            original_path: original_path,
            node_type: node_type,
        }
    }
}

/// A node in the tree is represented here
#[derive(Debug)]
pub struct Node {
    /// A node has a pointer to the parent
    pub(crate) parent: NodeIdx,
    /// a vector of pointers to the children, and
    pub(crate) children: Vec<NodeIdx>,
    /// its metadata
    pub data: NodeData,
}

impl Node {
    /// Returns a Node object
    ///
    /// # Arguments
    ///
    /// * `data` - A NodeData representing the metadata of the Node
    pub fn new(data: NodeData) -> Self {
        Self {
            parent: 0,
            children: vec![],
            data,
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub path_map: HashMap<PathBuf, NodeIdx>,
}

pub fn root_path() -> PathBuf {
    return PathBuf::from("");
}

pub fn root_idx() -> NodeIdx {
    return 0;
}

impl Tree {
    pub fn new() -> Self {
        let mut new_tree = Self {
            nodes: Vec::new(),
            path_map: HashMap::<PathBuf, NodeIdx>::new(),
        };

        let mut root = Node::new(NodeData::new(root_path(), NodeType::Directory));
        root.parent = 0;
        new_tree.nodes.push(root);
        new_tree.path_map.insert(root_path(), 0);

        return new_tree;
    }

    pub fn from_fold_function(from_tree: &Tree, f: fn(&Node) -> PathBuf) -> Self {
        let from_paths = (&from_tree.nodes)
            .into_iter()
            .filter(|x| match x.data.node_type {
                NodeType::File => true,
                NodeType::Directory => false,
            })
            .map(|x| x.data.path.clone())
            .collect();

        let to_paths = (&from_tree.nodes)
            .into_iter()
            .filter(|x| match x.data.node_type {
                NodeType::File => true,
                NodeType::Directory => false,
            })
            .map(f)
            .collect();

        return Tree::from_fold(&from_tree, from_paths, to_paths);
    }

    pub fn from_fold(from_tree: &Tree, from_paths: Vec<PathBuf>, to_paths: Vec<PathBuf>) -> Self {
        let mut new_tree = Tree::new();

        for i in 0..from_paths.len() {
            // TODO deal with None
            let from_file = from_tree.get_node_by_path(&from_paths[i]).unwrap();
            let mut new_data = from_file.data.clone();
            new_data.path = to_paths[i].clone();
            new_tree.add_node(new_data);
        }

        return new_tree;
    }

    /// Returns a vector of file names. Used for testing atm.
    pub fn get_file_names(&self) -> Vec<String> {
        let mut file_names = Vec::<String>::new();

        for node in &self.nodes {
            if let NodeType::File = node.data.node_type {
                let name = node
                    .data
                    .original_path
                    .file_name()
                    .map(|os_str| os_str.to_string_lossy().to_string());
                name.map(|s| file_names.push(s));
            }
        }

        return file_names;
    }

    pub fn add_node(&mut self, child_data: NodeData) {
        let ancestors: Vec<&Path> = child_data.path.ancestors().collect();

        for ancestor in ancestors.iter().rev() {
            let ancestor_buf = PathBuf::from(ancestor);

            match self.path_map.get(&ancestor_buf) {
                None => {
                    let ancestor_parent = match ancestor.parent() {
                        Some(x) => PathBuf::from(x),
                        None => PathBuf::from(""),
                    };

                    // Unwrap should be fine since all parents are either root or
                    //	already created
                    let parent_idx = self.path_map.get(&ancestor_parent).unwrap();

                    if ancestor_buf != child_data.path {
                        self.add_child(
                            *parent_idx,
                            NodeData::new(ancestor_buf, NodeType::Directory),
                        );
                    } else {
                        self.add_child(*parent_idx, child_data.clone());
                    }
                }
                Some(_parent_idx) => (),
            }
        }
    }

    pub fn add_child(&mut self, parent_idx: NodeIdx, child_data: NodeData) -> NodeIdx {
        let mut child = Node::new(child_data);
        let child_idx = self.nodes.len();

        let parent = &mut self.nodes[parent_idx];
        parent.children.push(child_idx);

        child.parent = parent_idx;

        self.nodes.push(child);
        self.path_map
            .insert(self.nodes[child_idx].data.path.clone(), child_idx);

        return child_idx;
    }

    pub fn get_node_by_path(&self, path: &PathBuf) -> Option<&Node> {
        match self.path_map.get(path) {
            None => None,
            Some(node_idx) => return Some(&self.nodes[*node_idx]),
        }
    }

    // pub fn get_children(&self, parent_idx: NodeIdx) -> Children<'_> {
    //     Children {tree: &self, parent_idx, current: 0}
    // }

    pub fn get_children(&self, parent_idx: NodeIdx) -> impl Iterator<Item = usize> + '_ {
        return self.nodes[parent_idx].children.iter().map(|i| i.clone());
    }

    pub fn get_child(&self, parent_idx: NodeIdx, file_name: String) -> Option<&NodeIdx> {
        let parent = &self.nodes[parent_idx];
        let mut child_path = parent.data.path.clone();
        child_path.push(file_name);

        return self.path_map.get(&child_path);
    }

    pub fn leaves(&self) -> impl Iterator<Item = usize> + '_ {
        return self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_i, x)| x.children.len() == 0)
            .map(|(i, _x)| i);
    }

    pub fn layers(&self) -> Layers {
        return Layers {
            tree: self,
            current_v: vec![root_idx()],
            next_v: vec![],
        };
    }

    pub fn data_iter<'a>(
        &'a self,
        i: impl Iterator<Item = usize> + 'a,
    ) -> impl Iterator<Item = &'a NodeData> + '_ {
        return i.map(|j| &self.nodes[j].data);
    }
}

// 'a is life-time shit
// pub struct Children<'a> {
// 	tree: &'a Tree,
// 	parent_idx: NodeIdx,
// 	current: usize
// }
//
// impl Iterator for Children<'_> {
// 	type Item = NodeIdx;
// 	fn next(&mut self) -> Option<Self::Item> {
// 		let parent_node = &self.tree.nodes[self.parent_idx];
//
// 		if self.current >= parent_node.children.len() {
// 			return None;
// 		}
//
// 		let curr_idx = parent_node.children[self.current];
// 		self.current += 1;
//
// 		return Some(curr_idx);
// 	}
// }

// // 'a is life-time shit
pub struct Layers<'a> {
    tree: &'a Tree,
    current_v: Vec<usize>,
    next_v: Vec<usize>,
}

impl Iterator for Layers<'_> {
    type Item = NodeIdx;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_v.len() == 0 {
            if self.next_v.len() == 0 {
                return None;
            } else {
                std::mem::swap(&mut self.next_v, &mut self.current_v);
            }
        }

        let c_idx = self.current_v.pop().unwrap();
        for c in &self.tree.nodes[c_idx].children {
            self.next_v.push(c.clone());
        }

        return Some(c_idx);
    }
}
