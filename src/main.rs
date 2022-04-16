use std::env;

use jack_compiler::io;
use jack_compiler::tokenizer::Tokenizer;
use jack_compiler::parser::Parser;
use jack_compiler::vm_writer::VMWriter;

fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!(
            "Error: wrong number of arguments: expected 1, got {}",
            args.len()
        );
        println!("Usage: jack_compiler <INPUT_FILE>.jack OR jack_compiler <INPUT_DIR>");
        std::process::exit(1);
    }
    args[0].clone()
}

fn main() {
    let path = parse_args();
    let file_paths = io::get_file_paths(&path);

    let tokenizer = Tokenizer::new();
    let parser = Parser::new();
    let vm_writer = VMWriter::new();

    for file_path in file_paths.iter() {
        let filepath_wo_ending = match file_path.rfind(".jack") {
            Some(idx) => &file_path[..idx],
            None => &file_path,
        };
        let lines = io::read_file(file_path);
        let tokens = tokenizer.tokenize(lines);
        let parse_tree = parser.parse(tokens, filepath_wo_ending);
        let vm_code = vm_writer.write(&parse_tree);
        io::write_file(&format!("{}.vm", filepath_wo_ending), &vm_code);
    }
}
