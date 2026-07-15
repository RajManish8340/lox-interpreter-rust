mod errors;
mod tokenization;
use std::fs::{self};

use clap::Parser;

use crate::{errors::HAS_ERRORS, tokenization::Scanner};

#[derive(Parser)]
struct Args {
    command: String,
    file_name: String,
}

pub fn main() {
    let args = Args::parse();
    let file_content = read_file(args.file_name.as_str());
    let mut scanner = Scanner::new(&file_content);
    if args.command == "tokenize" {
        let (tokens, errors) = Scanner::scan_token(&mut scanner);

        for error in &errors {
            print!(
                "[line {}] Error: {}: {}\r\n",
                error.line, error.message, error.character
            );
        }

        if !errors.is_empty() {
            *errors::HAS_ERRORS
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()) = true;
        }

        print!(
            "\r\n\r\nthe value of 'data' means if it contains errors or not -> {:?}\r\n\r\n",
            HAS_ERRORS
        );

        for token in &tokens {
            print!(
                "{:?} {} {} {} \r\n",
                token.kind, token.lexeme, token.literal, token.line
            );
        }
    } else {
        print!("not a valid command")
    }
}

fn read_file(file_name: &str) -> String {
    let content = fs::read_to_string(&file_name).unwrap_or_else(|_| {
        eprint!("error reading file {}", file_name);
        String::new()
    });
    content
}

// fn run_promt() -> io::Result<()> {
//     let mut buffer = String::new();
//     let input_stream_reader = io::stdin();
//
//     loop {
//         print!(">");
//         io::stdout().flush()?;
//
//         buffer.clear();
//
//         let bytes_read = input_stream_reader.read_line(&mut buffer)?;
//
//         if bytes_read == 0 {
//             break;
//         }
//
//         let line = buffer.trim_end();
//
//         run()
//       }
//     }
