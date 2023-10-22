use crate::from_ast::{to_rename, Rename, RenamePart};
use std::path::PathBuf;
use std::collections::HashMap;

// Testing rename parser

fn add_simple_variable(d: &mut HashMap<String, usize>, name: &str) {
    d.insert(name.to_string(), d.len());
}

fn text(t: &str) -> RenamePart {
    return RenamePart::Text(t.to_string());
}

fn idx(i: usize) -> RenamePart {
    return RenamePart::Idx(i);
}

#[test]
fn small_shuffle_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{b}/{a}/{f}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(1)],
                                    vec![idx(0)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn small_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{a}_{b}/{f}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0), text("_"), idx(1)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn small_shuffle_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{b}_{a}/{f}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(1), text("_"), idx(0)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn small_full_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{a}_{b}_{f}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0), text("_"), idx(1), text("_"), idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn long_shuffle_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "students");
    add_simple_variable(&mut vdm, "classes");
    add_simple_variable(&mut vdm, "subjects");
    let out = to_rename(&mut vdm, &"{classes}/{students}/{subjects}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(1)],
                                    vec![idx(0)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn long_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "students");
    add_simple_variable(&mut vdm, "classes");
    add_simple_variable(&mut vdm, "subjects");
    let out = to_rename(&mut vdm, &"{students} in {classes}/{subjects}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0), text(" in "), idx(1)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn long_shuffle_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "students");
    add_simple_variable(&mut vdm, "classes");
    add_simple_variable(&mut vdm, "subjects");
    let out = to_rename(&mut vdm, &"{classes} of {students}/{subjects}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(1), text(" of "), idx(0)],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn long_full_flatten_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "students");
    add_simple_variable(&mut vdm, "classes");
    add_simple_variable(&mut vdm, "subjects");
    let out = to_rename(&mut vdm, &"{students} in {classes} in {subjects}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0), text(" in "), idx(1), text(" in "), idx(2)]
                                    ]
                                }
                        )
            );
}

use std::any::TypeId;

#[test]
#[should_panic(expected = "Error in parsing, full part ended in wrong state")]
fn bad_layer_part_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{a}/{b/{f}".to_string());
    println!("out: {:?}", out);
}

#[test]
fn trail_text_part_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &"{a}/{b}_hi/{f}".to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0)],
                                    vec![idx(1), text("_hi")],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}

#[test]
fn simple_text_cancel_test() {
    let mut vdm: HashMap<String, usize> = HashMap::new();
    add_simple_variable(&mut vdm, "a");
    add_simple_variable(&mut vdm, "b");
    add_simple_variable(&mut vdm, "f");
    let out = to_rename(&mut vdm, &r#"{a}\\/{b}_\{h\\i}/{f}"#.to_string());
    println!("out: {:?}", out);
    assert!(out == Some(Rename{relative: false,
                                parts: vec![
                                    vec![idx(0), text(r"\")],
                                    vec![idx(1), text(r"_{h\i}")],
                                    vec![idx(2)]
                                    ]
                                }
                        )
            );
}
