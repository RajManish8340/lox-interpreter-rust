use std::fs::{self};

use clap::Parser;

#[derive(Parser)]
struct Args {
    command: String,
    file_name: String,
}

pub fn main() {
    let args = Args::parse();
    match args.command.as_str() {
        "tokenize" => {
            let content = read_file(args.file_name.as_str());

            if content.is_empty() {
                println!("EOF  null")
            }
        }
        _ => {
            panic!()
        }
    }

    fn read_file(file_name: &str) -> String {
        let content = fs::read_to_string(&file_name).unwrap_or_else(|_| {
            eprint!("error reading file {}", file_name);
            String::new()
        });
        content
    }
}
