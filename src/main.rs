use std::{
    fs::{self},
    io::{self, Write},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    command: String,
    file_name: String,
}

pub fn main() {
    let args = Args::parse();
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
