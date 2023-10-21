use std::fs::{create_dir, write};
use std::{fs::File, path::PathBuf};

#[derive(Clone)]
pub struct FileNode {
    pub name: String,
    pub children: Vec<FileNode>,
    pub content: Option<String>,
}

impl FileNode {
    pub fn dir(name: &str, children: Vec<FileNode>) -> Self {
        return FileNode {
            name: name.to_string(),
            children: children,
            content: None,
        };
    }
    pub fn file(name: &str) -> Self {
        return FileNode {
            name: name.to_string(),
            children: vec![],
            content: None,
        };
    }
    pub fn file_content(name: &str, content: &str) -> Self {
        return FileNode {
            name: name.to_string(),
            children: vec![],
            content: Some(content.to_string()),
        };
    }

    pub fn apply(&self, base_dir: &PathBuf) -> Result<(), std::io::Error> {
        let node_path = base_dir.join(&self.name);

        if !self.children.is_empty() {
            create_dir(&node_path)?;
        } else {
            match &self.content {
                None => write(&node_path, format!("{}", &self.name)),
                Some(content) => write(&node_path, content),
            }?
        }

        for child in &self.children {
            child.apply(&node_path)?;
        }

        Ok(())
    }
}
