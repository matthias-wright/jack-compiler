//! Represents the variables in the parse tree.
use std::fmt;
use std::fmt::Formatter;
use std::convert;

/// Represents a variable in the parse tree.
#[derive(Debug)]
pub struct VarNode {
    pub var_names: Vec<String>,
    pub var_kind: VarKind,
    pub var_type: VarType,
    pub class_var: bool,
}

impl fmt::Display for VarNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let class_var = if self.class_var {
            "classVarDec"
        } else {
            "varDec"
        };
        let var_type = match &self.var_type {
            VarType::ClassName(name) => format!("<identifier> {} </identifier>", name),
            _ => format!("<keyword> {} </keyword>", self.var_type),
        };

        let mut var_names = String::new();
        for i in 0..self.var_names.len() - 1 {
            var_names.push_str(&format!("<identifier> {} </identifier>\n<symbol> , </symbol>\n", self.var_names[i]));
        }
        var_names.push_str(&format!("<identifier> {} </identifier>\n", self.var_names[self.var_names.len() - 1]));
        write!(
            f,
            "<{}>\n\
            <keyword> {} </keyword>\n\
            {}\n\
            {}\
            <symbol> ; </symbol>\n\
            </{}>",
            class_var, self.var_kind, var_type, var_names, class_var
        )
    }
}

/// The variable kind: `static`, `field`, `argument`, `variable` (local).
#[derive(Clone, Debug)]
pub enum VarKind {
    Static,
    Field,
    Arg,
    Var,
}

impl fmt::Display for VarKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            VarKind::Static => "static",
            VarKind::Field => "field",
            VarKind::Var => "var",
            VarKind::Arg => "arg",
        };
        write!(f, "{}", s)
    }
}

impl convert::From<&VarKind> for String {
    fn from(var_kind: &VarKind) -> Self {
        match var_kind {
            VarKind::Static => "static".to_string(),
            VarKind::Field => "this".to_string(),
            VarKind::Var => "local".to_string(),
            VarKind::Arg => "argument".to_string(),
        }
    }
}

impl VarKind {
    pub fn get(var_kind: &str) -> VarKind {
        match var_kind {
            "static" => VarKind::Static,
            "field" => VarKind::Field,
            "var" => VarKind::Var,
            _ => {
                eprintln!("Parse Error: Unknown var kind: {}", var_kind);
                std::process::exit(1);
            }
        }
    }
}

/// The variable type: `int`, `char`, `boolean`, `className`.
#[derive(Clone, Debug)]
pub enum VarType {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            VarType::Int => "int",
            VarType::Char => "char",
            VarType::Boolean => "boolean",
            VarType::ClassName(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl VarType {
    pub fn get(data_type: &str) -> VarType {
        match data_type {
            "int" => VarType::Int,
            "char" => VarType::Char,
            "boolean" => VarType::Boolean,
            _ => VarType::ClassName(data_type.to_string()),
        }
    }
}
