use std::io;
use std::io::Write;
use std::process::exit;

use lexer::{Lexer};
use token::{TokenType};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        let _ = io::stdout().write(PROMPT.as_bytes());
        let _ = io::stdout().flush();

        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        let len = line.len();

        // strip \n
        line.truncate(len - 1);

        if line == "exit" {
            println!("{:?}", "Goodbye!");
            exit(0);
        }

        let mut l = Lexer::new(line);

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            } else {
                println!("Literal: {:?}, Type: {:?}", tok.literal, tok.token_type);
            }
        }
    }
}