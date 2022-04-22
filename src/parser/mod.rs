//! Reads the tokens and builds a parse tree.
use crate::tokenizer::tokens::{Constant, Token, TokenWrapper};

pub mod error;
use error::ParseError;

pub mod parse_tree;
use parse_tree::class::{ClassNode, ClassVarNode};
use parse_tree::expression::{
    ExpressionElement, ExpressionNode, SubroutineCallNode, TermElement, TermNode,
};
use parse_tree::statement::{
    DoStatementNode, IfStatementNode, LetStatementNode, ReturnStatementNode, Statement,
    WhileStatementNode,
};
use parse_tree::subroutine::{ParameterListNode, ParameterNode};
use parse_tree::subroutine::{SubroutineBodyNode, SubroutineNode, SubroutineType};
use parse_tree::var::{VarKind, VarNode, VarType};
use parse_tree::ParseTree;

/// Reads the tokens and builds a parse tree.
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    /// Takes in a Vec of [`TokenWrapper`](crate::tokenizer::tokens::TokenWrapper) values,
    /// and returns a [`ParseTree`](parse_tree::ParseTree).
    pub fn parse(&self, tokens: Vec<TokenWrapper>, filepath: &str) -> ParseTree {
        let filepath_wo_ending = match filepath.rfind(".jack") {
            Some(idx) => &filepath[..idx],
            None => &filepath,
        };
        let filename = match filepath_wo_ending.rfind("/") {
            Some(idx) => &filepath_wo_ending[idx + 1..],
            None => filepath_wo_ending,
        };

        let (class, _) = self.parse_class(&tokens[..], 0, filename);
        ParseTree { class_node: class }
    }

    //------------------------------
    // CLASSES
    //------------------------------

    fn parse_class(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
        filename: &str
    ) -> (ClassNode, usize) {
        match &tokens[index].token {
            Token::Keyword(class_keyword) if class_keyword == "class" => {
                if let Token::Identifier(name) = &tokens[index + 1].token {
                    if name != filename {
                        eprintln!(
                            "Parse Error in line {}: Class name \"{}\" must match file name \"{}\": {}",
                            tokens[index + 1].line.number,
                            name,
                            filename,
                            tokens[index + 1].line.content
                        );
                        std::process::exit(1);
                    }
                    match &tokens[index + 2].token {
                        Token::Symbol(symbol) if symbol == "{" => {
                            let mut index = index + 2;
                            index += 1;

                            // parse class vars
                            let mut class_vars = Vec::new();
                            loop {
                                match &tokens[index].token {
                                    Token::Keyword(keyword)
                                        if keyword == "static" || keyword == "field" =>
                                    {
                                        let (class_var, j) =
                                            self.parse_class_variable_declaration(tokens, index);
                                        index = j;
                                        class_vars.push(class_var);
                                    }
                                    _ => break,
                                }
                            }

                            // parse constructors, methods, functions
                            let mut subroutines = Vec::new();
                            loop {
                                match &tokens[index].token {
                                    Token::Keyword(keyword) if self.is_subroutine(keyword) => {
                                        let (subroutine, j) =
                                            self.parse_subroutine(tokens, index, keyword, false);
                                        index = j;
                                        subroutines.push(subroutine);
                                    }
                                    Token::Symbol(symbol) if symbol == "}" => {
                                        break;
                                    }
                                    _ => {
                                        eprintln!(
                                            "Parse Error in line {}: Only subroutines are allowed here: {}",
                                            tokens[index].line.number, tokens[index].line.content
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            (
                                ClassNode {
                                    name: name.to_string(),
                                    variables: class_vars,
                                    subroutines,
                                },
                                index + 1,
                            )
                        }
                        _ => {
                            eprintln!(
                                "Parse Error in line {}: Missing curly bracket for class definition: {}",
                                tokens[index + 2].line.number,
                                tokens[index + 2].line.content
                            );
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!(
                        "Parse Error in line {}: Missing class identifier: {}",
                        tokens[index + 1].line.number,
                        tokens[index + 1].line.content
                    );
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: Missing class keyword: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        }
    }

    fn parse_class_variable_declaration(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (ClassVarNode, usize) {
        let mut index = index;

        if let Token::Keyword(var_kind) = &tokens[index].token {
            let var_kind = VarKind::get(var_kind);
            index += 1;
            let var_type = match &tokens[index].token {
                Token::Identifier(var_type) | Token::Keyword(var_type) => VarType::get(&var_type),
                _ => {
                    eprintln!(
                        "Parse Error in line {}: Var kind must be followed by a var type: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            };
            index += 1;

            let mut var_names = Vec::new();
            loop {
                match &tokens[index].token {
                    Token::Symbol(symbol) if symbol == ";" => break,
                    Token::Symbol(symbol) if symbol == "," => (),
                    Token::Identifier(var_name) => {
                        var_names.push(var_name.clone());
                    }
                    _ => {
                        eprintln!(
                            "Parse Error in line {}: Unexpected token in variable declaration: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                }
                index += 1;
            }
            (
                ClassVarNode {
                    names: var_names,
                    var_kind,
                    var_type,
                },
                index + 1,
            )
        } else {
            eprintln!(
                "Parse Error in line {}: Class variable declaration must start with a keyword: {}",
                tokens[index].line.number, tokens[index].line.content
            );
            std::process::exit(1);
        }
    }

    //------------------------------
    // SUBROUTINES
    //------------------------------

    fn parse_subroutine(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
        subroutine_type: &str,
        inside_function: bool,
    ) -> (SubroutineNode, usize) {
        if inside_function {
            eprintln!(
                "Parse Error in line {}: Functions inside functions are not allowed: {}",
                tokens[index].line.number, tokens[index].line.content
            );
        }
        let mut index = index + 1; // skip function type: constructor, function, method
        match &tokens[index].token {
            Token::Keyword(return_val) | Token::Identifier(return_val) => {
                index += 1;
                match &tokens[index].token {
                    Token::Identifier(subroutine_name) => {
                        index += 1;
                        match &tokens[index].token {
                            Token::Symbol(symbol) if symbol == "(" => {
                                let (parameters, mut index) =
                                    self.parse_parameter_list(tokens, index);
                                match &tokens[index].token {
                                    Token::Symbol(symbol) if symbol == "{" => {
                                        let body_end_index =
                                            self.find_closing_bracket(tokens, index);
                                        let mut var_declarations = Vec::new();
                                        index += 1;
                                        loop {
                                            match &tokens[index].token {
                                                Token::Keyword(keyword) if keyword == "var" => {
                                                    let (var_dec, j) = self
                                                        .parse_variable_declaration(
                                                            tokens, index, false,
                                                        );
                                                    index = j;
                                                    var_declarations.push(var_dec);
                                                }
                                                _ => break,
                                            }
                                        }
                                        let (body, _) =
                                            self.parse_body(&tokens[..body_end_index], index);
                                        let subroutine_body = SubroutineBodyNode {
                                            variables: var_declarations,
                                            statements: body,
                                        };
                                        let return_type = if return_val == "void" {
                                            None
                                        } else {
                                            Some(VarType::get(return_val))
                                        };
                                        (
                                            SubroutineNode {
                                                name: subroutine_name.clone(),
                                                subroutine_type: SubroutineType::get(
                                                    subroutine_type,
                                                ),
                                                parameter_list: parameters,
                                                body: subroutine_body,
                                                return_type,
                                            },
                                            body_end_index + 1,
                                        )
                                    }
                                    _ => {
                                        eprintln!(
                                            "Parse Error in line {}: Missing subroutine body: {}",
                                            tokens[index].line.number, tokens[index].line.content
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            _ => {
                                eprintln!(
                                    "Parse Error in line {}: Missing parameter list for subroutine: {}",
                                    tokens[index].line.number, tokens[index].line.content
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    _ => {
                        eprintln!(
                            "Parse Error in line {}: Missing subroutine identifier or missing return type: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: Missing subroutine identifier or missing return type: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        }
    }

    fn parse_variable_declaration(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
        class_vars: bool,
    ) -> (VarNode, usize) {
        let mut index = index;

        if let Token::Keyword(var_kind) = &tokens[index].token {
            let var_kind = VarKind::get(var_kind);
            index += 1;
            let var_type = match &tokens[index].token {
                Token::Identifier(var_type) | Token::Keyword(var_type) => VarType::get(&var_type),
                _ => {
                    eprintln!(
                        "Parse Error in line {}: Var kind must be followed by a var type: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            };
            index += 1;

            let mut var_names = Vec::new();
            loop {
                match &tokens[index].token {
                    Token::Symbol(symbol) if symbol == ";" => break,
                    Token::Symbol(symbol) if symbol == "," => (),
                    Token::Identifier(var_name) => {
                        var_names.push(var_name.to_string());
                    }
                    _ => {
                        eprintln!(
                            "Parse Error in line {}: Unexpected token in variable declaration: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                }
                index += 1;
            }
            (
                VarNode {
                    var_names,
                    var_kind: var_kind.clone(),
                    var_type: var_type.clone(),
                    class_var: class_vars,
                },
                index + 1,
            )
        } else {
            eprintln!(
                "Parse Error in line {}: Variable declaration must start with keyword: {}",
                tokens[index].line.number, tokens[index].line.content
            );
            std::process::exit(1);
        }
    }

    fn parse_body(&self, tokens: &[TokenWrapper], index: usize) -> (Vec<Statement>, usize) {
        let mut index = index;
        let mut statements = Vec::new();
        loop {
            if index == tokens.len() {
                break;
            }
            match &tokens[index].token {
                Token::Keyword(keyword) if self.is_statement(keyword) => {
                    let (statement, j) = self.parse_statement(tokens, keyword, index);
                    index = j;
                    statements.push(statement);
                }
                _ => {
                    println!("{:?}", &tokens[index].token);
                    eprintln!(
                        "Parse Error in line {}: DEBUG: for now, only statements are allowed in bodies: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            }
        }
        (statements, index)
    }

    fn is_subroutine(&self, keyword: &str) -> bool {
        keyword == "constructor" || keyword == "function" || keyword == "method"
    }

    fn parse_parameter_list(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (ParameterListNode, usize) {
        let mut index = index + 1;

        let mut parameters = Vec::new();
        loop {
            match &tokens[index].token {
                Token::Symbol(symbol) if symbol == ")" => {
                    index += 1;
                    break;
                }
                Token::Keyword(var_type) | Token::Identifier(var_type) => {
                    index += 1;
                    if let Token::Identifier(var_name) = &tokens[index].token {
                        index += 1;
                        parameters.push(ParameterNode {
                            name: var_name.clone(),
                            var_type: VarType::get(&var_type[..]),
                        });
                    } else {
                        eprintln!(
                            "Parse Error in line {}: Missing identifier in parameter list: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                }
                Token::Symbol(symbol) if symbol == "," => index += 1,
                _ => {
                    eprintln!(
                        "Parse Error in line {}: Unexpected token in parameter list: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            }
        }

        (ParameterListNode { parameters }, index)
    }

    //------------------------------
    // STATEMENTS
    //------------------------------

    fn parse_statement(
        &self,
        tokens: &[TokenWrapper],
        statement: &str,
        index: usize,
    ) -> (Statement, usize) {
        if statement == "let" {
            let (statement, index) = self.parse_let_statement(tokens, index, index);
            (Statement::Let(statement), index)
        } else if statement == "do" {
            let (statement, index) = self.parse_do_statement(tokens, index);
            (Statement::Do(statement), index)
        } else if statement == "if" {
            let (statement, index) = self.parse_if_statement(tokens, index);
            (Statement::If(statement), index)
        } else if statement == "while" {
            let (statement, index) = self.parse_while_statement(tokens, index);
            (Statement::While(statement), index)
        } else if statement == "return" {
            let (statement, index) = self.parse_return_statement(tokens, index);
            (Statement::Return(statement), index)
        } else {
            // cannot happen
            std::process::exit(1);
        }
    }

    fn parse_let_statement(
        &self,
        tokens: &[TokenWrapper],
        start_index: usize,
        end_index: usize,
    ) -> (LetStatementNode, usize) {
        let mut index = start_index + 1;

        if let Token::Identifier(var_name) = &tokens[index].token {
            index += 1;
            // parse [ expression ]
            let lhs_expression = match &tokens[index].token {
                Token::Symbol(symbol) if symbol == "[" => {
                    let expr_end_index = self.find_closing_bracket(tokens, index);
                    let lhs_expression =
                        self.parse_expression(tokens, index + 1, expr_end_index - 1);
                    index = expr_end_index + 1;
                    Some(lhs_expression)
                }
                _ => None,
            };
            // parse expression after =
            let rhs_expression = match &tokens[index].token {
                Token::Symbol(symbol) if symbol == "=" => {
                    index += 1;
                    let semicolon_index =
                        self.find_symbol(tokens, ";", index, end_index)
                            .expect(&format!(
                            "Parse Error in Line {}: let statement must end with a semicolon: {}",
                            tokens[index].line.number, tokens[index].line.content
                        ));

                    let rhs_expression = self.parse_expression(tokens, index, semicolon_index - 1);
                    index = semicolon_index;
                    rhs_expression
                }
                _ => {
                    eprintln!(
                        "Parse Error in line {}: let statement in missing the '=' symbol: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            };
            (
                LetStatementNode {
                    var_name: var_name.to_string(),
                    lhs_expression,
                    rhs_expression,
                },
                index + 1,
            )
        } else {
            eprintln!(
                "Parse Error in line {}: 'let' keyword must be followed by an identifier: {}",
                tokens[index].line.number, tokens[index].line.content
            );
            std::process::exit(1);
        }
    }

    fn parse_do_statement(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (DoStatementNode, usize) {
        let mut index = index + 1;

        let caller = if let Token::Identifier(caller) = &tokens[index].token {
            let mut j = index + 1;
            match &tokens[j].token {
                Token::Symbol(symbol) if symbol == "." => {
                    j += 1;
                    if let Token::Identifier(_func_name) = &tokens[j].token {
                        index = j;
                        Some(caller.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            eprintln!(
                "Parse Error in line {}: 'do' keyword must be followed by an identifier: {}",
                tokens[index].line.number, tokens[index].line.content
            );
            std::process::exit(1);
        };
        let subroutine_call = if let Token::Identifier(func_name) = &tokens[index].token {
            index += 1;
            let expression_nodes = match &tokens[index].token {
                Token::Symbol(symbol) if symbol == "(" => {
                    let closing_bracket_index = self.find_closing_bracket(tokens, index);
                    let expression_list =
                        self.parse_expression_list(tokens, index + 1, closing_bracket_index - 1);
                    index = closing_bracket_index + 1;
                    expression_list
                }
                _ => {
                    eprintln!(
                        "Parse Error in line {}: Subroutine name in do statement must be followed by an open bracket: {}",
                        tokens[index].line.number, tokens[index].line.content
                    );
                    std::process::exit(1);
                }
            };
            SubroutineCallNode {
                subroutine_name: func_name.to_string(),
                caller,
                expression_list: expression_nodes,
                semicolon: true,
            }
        } else {
            eprintln!(
                "Parse Error in line {}: Missing subroutine name in do statement: {}",
                tokens[index].line.number, tokens[index].line.content
            );
            std::process::exit(1);
        };
        match &tokens[index].token {
            Token::Symbol(symbol) if symbol == ";" => (),
            _ => {
                eprintln!(
                    "Parse Error in line {}: do statement must end with a semicolon: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        }
        (DoStatementNode { subroutine_call }, index + 1)
    }

    fn parse_return_statement(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (ReturnStatementNode, usize) {
        let index = index + 1;
        match &tokens[index].token {
            Token::Symbol(symbol) if symbol == ";" => {
                (ReturnStatementNode { expression: None }, index + 1)
            }
            _ => {
                let semicolon_index =
                    self.find_symbol(tokens, ";", index, tokens.len())
                        .expect(&format!(
                            "Parse Error in line {}: Missing semicolon in return statement: {}",
                            tokens[index].line.number, tokens[index].line.content
                        ));

                let expression = self.parse_expression(tokens, index, semicolon_index - 1);
                (
                    ReturnStatementNode {
                        expression: Some(expression),
                    },
                    semicolon_index + 1,
                )
            }
        }
    }

    fn parse_if_statement(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (IfStatementNode, usize) {
        let mut index = index + 1;
        let condition = match &tokens[index].token {
            Token::Symbol(symbol) if symbol == "(" => {
                let closing_bracket_index = self.find_closing_bracket(tokens, index);
                let expression =
                    self.parse_expression(tokens, index + 1, closing_bracket_index - 1);
                index = closing_bracket_index + 1;
                expression
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: if statement must be followed by an open bracket: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        };

        match &tokens[index].token {
            Token::Symbol(symbol) if symbol == "{" => {
                let end_if_body = self.find_closing_bracket(tokens, index);
                let (if_block, _) = self.parse_body(&tokens[..end_if_body], index + 1);
                index = end_if_body + 1; // skip closing curly bracket
                let (else_block, j) = if index < tokens.len() {
                    match &tokens[index].token {
                        Token::Keyword(keyword) if keyword == "else" => {
                            index += 1;
                            match &tokens[index].token {
                                Token::Symbol(symbol) if symbol == "{" => {
                                    let end_else_body = self.find_closing_bracket(tokens, index);
                                    let (else_block, _) =
                                        self.parse_body(&tokens[..end_else_body], index + 1);
                                    (Some(else_block), end_else_body + 1)
                                }
                                _ => {
                                    eprintln!(
                                        "Parse Error in line {}: 'else' keyword in if statement must be followed by an open curly bracket: {}",
                                        tokens[index].line.number, tokens[index].line.content
                                    );
                                    std::process::exit(1);
                                }
                            }
                        }
                        _ => (None, index),
                    }
                } else {
                    (None, index)
                };
                index = j;
                (
                    IfStatementNode {
                        condition,
                        if_block,
                        else_block,
                    },
                    index,
                )
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: Expression in if statement must be followed by an open curly bracket: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        }
    }

    fn parse_while_statement(
        &self,
        tokens: &[TokenWrapper],
        index: usize,
    ) -> (WhileStatementNode, usize) {
        let mut index = index + 1;

        let condition = match &tokens[index].token {
            Token::Symbol(symbol) if symbol == "(" => {
                let closing_bracket_index = self.find_closing_bracket(tokens, index);
                let expression =
                    self.parse_expression(tokens, index + 1, closing_bracket_index - 1);
                index = closing_bracket_index + 1;
                expression
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: while statement must be followed by an open bracket: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        };

        match &tokens[index].token {
            Token::Symbol(symbol) if symbol == "{" => {
                let end_body = self.find_closing_bracket(tokens, index);
                let (block, _) = self.parse_body(&tokens[..end_body], index + 1);
                (WhileStatementNode { condition, block }, end_body + 1)
            }
            _ => {
                eprintln!(
                    "Parse Error in line {}: Expression in while statement must be followed by an open curly bracket: {}",
                    tokens[index].line.number, tokens[index].line.content
                );
                std::process::exit(1);
            }
        }
    }

    //------------------------------
    // EXPRESSIONS
    //------------------------------

    fn parse_expression(
        &self,
        tokens: &[TokenWrapper],
        start_index: usize,
        end_index: usize,
    ) -> ExpressionNode {
        let mut expr_elements = Vec::new();
        let mut index = start_index;

        // 0 = nothing processed, 1 = token processed, 2 = operator processed
        let mut status = 0;
        while index < tokens.len() && index <= end_index {
            match &tokens[index].token {
                Token::Symbol(symbol) if symbol == ";" => {
                    if status == 0 {
                        eprintln!(
                            "Parse Error in line {}: Expression must contain at least one term: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                    break;
                }
                Token::Symbol(symbol) if self.is_operator(symbol) && status == 1 => {
                    // check status == 1 to avoid this case when there is a unary op in front of a term
                    if status == 2 {
                        eprintln!(
                            "Parse Error in line {}: Operator cannot be followed by an operator in expression: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                    status = 2;
                    expr_elements.push(ExpressionElement::Operator(symbol.to_string()));
                    index += 1;
                }
                _ => {
                    if status == 1 {
                        eprintln!(
                            "Parse Error in line {}: Term cannot be followed by a term in expression: {}",
                            tokens[index].line.number, tokens[index].line.content
                        );
                        std::process::exit(1);
                    }
                    status = 1;
                    let (term_node, j) = self.parse_term(tokens, index);
                    expr_elements.push(ExpressionElement::Term(term_node));
                    index = j;
                }
            }
        }
        ExpressionNode {
            elements: expr_elements,
        }
    }

    fn parse_term(&self, tokens: &[TokenWrapper], start_index: usize) -> (TermNode, usize) {
        let index = start_index;
        match &tokens[index].token {
            Token::Constant(Constant::IntegerConstant(value)) => (
                TermNode {
                    elements: vec![TermElement::IntegerConstant(*value)],
                },
                index + 1,
            ),
            Token::Constant(Constant::StringConstant(value)) => (
                TermNode {
                    elements: vec![TermElement::StringConstant(value.to_string())],
                },
                index + 1,
            ),
            Token::Symbol(symbol) if symbol == "(" || symbol == "[" => {
                let closing_bracket_index = self.find_closing_bracket(tokens, index);
                let expression =
                    self.parse_expression(tokens, index + 1, closing_bracket_index - 1);
                let mut term_elems = Vec::new();
                term_elems.push(TermElement::Symbol(symbol.to_string()));
                term_elems.push(TermElement::Expression(expression));
                let closing_bracket = if symbol == "(" {
                    ")".to_string()
                } else {
                    "]".to_string()
                };
                term_elems.push(TermElement::Symbol(closing_bracket));
                (
                    TermNode {
                        elements: term_elems,
                    },
                    closing_bracket_index + 1,
                )
            }
            Token::Identifier(identifier) => {
                let mut j = index + 1;
                match &tokens[j].token {
                    Token::Symbol(symbol) if symbol == "[" => {
                        // foo[expression]
                        let closing_bracket_index = self.find_closing_bracket(tokens, j);
                        let expression =
                            self.parse_expression(tokens, j + 1, closing_bracket_index - 1);
                        let mut term_elems = Vec::new();
                        term_elems.push(TermElement::Identifier(identifier.to_string()));
                        term_elems.push(TermElement::Symbol("[".to_string()));
                        term_elems.push(TermElement::Expression(expression));
                        term_elems.push(TermElement::Symbol("]".to_string()));
                        (
                            TermNode {
                                elements: term_elems,
                            },
                            closing_bracket_index + 1,
                        )
                    }
                    Token::Symbol(symbol) if symbol == "." => {
                        // foo.bar(expressionList)
                        // Foo.bar(expressionList)
                        j += 1;
                        match &tokens[j].token {
                            Token::Identifier(func_name) => {
                                j += 1;
                                match &tokens[j].token {
                                    Token::Symbol(symbol) if symbol == "(" => {
                                        let closing_bracket_index =
                                            self.find_closing_bracket(tokens, j);
                                        let expression_list = self.parse_expression_list(
                                            tokens,
                                            j + 1,
                                            closing_bracket_index - 1,
                                        );

                                        let subroutine_call = SubroutineCallNode {
                                            subroutine_name: func_name.to_string(),
                                            expression_list,
                                            caller: Some(identifier.to_string()),
                                            semicolon: false,
                                        };
                                        (
                                            TermNode {
                                                elements: vec![TermElement::SubroutineCall(
                                                    subroutine_call,
                                                )],
                                            },
                                            closing_bracket_index + 1,
                                        )
                                    }
                                    _ => {
                                        eprintln!(
                                            "Parse Error in line {}: Missing opening bracket after subroutine name: {:?}",
                                            tokens[index].line.number, tokens[index].token
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            _ => {
                                eprintln!(
                                    "Parse Error in line {}: Missing subroutine name: {:?}",
                                    tokens[index].line.number, tokens[index].token
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    Token::Symbol(symbol) if symbol == "(" => {
                        // bar(expression)
                        let closing_bracket_index = self.find_closing_bracket(tokens, j);
                        let expression_list =
                            self.parse_expression_list(tokens, j + 1, closing_bracket_index - 1);

                        let subroutine_call = SubroutineCallNode {
                            subroutine_name: identifier.to_string(),
                            expression_list,
                            caller: None,
                            semicolon: false,
                        };
                        (
                            TermNode {
                                elements: vec![TermElement::SubroutineCall(subroutine_call)],
                            },
                            closing_bracket_index + 1,
                        )
                    }
                    _ => {
                        // foo
                        (
                            TermNode {
                                elements: vec![TermElement::Identifier(identifier.to_string())],
                            },
                            index + 1,
                        )
                    }
                }
            }
            Token::Keyword(keyword) => (
                TermNode {
                    elements: vec![TermElement::KeywordConstant(keyword.to_string())],
                },
                index + 1,
            ),
            Token::Symbol(symbol)
            if (symbol == "-" || symbol == "~")
                && index > 0
                && self.token_is_symbol(&tokens[index].token) =>
                {
                    // unary operator followed by term
                    let (term_node, j) = self.parse_term(tokens, index + 1);
                    let op = TermElement::Symbol(symbol.to_string());
                    (
                        TermNode {
                            elements: vec![op, TermElement::Term(term_node)],
                        },
                        j + 1,
                    )
                }
            _ => {
                eprintln!(
                    "Parse Error in line {}: Unexpected token found in term: {:?}",
                    tokens[index].line.number, tokens[index].token
                );
                std::process::exit(1);
            }
        }
    }

    fn parse_expression_list(
        &self,
        tokens: &[TokenWrapper],
        start_index: usize,
        end_index: usize,
    ) -> Vec<ExpressionNode> {
        let mut index = start_index;
        let mut expression_nodes = Vec::new();
        while index <= end_index {
            let expr_end_index = match self.next_comma_in_expression_list(tokens, index, end_index) {
                Ok(comma_index) => comma_index,
                Err(_) => end_index + 1,
            };
            let expression = self.parse_expression(tokens, index, expr_end_index - 1);
            expression_nodes.push(expression);
            index = expr_end_index + 1;
        }
        expression_nodes
    }

    //------------------------------
    // HELPER FUNCTIONS
    //------------------------------

    fn is_statement(&self, keyword: &str) -> bool {
        keyword == "let"
            || keyword == "if"
            || keyword == "else"
            || keyword == "while"
            || keyword == "do"
            || keyword == "return"
    }

    fn find_closing_bracket(&self, tokens: &[TokenWrapper], start_index: usize) -> usize {
        let (opening_bracket, closing_bracket) = match &tokens[start_index].token {
            Token::Symbol(symbol) if symbol == "{" => ("{", "}"),
            Token::Symbol(symbol) if symbol == "(" => ("(", ")"),
            Token::Symbol(symbol) if symbol == "[" => ("[", "]"),
            _ => {
                eprintln!(
                    "Parse Error: Start token for closing bracket search must be an open bracket."
                );
                std::process::exit(1);
            }
        };

        let mut stack = Vec::new();
        stack.push(opening_bracket);

        for i in start_index + 1..tokens.len() {
            match &tokens[i].token {
                Token::Symbol(symbol) => {
                    if *symbol == opening_bracket {
                        stack.push(opening_bracket);
                    } else if *symbol == closing_bracket {
                        if let Some(bracket) = stack.pop() {
                            if bracket != opening_bracket {
                                eprintln!(
                                    "Parse Error: Cannot find opening bracket for bracket in line {}: {}.",
                                    tokens[i].line.number,
                                    tokens[i].line.content
                                );
                                std::process::exit(1);
                            } else {
                                if stack.is_empty() {
                                    return i;
                                }
                            }
                        } else {
                            eprintln!(
                                "Parse Error: Cannot find closing bracket for bracket in line {}: {}.",
                                tokens[start_index].line.number,
                                tokens[start_index].line.content
                            );
                            std::process::exit(1);
                        }
                    }
                }
                _ => (),
            }
        }
        // dummy return
        0
    }

    fn find_symbol(
        &self,
        tokens: &[TokenWrapper],
        target_symbol: &str,
        start_index: usize,
        end_index: usize,
    ) -> Result<usize, ParseError> {
        let mut index = start_index;
        loop {
            if index == end_index {
                return Err(ParseError);
            }
            if let Token::Symbol(symbol) = &tokens[index].token {
                if symbol == target_symbol {
                    return Ok(index);
                }
            }
            index += 1;
        }
    }

    fn next_comma_in_expression_list(
        &self,
        tokens: &[TokenWrapper],
        start_index: usize,
        end_index: usize,
    ) -> Result<usize, ParseError> {
        let mut index = start_index;
        let mut bracket_count = 0;
        loop {
            if index == end_index {
                return Err(ParseError);
            }
            if let Token::Symbol(symbol) = &tokens[index].token {
                if symbol == "(" {
                    bracket_count += 1;
                }
                if symbol == ")" {
                    bracket_count -= 1;
                }
                if symbol == "," && bracket_count == 0 {
                    return Ok(index);
                }
            }
            index += 1;
        }
    }

    fn token_is_symbol(&self, token: &Token) -> bool {
        match token {
            Token::Symbol(_) => true,
            _ => false,
        }
    }

    fn is_operator(&self, symbol: &str) -> bool {
        symbol == "+"
            || symbol == "-"
            || symbol == "*"
            || symbol == "/"
            || symbol == "&"
            || symbol == "|"
            || symbol == "<"
            || symbol == ">"
            || symbol == "="
    }
}
