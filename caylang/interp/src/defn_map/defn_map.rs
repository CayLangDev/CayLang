use crate::from_ast::{IntoInterpObject};
use caylang_parser::ast::{Expr, Ident, NodePrototype, NodeType, Prototype, TreePrototype, NodePrototype};
use crate::from_ast::{InterpObject, OperationApplication};
use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};

// map from variable names to values, that handles the ignore identifier cleanly
// pub type DefnMap = ;


pub enum LookupError {
    IgnoreLookup,
    VariableNotFound
}

pub enum TargetedLookupError {
    IgnoreLookup,
    VariableNotFound,
    IncorrectTypeObjectFound
}

pub enum AddStatus {
    OldReplaced,
    Ignored
}

pub type Object = Prototype;

#[derive(Debug)]
pub struct DefnMap {
    pub(super) data: HashMap<String, Object>
}

pub fn new_defn_map() -> DefnMap {
    return DefnMap { data: HashMap::new() };
}

impl DefnMap {
    // add default values
    pub fn add_defaults(&mut self) {
        let defaults = vec![("Directory", NodePrototype {regex: r".*".to_string(), node_type: NodeType::Dir}),
                            ("File",      NodePrototype {regex: r".*".to_string(), node_type: NodeType::File})];
        for (name, prototype) in defaults {
            _ = self.add_object(Ident::Variable(name.to_string()), Prototype::NodePrototype(prototype));
            // should probably throw if there's an overwrite here, means user redefined a default
        }
    }

    // loads all top level declarations, returns vector of remaining operations
    pub fn load_objects(&mut self, exprs: Expr) -> Vec<OperationApplication> {
        let mut operations = vec![];
        match exprs {
            Expr::ExprList(exprs) => {
                for expr in exprs {
                    let obj = expr.to_interp_object();
                    match obj {
                        Some(obj) => {
                            match obj {
                                InterpObject::Declaration(dec) => {
                                    _ = self.add_object(Ident::Variable(dec.name), dec.prototype);
                                }
                                InterpObject::Application(op) => {
                                    operations.push(op);
                                }
                            }
                        }
                        None => { }
                    }
                }
            }
            _ => panic!("trying to load expressions, but not an ExprList expression")
        }
        return operations;
    }

    pub fn get_object(&self, name: &Ident) -> Result<&Object, LookupError> {
        match name {
            Ident::Variable(s) => {
                match self.data.get(s) {
                    Some(val) => Ok(val),
                    None => Err(LookupError::VariableNotFound)
                }
            }
            Ident::Ignored => Err(LookupError::IgnoreLookup)
        }
    }

    pub fn get_tree_object(&self, name: &Ident) ->  Result<&TreePrototype, TargetedLookupError> {
        match self.get_object(name) {
            Ok(r) => match r {
                 Prototype::TreePrototype(p) => Ok(p),
                 _ => Err(TargetedLookupError::IncorrectTypeObjectFound)
            }
            Err(e) => Err(match e {
                LookupError::IgnoreLookup => TargetedLookupError::IgnoreLookup,
                LookupError::VariableNotFound => TargetedLookupError::VariableNotFound
            })
        }
    }

    pub fn get_node_object(&self, name: &Ident) ->  Result<&NodePrototype, TargetedLookupError> {
        match self.get_object(name) {
            Ok(r) => match r {
                 Prototype::TreePrototype(p) => Ok(p),
                 _ => Err(TargetedLookupError::IncorrectTypeObjectFound)
            }
            Err(e) => Err(match e {
                LookupError::IgnoreLookup => TargetedLookupError::IgnoreLookup,
                LookupError::VariableNotFound => TargetedLookupError::VariableNotFound
            })
        }
    }



    pub fn add_object(&mut self, name: Ident, obj: Object) -> Result<(), AddStatus> {
        match name {
            Ident::Variable(s) => {
                match self.data.insert(s, obj) {
                    Some(_val) => Err(AddStatus::OldReplaced),
                    None => Ok(()) // added a new object with a new name
                }
            }
            Ident::Ignored => Err(AddStatus::Ignored)
        }
    }
}
