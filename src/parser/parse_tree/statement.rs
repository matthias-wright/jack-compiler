//! Represents the statements in the parse tree.
use std::fmt;
use std::fmt::Formatter;

use super::expression::{ExpressionNode, SubroutineCallNode};

/// Represents a statement in the parse tree.
#[derive(Debug)]
pub enum Statement {
    Let(LetStatementNode),
    If(IfStatementNode),
    While(WhileStatementNode),
    Do(DoStatementNode),
    Return(ReturnStatementNode),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(statement) => {
                write!(f, "{}", statement)
            }
            Statement::If(statement) => {
                write!(f, "{}", statement)
            }
            Statement::While(statement) => {
                write!(f, "{}", statement)
            }
            Statement::Do(statement) => {
                write!(f, "{}", statement)
            }
            Statement::Return(statement) => {
                write!(f, "{}", statement)
            }
        }
    }
}

/// Represents a while statement in the parse tree.
/// Grammar rule: `while` `(` expression `)` `{` statement* `}`
#[derive(Debug)]
pub struct WhileStatementNode {
    pub condition: ExpressionNode,
    pub block: Vec<Statement>,
}

impl fmt::Display for WhileStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let keyword = "<keyword> while </keyword>";
        let open_bracket = "<symbol> ( </symbol>";
        let closing_bracket = "<symbol> ) </symbol>";
        let mut block = String::from("<symbol> { </symbol>\n<statements>\n");
        for statement in self.block.iter() {
            block.push_str(&format!("{}", statement));
        }
        block.push_str("</statements>\n<symbol> } </symbol>\n");
        write!(
            f,
            "<whileStatement>\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\
            </whileStatement>\n",
            keyword, open_bracket, self.condition, closing_bracket, block
        )
    }
}

/// Represents an if statement in the parse tree.
/// Grammar rule: `if` `(` expression `)` `{` statement* `}`
/// (`else` `{` statement* `}`)?
#[derive(Debug)]
pub struct IfStatementNode {
    pub condition: ExpressionNode,
    pub if_block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>,
}

impl fmt::Display for IfStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let keyword = "<keyword> if </keyword>";
        let open_bracket = "<symbol> ( </symbol>";
        let closing_bracket = "<symbol> ) </symbol>";
        let mut if_block = String::from("<symbol> { </symbol>\n<statements>\n");
        for statement in self.if_block.iter() {
            if_block.push_str(&format!("{}", statement));
        }
        if_block.push_str("</statements>\n<symbol> } </symbol>\n");
        let else_block = match &self.else_block {
            Some(statements) => {
                let mut else_block =
                    String::from("<keyword> else </keyword>\n<symbol> { </symbol>\n<statements>\n");
                for statement in statements.iter() {
                    else_block.push_str(&format!("{}", statement));
                }
                else_block.push_str("</statements>\n<symbol> } </symbol>\n");
                else_block
            }
            None => "".to_string(),
        };
        write!(
            f,
            "<ifStatement>\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\
            {}\
            </ifStatement>\n",
            keyword, open_bracket, self.condition, closing_bracket, if_block, else_block
        )
    }
}

/// Represents a return statement in the parse tree.
/// Grammar rule: `return` expression? `;`
#[derive(Debug)]
pub struct ReturnStatementNode {
    pub expression: Option<ExpressionNode>,
}

impl fmt::Display for ReturnStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.expression {
            Some(expr) => {
                write!(f, "<returnStatement>\n<keyword> return </keyword>\n{}\n<symbol> ; </symbol>\n</returnStatement>\n", expr)
            }
            None => {
                write!(f, "<returnStatement>\n<keyword> return </keyword>\n<symbol> ; </symbol>\n</returnStatement>\n")
            }
        }
    }
}

/// Represents a let statement in the parse tree.
/// Grammar rule: `let` varName (`[` expression `]`)? `=` expression `;`
#[derive(Debug)]
pub struct LetStatementNode {
    pub var_name: String,
    pub lhs_expression: Option<ExpressionNode>,
    pub rhs_expression: ExpressionNode,
}

impl fmt::Display for LetStatementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keyword = "<keyword> let </keyword>";
        let identifier = format!("<identifier> {} </identifier>", self.var_name);
        let lhs_expression = match &self.lhs_expression {
            Some(expression) => {
                format!(
                    "<symbol> [ </symbol>\n{}\n<symbol> ] </symbol>\n",
                    expression
                )
            }
            None => "".to_string(),
        };
        let equal_sign = "<symbol> = </symbol>";
        let semicolon = "<symbol> ; </symbol>";
        write!(
            f,
            "<letStatement>\n\
            {}\n\
            {}\n\
            {}\
            {}\n\
            {}\n\
            {}\n\
            </letStatement>\n",
            keyword, identifier, lhs_expression, equal_sign, self.rhs_expression, semicolon
        )
    }
}

/// Represents a do statement in the parse tree.
/// Grammar rule: `do` subroutineCall `;`
#[derive(Debug)]
pub struct DoStatementNode {
    pub subroutine_call: SubroutineCallNode,
}

impl fmt::Display for DoStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<doStatement>\n\
            <keyword> do </keyword>\n\
            {}\n\
            </doStatement>\n",
            self.subroutine_call
        )
    }
}
