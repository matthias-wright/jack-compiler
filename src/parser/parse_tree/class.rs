//! Represents a class in the parse tree.
use std::fmt;
use std::fmt::Formatter;

use super::var::{VarType, VarKind};
use super::subroutine::SubroutineNode;

/// Represents a class in the parse tree.
/// Grammar rule: `class` className `{` classVarDec* subroutineDec* `}`
#[derive(Debug)]
pub struct ClassNode {
    pub name: String,
    pub variables: Vec<ClassVarNode>,
    pub subroutines: Vec<SubroutineNode>,
}

impl fmt::Display for ClassNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let class = "<keyword> class </keyword>";
        let name = format!("<identifier> {} </identifier>", self.name);
        let open_bracket = "<symbol> { </symbol>";
        let closed_bracket = "<symbol> } </symbol>";

        let mut variables = String::new();
        for variable in self.variables.iter() {
            variables.push_str(&format!("{}\n", variable));
        }
        let mut subroutines = String::new();
        for subroutine in self.subroutines.iter() {
            subroutines.push_str(&format!("{}", subroutine));
        }
        write!(
            f,
            "<class>\n\
            {}\n\
            {}\n\
            {}\n\
            {}\
            {}\
            {}\n\
            </class>\n",
            class, name, open_bracket, variables, subroutines, closed_bracket
        )
    }
}

/// Represents the variables of class.
/// Grammar rule: (`static` | `field`) type varName (`,` varName)* `;`
#[derive(Debug)]
pub struct ClassVarNode {
    pub names: Vec<String>,
    pub var_kind: VarKind,
    pub var_type: VarType,
}

impl fmt::Display for ClassVarNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut names = String::new();
        if !self.names.is_empty() {
            for i in 0..self.names.len() - 1 {
                names.push_str(&format!(
                    "<identifier> {} </identifier>\n\
                     <symbol> , </symbol>\n",
                    self.names[i]
                ));
            }
            names.push_str(&format!(
                "<identifier> {} </identifier>\n\
                 <symbol> ; </symbol>",
                self.names[self.names.len() - 1]
            ));
        }
        let var_type = match &self.var_type {
            VarType::ClassName(name) => format!("<identifier> {} </identifier>", name),
            _ => format!("<keyword> {} </keyword>", self.var_type),
        };
        write!(
            f,
            "<classVarDec>\n\
            <keyword> {} </keyword>\n\
            {}\n\
            {}\n\
            </classVarDec>",
            self.var_kind, var_type, names
        )
    }
}
