use std::fs;

use jack_compiler::io;
use jack_compiler::tokenizer::Tokenizer;
use jack_compiler::parser::Parser;

fn remove_whitespace(s: String) -> String {
    let mut chars = Vec::new();
    let mut found_newline = false;
    for char in s.chars() {
        if char == '\n' {
            found_newline = true;
        }
        if found_newline && char != '\n' {
            if char == ' ' {
                continue;
            } else {
                found_newline = false;
            }
        }
        chars.push(char);
    }
    chars.into_iter().collect()
}

fn compare(path_to_jack_file: &str, path_to_target_xml: &str) -> (String, String) {
    let tokenizer = Tokenizer::new();
    let parser = Parser::new();
    let lines = io::read_file(path_to_jack_file);
    let tokens = tokenizer.tokenize(lines);
    let parse_tree = parser.parse(tokens, path_to_jack_file);
    let xml = format!("{}", parse_tree.class_node);
    let target_xml = fs::read_to_string(path_to_target_xml)
        .expect("Reading target file failed.");
    let target_xml = remove_whitespace(target_xml);
    (xml, target_xml.replace("\r", ""))
}

#[test]
fn expr_less_square_main_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/Main.jack",
        "tests/aux_files/ExpressionLessSquare/Main.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn expr_less_square_square_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/Square.jack",
        "tests/aux_files/ExpressionLessSquare/Square.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn expr_less_square_square_game_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/SquareGame.jack",
        "tests/aux_files/ExpressionLessSquare/SquareGame.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_main_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/Main.jack",
        "tests/aux_files/Square/Main.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_square_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/Square.jack",
        "tests/aux_files/Square/Square.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_square_game_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/SquareGame.jack",
        "tests/aux_files/Square/SquareGame.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn array_test_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ArrayTest/Main.jack",
        "tests/aux_files/ArrayTest/Main.xml",
    );
    assert_eq!(xml, target_xml);
}
