use lexer::{Lexer};
use token::{Token, TokenType};
//use ast::{Program};

pub struct Parser<'p> {
    lexer: &'p mut Lexer,
    errors: Vec<&'p str>,
    current_token: Token<'p>,
    peek_token: Token<'p>
}

// l must live at least as long as p
impl<'p, 'l: 'p> Parser<'p> {
    pub fn new(lexer: &'l mut Lexer) -> Parser<'p> {
//        let current_token = {
//            lexer.next_token();
//            lexer.current_token()
//        };
//
//        let peek_token = {
//            lexer.next_token();
//            lexer.current_token()
//        };

        lexer.next_token();
        let blah = lexer.current_token();

        let current_token = Token {
          token_type: TokenType::Illegal,
          literal: ""
        };
        let peek_token = Token {
          token_type: TokenType::Illegal,
          literal: ""
        };

        Parser { lexer, current_token, peek_token, errors: Vec::new() }
    }

//    pub fn parse_program(&mut self) -> Program {
//        Program {
//            statements: Vec<Box<Statement>>
//        }
//    }

    fn next_token(&'p mut self) {
        self.current_token = self.peek_token;
        self.peek_token = {
            self.lexer.next_token();
            self.lexer.current_token()
        };
    }
}
