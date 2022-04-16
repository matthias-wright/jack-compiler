use std::fs;

use jack_compiler::io;
use jack_compiler::tokenizer::Tokenizer;
use jack_compiler::parser::Parser;
use jack_compiler::vm_writer::VMWriter;

fn compare(path_to_jack_file: &str, path_to_target_vm: &str) -> (String, String) {
    let tokenizer = Tokenizer::new();
    let parser = Parser::new();
    let vm_writer = VMWriter::new();
    let lines = io::read_file(path_to_jack_file);
    let tokens = tokenizer.tokenize(lines);
    let parse_tree = parser.parse(tokens, path_to_jack_file);
    let vm_code = vm_writer.write(&parse_tree);
    let target_vm = fs::read_to_string(path_to_target_vm)
        .expect("Reading target file failed.");
    (vm_code, target_vm)
}

#[test]
fn square_main_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Square/Main.jack",
        "tests/aux_files/Square/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn square_square_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Square/Square.jack",
        "tests/aux_files/Square/Square.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn square_square_game_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Square/SquareGame.jack",
        "tests/aux_files/Square/SquareGame.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn seven_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Seven/Main.jack",
        "tests/aux_files/Seven/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn convert_to_bin_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/ConvertToBin/Main.jack",
        "tests/aux_files/ConvertToBin/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn square_dance_main_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/SquareDance/Main.jack",
        "tests/aux_files/SquareDance/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn square_dance_square_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/SquareDance/Square.jack",
        "tests/aux_files/SquareDance/Square.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn square_dance_square_game_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/SquareDance/SquareGame.jack",
        "tests/aux_files/SquareDance/SquareGame.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn average_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Average/Main.jack",
        "tests/aux_files/Average/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn pong_main_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Pong/Main.jack",
        "tests/aux_files/Pong/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn pong_ball_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Pong/Ball.jack",
        "tests/aux_files/Pong/Ball.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn pong_bat_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Pong/Bat.jack",
        "tests/aux_files/Pong/Bat.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn pong_pong_game_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/Pong/PongGame.jack",
        "tests/aux_files/Pong/PongGame.vm",
    );
    assert_eq!(vm_code, target_vm);
}

#[test]
fn complex_arrays_test() {
    let (vm_code, target_vm) = compare(
        "tests/aux_files/ComplexArrays/Main.jack",
        "tests/aux_files/ComplexArrays/Main.vm",
    );
    assert_eq!(vm_code, target_vm);
}
