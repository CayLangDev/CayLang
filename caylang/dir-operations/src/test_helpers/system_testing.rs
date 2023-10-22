use crate::test_helpers::tree_builder::FileNode;
use crate::tree::{NodeData, NodeIdx, NodeType, Tree};
use crate::filesys::{load_full_tree, write_full_tree};
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};
extern crate fs_extra;
use fs_extra::dir;
use fs_extra::dir::copy;
use dir_diff::is_different;
use tempdir::TempDir;
use jwalk::WalkDir;

fn dfs(tree: &Tree, current_idx: NodeIdx) {
    let root = &tree.nodes[current_idx];
    println!("{:?}", root.data.path);
    for child in tree.get_children(current_idx) {
        dfs(&tree, child);
    }
}

pub fn system_test(
    cay_code: &str,
    in_structure: &FileNode,
    out_structure: &FileNode,
) -> Result<(), std::io::Error> {



    // std::env::set_current_dir();
    // let cd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // println!("cd: {:?}", cd);
    // let p = cd.parent().unwrap();
    // println!("par: {:?}", p);
    // let p = p.parent().unwrap();
    // println!("parpar: {:?}", p);
    // std::env::set_current_dir(p);
    // std::env::set_current_dir(std::env::current_dir().unwrap().parent().unwrap());
    // println!("cd1: {:?}", std::env::current_dir());

    let tmp_dir = TempDir::new("test").unwrap();
    let root_path = PathBuf::from(tmp_dir.path());

    let mut tmp_in = root_path.clone();
    tmp_in.push("testbed");
    tmp_in.push("test_root");

    let mut tmp_out = root_path.clone();
    tmp_out.push("expected");
    tmp_out.push("test_root");

    let mut tmp_cay = root_path.clone();
    tmp_cay.push("test.cay");

    fs::create_dir_all(&tmp_in)?;
    fs::create_dir_all(&tmp_out)?;
    println!("cd2: {:?}", std::env::current_dir());

    in_structure.apply(&tmp_in)?;
    out_structure.apply(&tmp_out)?;

     println!("cd3: {:?}", std::env::current_dir());

    let c = cay_code.replace("<PATH>", tmp_in.to_str().unwrap());
    // println!("code: {:?}", c);

    fs::write(
        &tmp_cay,
        &cay_code.replace("<PATH>", tmp_in.to_str().unwrap()),
    )?;

    println!("cd4: {:?}", std::env::current_dir());
    // Call main cay exec on the file
    let mut cmd = Command::new("cargo");
    let _ = cmd
        .arg("run")
        .arg("build")
        .arg("-v")
        .arg("-r")
        .arg(tmp_cay.to_str().unwrap());

    // Run cmd
    println!("Output: {:?}", cmd.output());
    // match cmd.status() {
    //     Ok(status) => status,
    //     Err(e) => {
    //         eprintln!("Failed to execute command: {}", e);
    //         exit(1);
    //     }
    // };
    println!("Done run");


    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();

            if let Ok(contents) = fs::read_to_string(&file_path) {
                println!("File: {:?}", file_path);
                println!("Content: \n{}\n", contents);
            } else {
                eprintln!("Failed to read file: {:?}", file_path);
            }
        }
    }

    dfs(&load_full_tree(&root_path), 0);
    load_full_tree(&tmp_in).print();
    load_full_tree(&tmp_out).print();

    assert!(!is_different(tmp_in.to_str().unwrap(), tmp_out.to_str().unwrap()).unwrap());
    return Ok(());
}

pub fn run_test(path: &str, f: fn(&Tree) -> Tree) {
    let test_path = PathBuf::from(path);
    let mut in_path = test_path.clone();
    in_path.push("in");
    let mut out_path = test_path.clone();
    out_path.push("out");

    let tmp_dir = TempDir::new("test").unwrap();
    let root_path = PathBuf::from(tmp_dir.path());

    let options = dir::CopyOptions::new();
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
    let _ = copy(&in_path, &root_path, &options);
    let mut tmp_in = root_path.clone();
    tmp_in.push("in");
    let mut tmp_out = root_path.clone();
    tmp_out.push("out");
    let _ = fs::rename(&tmp_in, &tmp_out);

    let tree = load_full_tree(&tmp_out);
    let new_tree = f(&tree);

    write_full_tree(&tmp_out, &tmp_out, &tree, &new_tree);
    println!("=Old tree=");
    dfs(&tree, 0);
    println!("=New tree=");
    dfs(&new_tree, 0);

    assert!(!is_different(out_path.to_str().unwrap(), tmp_out.to_str().unwrap()).unwrap());
}

