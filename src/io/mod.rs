//! This module handles file reading and writing.
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub mod line;
use line::Line;

/// Returns the paths of all the Jack files in the specified directory.
pub fn get_file_paths(path: &str) -> Vec<String> {
    let mut file_paths = Vec::new();
    if Path::new(path).is_dir() {
        let files = fs::read_dir(path).expect(&format!("Error: could not read directory: '{}'", path));
        for f in files {
            if let Ok(f) = f {
                if let Ok(filename) = f.path().into_os_string().into_string() {
                    if filename.ends_with(".jack") {
                        file_paths.push(filename);
                    }
                } else {
                    println!("Warning: the file '{:?}' will be ignored because the filename is not proper UTF-8.", f.file_name());
                }
            }
        }
    } else {
        if path.ends_with(".jack") {
            file_paths.push(path.to_string());
        } else {
            eprintln!("Error: if a single file is specified, it must end with '.jack': '{}'", path);
            std::process::exit(1);
        }
    }
    file_paths
}

/// Reads a Jack file and returns a Vec containing a [`Line`](line::Line) for each line in the file.
pub fn read_file(file_path: &str) -> Vec<Rc<Line>> {
    let contents = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: failed to read file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };
    contents
        .split('\n')
        .map(|s| {
            if let Some(index) = s.find("//") {
                &s[..index]
            } else {
                s
            }
        })
        .enumerate()
        .map(|(i, s)| Line::new(s.trim(), i + 1))
        .map(|line| line.replace_content("\r", ""))
        .filter(|line| !line.content.is_empty())
        .filter(|line| !line.content.starts_with("//"))
        .map(|line| Rc::new(line))
        .collect()
}

/// Writes the specified content to the specified file.
pub fn write_file(file_path: &str, content: &str) {
    match fs::write(file_path, content) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: failed to write to file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };
}