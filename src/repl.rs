use std::io;
use std::io::Write;
use std::process::exit;

use lexer::{Lexer};
use token::{TokenType};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut stdout = io::stdout();
    loop {
        stdout.write(PROMPT.as_bytes()).expect("Failed to write prompt");
        stdout.flush().expect("Failed to flush stdout");

        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        let len = line.len();

        // strip \n
        line.truncate(len - 1);

        if line == "exit" {
            stdout.write("Goodbye!".as_bytes()).expect("Failed to write exit message");
            exit(0);
        }

        let mut l = Lexer::new(line);

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            } else {
                // TODO format string w/o using println!, mostly as an exercise
                println!("Literal: {:?}, Type: {:?}", tok.literal, tok.token_type);
            }
        }
    }
}