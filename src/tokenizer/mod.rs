//! Reads text files containing Jack code and produces a list of tokens.
use crate::io::line::Line;
use std::rc::Rc;

pub mod tokens;
use tokens::{Constant, Token, TokenWrapper};

/// Reads text files containing Jack code and produces a list of tokens.
pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {}
    }

    /// Takes in a Vec of [`Line`](crate::io::line::Line) values,
    /// one for each line in the Jack file and returns a list of
    /// [`TokenWrapper`](tokens::TokenWrapper) values.
    pub fn tokenize(&self, lines: Vec<Rc<Line>>) -> Vec<TokenWrapper> {
        let mut tokens = Vec::new();

        let mut comment = false;
        for line in lines {
            // ignore comments
            if line.content.starts_with("/*") {
                // a string that starts with /** also starts with /*
                if line.content.ends_with("*/") {
                    continue;
                } else {
                    comment = true;
                }
            }
            if line.content.ends_with("*/") || line.content.starts_with("*/") {
                comment = false;
                continue;
            }
            if comment {
                continue;
            }

            let chars: Vec<char> = line.content.chars().collect();
            self.tokenize_line(chars, 0, &line, &mut tokens);
        }

        tokens
    }

    fn tokenize_line(
        &self,
        chars: Vec<char>,
        start_index: usize,
        line: &Rc<Line>,
        tokens: &mut Vec<TokenWrapper>,
    ) {
        if start_index == chars.len() {
            return;
        }

        let mut i = start_index;
        while i < chars.len() {
            if self.is_symbol(chars[i]) {
                tokens.push(TokenWrapper {
                    token: Token::Symbol(chars[i].to_string()),
                    line: line.clone(),
                });
                i += 1;
            } else if chars[i] == ' ' {
                i += 1;
            } else if chars[i] == '"' {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    j += 1;
                }
                let string_constant: String = chars[i + 1..j].iter().collect();
                i = j + 1; // skip last double quote
                tokens.push(TokenWrapper {
                    token: Token::Constant(Constant::StringConstant(string_constant)),
                    line: line.clone(),
                });
            } else {
                let mut j = i;
                while j < chars.len() && !self.is_symbol(chars[j]) && chars[j] != ' ' {
                    j += 1;
                }
                let unknown_token: String = chars[i..j].iter().collect();
                i = j;
                //println!("{}", unknown_token);
                if unknown_token.is_empty() {
                    // not sure if this is necessary
                    continue;
                } else if unknown_token.chars().next().unwrap().is_digit(10) {
                    // if token starts with digit, it is a number
                    match unknown_token.parse::<u32>() {
                        Ok(num) => {
                            tokens.push(TokenWrapper {
                                token: Token::Constant(Constant::IntegerConstant(num)),
                                line: line.clone(),
                            });
                        }
                        Err(_) => {
                            eprintln!("Parse error in line {}:", line.number);
                            eprintln!("{}", line.content);
                            eprintln!("Identifiers cannot start with a digit.");
                            std::process::exit(1);
                        }
                    }
                } else if self.is_keyword(&unknown_token) {
                    tokens.push(TokenWrapper {
                        token: Token::Keyword(unknown_token),
                        line: line.clone(),
                    });
                } else {
                    tokens.push(TokenWrapper {
                        token: Token::Identifier(unknown_token),
                        line: line.clone(),
                    });
                }
            }
        }
    }

    /// Returns the XML representation of the given tokens, as specified in the book.
    pub fn write_xml(&self, tokens: Vec<TokenWrapper>) -> String {
        let mut xml_lines = Vec::new();
        xml_lines.push("<tokens>".to_string());
        for token in tokens {
            match token.token {
                Token::Symbol(s) => {
                    if s == "<" {
                        xml_lines.push(format!("<symbol> {} </symbol>", "&lt;"));
                    } else if s == ">" {
                        xml_lines.push(format!("<symbol> {} </symbol>", "&gt;"));
                    } else if s == "&" {
                        xml_lines.push(format!("<symbol> {} </symbol>", "&amp;"));
                    } else if s == "\"" {
                        xml_lines.push(format!("<symbol> {} </symbol>", "&quot;"));
                    } else {
                        xml_lines.push(format!("<symbol> {} </symbol>", s));
                    }
                },
                Token::Keyword(k) => xml_lines.push(format!("<keyword> {} </keyword>", k)),
                Token::Identifier(name) => xml_lines.push(format!("<identifier> {} </identifier>", name)),
                Token::Constant(constant) => match constant {
                    Constant::IntegerConstant(i) => {
                        xml_lines.push(format!("<integerConstant> {} </integerConstant>", i))
                    }
                    Constant::StringConstant(s) => {
                        xml_lines.push(format!("<stringConstant> {} </stringConstant>", s))
                    }
                    Constant::BooleanConstant(b) => {
                        xml_lines.push(format!("<booleanConstant> {} </booleanConstant>", b))
                    }
                },
            }
        }
        xml_lines.push("</tokens>".to_string());
        let mut xml = xml_lines.join("\n");
        xml.push('\n');
        xml
    }

    fn is_symbol(&self, c: char) -> bool {
        c == '('
            || c == ')'
            || c == '['
            || c == ']'
            || c == '{'
            || c == '}'
            || c == ','
            || c == ';'
            || c == '='
            || c == '.'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '&'
            || c == '|'
            || c == '~'
            || c == '<'
            || c == '>'
    }

    fn is_keyword(&self, s: &str) -> bool {
        s == "class"
            || s == "constructor"
            || s == "method"
            || s == "function"
            || s == "int"
            || s == "boolean"
            || s == "char"
            || s == "void"
            || s == "var"
            || s == "static"
            || s == "field"
            || s == "let"
            || s == "do"
            || s == "if"
            || s == "else"
            || s == "while"
            || s == "return"
            || s == "true"
            || s == "false"
            || s == "null"
            || s == "this"
    }
}
