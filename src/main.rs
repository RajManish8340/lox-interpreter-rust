mod ast;
mod common;
mod errors;
mod parse;
mod scanner;
mod token;
use std::{
    fs::{self},
    hint::assert_unchecked,
};

use clap::Parser;

use crate::{
    errors::HAS_ERRORS,
    parse::{AstParser, print_expr},
    scanner::Scanner,
};

#[derive(Parser)]
struct Args {
    command: String,
    file_name: String,
}

pub fn main() {
    let args = Args::parse();
    let file_content = read_file(args.file_name.as_str());
    let mut scanner = Scanner::new(&file_content);
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

    match args.command.as_str() {
        "tokenize" => {
            for token in &tokens {
                let literal = token
                    .literal
                    .to_owned()
                    .unwrap_or_else(|| token::LiteralType::String("null".to_string()));
                print!(
                    "{:?} {} {} {} \r\n",
                    token.kind, token.lexeme, literal, token.line
                );
            }
        }
        "parse" => {
            let mut ast_parser = AstParser::new(tokens, 0);
            match ast_parser.factor() {
                Ok(expr) => println!("{}", print_expr(&expr)),
                Err(e) => println!("{}", e),
            }
        }

        _ => {
            println!("not a valid command")
        }
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
