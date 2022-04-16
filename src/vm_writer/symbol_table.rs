use crate::parser::parse_tree::var::{VarKind, VarType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub var_type: VarType,
    pub var_kind: VarKind,
    pub index: u32,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub class_scope: HashMap<String, Symbol>,
    pub subroutine_scope: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            class_scope: HashMap::new(),
            subroutine_scope: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, var_type: &VarType, var_kind: &VarKind, index: u32) {
        let symbol = Symbol {
            name: name.clone(),
            var_type: var_type.clone(),
            var_kind: var_kind.clone(),
            index,
        };

        match var_kind {
            VarKind::Static | VarKind::Field => {
                self.class_scope.insert(name.clone(), symbol);
            }
            VarKind::Var | VarKind::Arg => {
                self.subroutine_scope.insert(name.clone(), symbol);
            }
        }
    }

    pub fn get_var_type(&self, name: &String) -> &VarType {
        if self.subroutine_scope.contains_key(name) {
            &self.subroutine_scope.get(name).unwrap().var_type
        } else {
            &self
                .class_scope
                .get(name)
                .expect(&format!("Unknown variable name: {}", name))
                .var_type
        }
    }

    pub fn get_var_kind(&self, name: &String) -> String {
        if self.subroutine_scope.contains_key(name) {
            String::from(&self.subroutine_scope.get(name).unwrap().var_kind)
        } else {
            String::from(
                &self
                    .class_scope
                    .get(name)
                    .expect(&format!("Unknown variable name: {}", name))
                    .var_kind,
            )
        }
    }

    pub fn get_var_index(&self, name: &String) -> u32 {
        if self.subroutine_scope.contains_key(name) {
            self.subroutine_scope.get(name).unwrap().index
        } else {
            self.class_scope
                .get(name)
                .expect(&format!("Unknown variable name: {}", name))
                .index
        }
    }

    pub fn contains(&self, name: &String) -> bool {
        self.class_scope.contains_key(name) || self.subroutine_scope.contains_key(name)
    }

    pub fn clear_subroutine_scope(&mut self) {
        self.subroutine_scope.clear();
    }

}
