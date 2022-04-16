//! Represents an expression in the parse tree.
use std::fmt;
use std::fmt::Formatter;

/// Represents an expression in the parse tree.
/// Grammar rule: term (op term)*
#[derive(Debug)]
pub struct ExpressionNode {
    pub elements: Vec<ExpressionElement>,
}

impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut elements = String::new();
        if self.elements.is_empty() {
            elements.push_str("<expression></expression>");
        } else {
            elements.push_str("<expression>\n");
            for element in self.elements.iter() {
                elements.push_str(&format!("{}\n", element));
            }
            elements.push_str("</expression>");
        }
        write!(f, "{}", elements)
    }
}

/// Represents an expression element. An expression element is either a term or an operator.
#[derive(Debug)]
pub enum ExpressionElement {
    Term(TermNode),
    Operator(String),
}

impl fmt::Display for ExpressionElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionElement::Term(term) => {
                write!(f, "{}", term)
            }
            ExpressionElement::Operator(op) => {
                // xml special characters
                let op_str = match op.as_str() {
                    "<" => "&lt;",
                    ">" => "&gt;",
                    "&" => "&amp;",
                    "\"" => "&quot",
                    "'" => "&apos;",
                    _ => op,
                };
                write!(f, "<symbol> {} </symbol>", op_str)
            }
        }
    }
}

/// Represents a term in the parse tree. A term node can contain
/// arbitrary many terms.
#[derive(Debug)]
pub struct TermNode {
    pub elements: Vec<TermElement>,
}

impl fmt::Display for TermNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.elements.is_empty() {
            write!(f, "<term>\n</term>")
        } else {
            let mut term_elems = String::new();
            for i in 0..self.elements.len() - 1 {
                term_elems.push_str(&format!("{}\n", self.elements[i]));
            }
            term_elems.push_str(&format!("{}", self.elements[self.elements.len() - 1]));
            write!(f, "<term>\n{}\n</term>", term_elems)
        }
    }
}

/// Represents a term in the parse tree.
/// Grammar rule: integerConstant | stringConstant | keywordConstant | varName |
/// varName `[` expression `]` | subroutineCall | `(` expression `)` | unaryOp term
#[derive(Debug)]
pub enum TermElement {
    Identifier(String),
    Symbol(String),
    IntegerConstant(u32),
    KeywordConstant(String),
    StringConstant(String),
    Expression(ExpressionNode),
    Term(TermNode),
    SubroutineCall(SubroutineCallNode),
}

impl fmt::Display for TermElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TermElement::Identifier(s) => {
                write!(f, "<identifier> {} </identifier>", s)
            }
            TermElement::Symbol(s) => {
                write!(f, "<symbol> {} </symbol>", s)
            }
            TermElement::IntegerConstant(n) => {
                write!(f, "<integerConstant> {} </integerConstant>", n)
            }
            TermElement::StringConstant(n) => {
                write!(f, "<stringConstant> {} </stringConstant>", n)
            }
            TermElement::KeywordConstant(keyword) => {
                write!(f, "<keyword> {} </keyword>", keyword)
            }
            TermElement::Expression(expression) => {
                write!(f, "{}", expression)
            }
            TermElement::Term(term) => {
                write!(f, "{}", term)
            }
            TermElement::SubroutineCall(subroutine_call) => {
                write!(f, "{}", subroutine_call)
            }
        }
    }
}

/// Represents a term in the parse tree.
/// Grammar rule: integerConstant | stringConstant | keywordConstant | varName |
/// varName `[` expression `]` | subroutineCall | `(` expression `)` | unaryOp term
#[derive(Debug)]
pub struct SubroutineCallNode {
    pub subroutine_name: String,
    pub expression_list: Vec<ExpressionNode>,
    pub caller: Option<String>, // className or varName
    pub semicolon: bool,
}

impl fmt::Display for SubroutineCallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let caller = match &self.caller {
            Some(name) => format!(
                "<identifier> {} </identifier>\n<symbol> . </symbol>\n",
                name
            ),
            None => "".to_string(),
        };
        let sub_name = format!("<identifier> {} </identifier>", self.subroutine_name);
        let mut expr_list = String::new();
        if self.expression_list.is_empty() {
            expr_list.push_str(
                "<symbol> ( </symbol>\n<expressionList>\n</expressionList>\n<symbol> ) </symbol>",
            );
        } else {
            expr_list.push_str("<symbol> ( </symbol>\n<expressionList>\n");
            for i in 0..self.expression_list.len() - 1 {
                expr_list.push_str(&format!(
                    "{}\n<symbol> , </symbol>\n",
                    self.expression_list[i]
                ));
            }
            expr_list.push_str(&format!(
                "{}\n",
                self.expression_list[self.expression_list.len() - 1]
            ));
            expr_list.push_str("</expressionList>\n<symbol> ) </symbol>");
        }
        let semicolon = if self.semicolon { "\n<symbol> ; </symbol>" } else { "" };
        write!(
            f,
            "{}{}\n{}{}",
            caller, sub_name, expr_list, semicolon
        )
    }
}

