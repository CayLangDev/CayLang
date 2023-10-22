use crate::validate::{get_tree_prototype, get_node_prototype, ValidationError, validate_tree};
use crate::defn_map::{DefnMap, new_defn_map, LookupError};
use caylang_parser::test_helpers::{to_ident,
                                   to_super_ident,
                                   simple_tree_prototype,
                                   simple_node_prototype,
    compl_tree_prototype
};
use caylang_parser::ast::{Prototype, TreePrototype, NodePrototype,
                    NodeType, SuperIdent, ParamIdent, Ident, StructurePair};

use caylang_io::test_helpers::tree_builder::{file, dir};


fn make_tree_prototype(d: &mut DefnMap, i: &str, layers: Vec<&str>) {
    let u_layers = &layers[..layers.len()-1];
    let edges = &layers[layers.len()-1..];
    for l in u_layers.iter() {
        d.add_object(to_ident(l), Prototype::NodePrototype(simple_node_prototype(l, NodeType::Dir)));
    }
    for e in edges.iter() {
        d.add_object(to_ident(e), Prototype::NodePrototype(simple_node_prototype(e, NodeType::File)));
    }

    d.add_object(to_ident(i), Prototype::TreePrototype(compl_tree_prototype(i, u_layers, edges)));
}

#[test]
fn test_make_tree_prototype() {
    let mut d = new_defn_map();
    make_tree_prototype(&mut d, "t", vec!["a","b","f"]);

    fn dir_p(s: &str) -> Prototype {
        return Prototype::NodePrototype(simple_node_prototype(s, NodeType::Dir));
    }

    fn file_p(s: &str) -> Prototype {
        return Prototype::NodePrototype(simple_node_prototype(s, NodeType::File));
    }

    fn res(p: &Prototype) -> Result<&Prototype, LookupError> {
        return Ok(p);
    }

    let a = dir_p("a");
    let b = dir_p("b");
    let f = file_p("f");

    assert_eq!(d.get_object(&to_super_ident("a")), res(&a).cloned());
    assert_eq!(d.get_object(&to_super_ident("b")), res(&b).cloned());
    assert_eq!(d.get_object(&to_super_ident("f")), res(&f).cloned());
}

#[test]
fn test_make_tree_prototype_structure() {
    let mut d = new_defn_map();
    make_tree_prototype(&mut d, "t", vec!["a","b","f"]);

    let p = Prototype::TreePrototype(TreePrototype {
        regex: "t".to_string(),
        layers: vec![StructurePair(Ident::Ignored, to_super_ident("a")),
                     StructurePair(Ident::Ignored, to_super_ident("b"))],
        edges: vec![StructurePair(Ident::Ignored, to_super_ident("f"))]
    });

    fn res(p: &Prototype) -> Result<&Prototype, LookupError> {
        return Ok(p);
    }

    assert_eq!(d.get_object(&to_super_ident("t")), res(&p).cloned());
}

#[test]
fn test_get_tree_prototype() {
    let mut d = new_defn_map();
    let p = simple_tree_prototype("a");
    _ = d.add_object(
        to_ident("a"),
        Prototype::TreePrototype(p)
    );

    let p = simple_tree_prototype("a");
    assert_eq!(get_tree_prototype(&d, &to_super_ident("a")), (Ok(&p) as Result<&TreePrototype, ValidationError>).cloned());

    let b = to_super_ident("b");
    let res = get_tree_prototype(&d, &b);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(b)));

    let i = to_super_ident("_");
    let res = get_tree_prototype(&d, &i);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(i)));
}

#[test]
fn test_get_node_prototype() {
    let mut d = new_defn_map();
    let p = simple_node_prototype("a", NodeType::File);
    _ = d.add_object(
        to_ident("a"),
        Prototype::NodePrototype(p)
    );

    let p = simple_node_prototype("a", NodeType::File);
    assert_eq!(get_node_prototype(&d, &to_super_ident("a")),
               (Ok(&p) as Result<&NodePrototype, ValidationError>).cloned());

    let b = to_super_ident("b");
    let res = get_node_prototype(&d, &b);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(b)));

    let i = to_super_ident("_");
    let res = get_node_prototype(&d, &i);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(i)));
}

#[test]
fn test_false_negative() {
    let tree = dir("a", vec![dir("b", vec![file("f")])]).build_tree();
    tree.print();
    let mut d = new_defn_map();
    d.add_defaults();

    make_tree_prototype(&mut d, "t", vec!["a","b","f"]);

    let res = validate_tree(&d, &tree, &to_super_ident("t"));
    println!("out: {:?}", res);
    assert!(res.is_ok());

}

#[test]
fn test_simple_false_negative() {
    let tree = dir("a", vec![file("f")]).build_tree();
    tree.print();
    let mut d = new_defn_map();
    d.add_defaults();

    make_tree_prototype(&mut d, "t", vec!["a","f"]);

    let res = validate_tree(&d, &tree, &to_super_ident("t"));
    println!("out: {:?}", res);
    assert!(res.is_ok());

}
