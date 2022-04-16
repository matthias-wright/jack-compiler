use std::fs;

use jack_compiler::io;
use jack_compiler::tokenizer::Tokenizer;

fn compare(path_to_jack_file: &str, path_to_target_xml: &str) -> (String, String) {
    let tokenizer = Tokenizer::new();
    let lines = io::read_file(path_to_jack_file);
    let tokens = tokenizer.tokenize(lines);
    let xml = tokenizer.write_xml(tokens);
    let target_xml = fs::read_to_string(path_to_target_xml)
        .expect("Reading target file failed.");
    (xml, target_xml.replace("\r", ""))
}

#[test]
fn array_test_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ArrayTest/Main.jack",
        "tests/aux_files/ArrayTest/MainT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_main_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/Main.jack",
        "tests/aux_files/Square/MainT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_square_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/Square.jack",
        "tests/aux_files/Square/SquareT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn square_square_game_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/Square/SquareGame.jack",
        "tests/aux_files/Square/SquareGameT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn expr_less_square_main_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/Main.jack",
        "tests/aux_files/ExpressionLessSquare/MainT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn expr_less_square_square_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/Square.jack",
        "tests/aux_files/ExpressionLessSquare/SquareT.xml",
    );
    assert_eq!(xml, target_xml);
}

#[test]
fn expr_less_square_square_game_test() {
    let (xml, target_xml) = compare(
        "tests/aux_files/ExpressionLessSquare/SquareGame.jack",
        "tests/aux_files/ExpressionLessSquare/SquareGameT.xml",
    );
    assert_eq!(xml, target_xml);
}
