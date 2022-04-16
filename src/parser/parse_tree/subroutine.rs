//! Represents a subroutine in the parse tree.
use std::fmt;
use std::fmt::Formatter;

use super::statement::Statement;
use super::var::{VarNode, VarType};

/// Represents a subroutine in the parse tree.
/// Grammar rule: (`constructor` | `function` | `method`) (`void` | type)
/// subroutineName `(` parameterList `)` subroutineBody
#[derive(Debug)]
pub struct SubroutineNode {
    pub name: String,
    pub subroutine_type: SubroutineType,
    pub return_type: Option<VarType>,
    pub parameter_list: ParameterListNode,
    pub body: SubroutineBodyNode,
}

impl fmt::Display for SubroutineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let return_type = match &self.return_type {
            Some(return_type) => match return_type {
                VarType::ClassName(class_name) => {
                    format!("<identifier> {} </identifier>", class_name)
                }
                _ => format!("<keyword> {} </keyword>", return_type),
            },
            None => "<keyword> void </keyword>".to_string(),
        };
        let subroutine_type = format!("<keyword> {} </keyword>", self.subroutine_type);
        let name = format!("<identifier> {} </identifier>", self.name);
        write!(
            f,
            "<subroutineDec>\n\
            {}\n\
            {}\n\
            {}\n\
            <symbol> ( </symbol>\n\
            {}\n\
            <symbol> ) </symbol>\n\
            {}\
            </subroutineDec>\n",
            subroutine_type, return_type, name, self.parameter_list, self.body
        )
    }
}

/// Represents a subroutine body in the parse tree.
/// Grammar rule: `{` varDec* statement* `}`
#[derive(Debug)]
pub struct SubroutineBodyNode {
    pub variables: Vec<VarNode>,
    pub statements: Vec<Statement>,
}

impl fmt::Display for SubroutineBodyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vars = String::new();
        for variable in self.variables.iter() {
            vars.push_str(&format!("{}\n", variable));
        }
        let mut statements = String::from("<statements>\n");
        for statement in self.statements.iter() {
            statements.push_str(&format!("{}", statement));
        }
        statements.push_str("</statements>\n");
        write!(
            f,
            "<subroutineBody>\n\
            <symbol> {{ </symbol>\n\
            {}\
            {}\
            <symbol> }} </symbol>\n\
            </subroutineBody>\n",
            vars, statements
        )
    }
}

/// Represents a parameter list in the parse tree.
/// Grammar rule: ((type varName)(`,` type varName)*)?
#[derive(Debug)]
pub struct ParameterListNode {
    pub parameters: Vec<ParameterNode>,
}

impl fmt::Display for ParameterListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.parameters.is_empty() {
            write!(f, "<parameterList>\n</parameterList>")
        } else {
            let mut parameters = String::new();
            for i in 0..self.parameters.len() - 1 {
                parameters.push_str(&format!("{}\n<symbol> , </symbol>\n", self.parameters[i]));
            }
            parameters.push_str(&format!("{}", self.parameters[self.parameters.len() - 1]));
            write!(f, "<parameterList>\n{}\n</parameterList>", parameters)
        }
    }
}

/// Represents a parameter in the parse tree.
#[derive(Debug)]
pub struct ParameterNode {
    pub name: String,
    pub var_type: VarType,
}

impl fmt::Display for ParameterNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<keyword> {} </keyword>\n\
            <identifier> {} </identifier>",
            self.var_type, self.name
        )
    }
}

/// The subroutine type: `constructor`, `function`, or `method`.
#[derive(Clone, Debug)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

impl fmt::Display for SubroutineType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            SubroutineType::Constructor => "constructor",
            SubroutineType::Function => "function",
            SubroutineType::Method => "method",
        };
        write!(f, "{}", s)
    }
}

impl SubroutineType {
    pub fn get(subroutine_type: &str) -> SubroutineType {
        match subroutine_type {
            "constructor" => SubroutineType::Constructor,
            "function" => SubroutineType::Function,
            "method" => SubroutineType::Method,
            _ => {
                eprintln!("Parse Error: Unknown subroutine: {}", subroutine_type);
                std::process::exit(1);
            }
        }
    }
}
