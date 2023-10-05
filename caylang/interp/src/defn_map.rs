use crate::from_ast::{intoInterpObject, toInterpObject};
use caylang_parser::ast::{Expr, Ident, NodePrototype, NodeType, Prototype};
use crate::from_ast::{InterpObject, OperationApplication};
use std::collections::HashMap;

// map from variable names to values, that handles the ignore identifier cleanly
// pub type DefnMap = ;


pub enum LookupError {
    ignore_lookup,
    variable_not_found
}

pub enum AddStatus {
    old_replaced,
    ignored
}

pub type Object = Prototype;

#[derive(Debug)]
pub struct DefnMap {
    data: HashMap<String, Object>
}

pub fn new_defn_map() -> DefnMap {
    return DefnMap { data: HashMap::new() };
}

impl DefnMap {
    // add default values
    pub fn add_defaults(&mut self) {
        let defaults = vec![("Directory", NodePrototype {regex: r".*".to_string(), node_type: NodeType::Dir}),
                            ("File", NodePrototype {regex: r".*".to_string(), node_type: NodeType::File})];
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
                    None => Err(LookupError::variable_not_found)
                }
            }
            Ident::Ignored => Err(LookupError::ignore_lookup)
        }
    }

    pub fn add_object(&mut self, name: Ident, obj: Object) -> Result<(), AddStatus> {
        match name {
            Ident::Variable(s) => {
                match self.data.insert(s, obj) {
                    Some(val) => Err(AddStatus::old_replaced),
                    None => Ok(()) // added a new object with a new name
                }
            }
            Ident::Ignored => Err(AddStatus::ignored)
        }
    }
}
