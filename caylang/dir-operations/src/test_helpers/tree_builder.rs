use crate::tree::{Tree, NodeIdx, NodeData, NodeType, root_path, root_idx};
use std::fs::{create_dir, write};
use std::{fs::File, path::PathBuf};

#[derive(Clone)]
pub struct FileNode {
    pub name: String,
    pub children: Vec<FileNode>,
}

pub fn dir(name: &str, children: Vec<FileNode>) -> FileNode {
    return FileNode::dir(name, children);
}

pub fn file(name: &str) -> FileNode {
    return FileNode::file(name);
}

impl FileNode {
    pub fn dir(name: &str, children: Vec<FileNode>) -> Self {
        return FileNode {
            name: name.to_string(),
            children: children,
        };
    }
    pub fn file(name: &str) -> Self {
        return FileNode {
            name: name.to_string(),
            children: vec![],
        };
    }

    pub fn apply(&self, base_dir: &PathBuf) -> Result<(), std::io::Error> {
        let node_path = base_dir.join(&self.name);

        if !self.children.is_empty() {
            create_dir(&node_path)?;
        } else {
            write(&node_path, format!("{}", &self.name))?;
        }

        for child in &self.children {
            child.apply(&node_path)?;
        }

        Ok(())
    }

    pub fn build_tree(self: FileNode) -> Tree {
        let mut res = Tree::new();

        pub fn build_tree_dfs(
            res: &mut Tree,
            parent_path: PathBuf,
            parent_idx: NodeIdx,
            current_node: FileNode
        ) {
            let cur_path = parent_path.join(current_node.name);
            let cur_type = if current_node.children.len() == 0 { NodeType::File }
                        else { NodeType::Directory};
            let cur_idx = res.add_child(parent_idx,
                                        NodeData::new(cur_path.clone(),
                                        cur_type));
            for c in current_node.children {
                build_tree_dfs(res, cur_path.clone(), cur_idx, c);
            }
        }

        build_tree_dfs(&mut res, root_path(), root_idx(), self);

        return res;
    }
}
