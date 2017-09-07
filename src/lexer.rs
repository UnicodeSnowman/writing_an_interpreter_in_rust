use token;

pub struct Lexer {
  input: String, // TODO can I get away with using &str here instead?
  position: usize,
  read_position: usize,
  ch: Option<char>,
}

fn is_letter(ch: char) -> bool {
  ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z'
}

fn is_digit(ch: char) -> bool {
  ch >= '0' && ch <= '9'
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      input: String::from(input),
      position: 0,
      read_position: 0,
      ch: input.chars().nth(0),
    };

    lexer.read_char();
    lexer
  }

  pub fn next_token(&mut self) -> token::Token {
    let mut tok = token::Token {
      token_type: token::TokenType::Illegal,
      literal: ""
    };

    self.skip_whitespace();

    match self.ch {
      Some('=') => {
        match self.peek_char() {
          Some('=') => {
            self.read_char();
            tok.token_type = token::TokenType::Eq;
            tok.literal = "==";
          },
          Some(_) => {
            tok.token_type = token::TokenType::Assign;
            tok.literal = "=";
          },
          None => ()
        }
      },
      Some(';') => {
        tok.token_type = token::TokenType::Semicolon;
        tok.literal = ";";
      },
      Some('+') => {
        tok.token_type = token::TokenType::Plus;
        tok.literal = "+";
      },
      Some('-') => {
        tok.token_type = token::TokenType::Minus;
        tok.literal = "-";
      },
      Some('/') => {
        tok.token_type = token::TokenType::Slash;
        tok.literal = "/";
      },
      Some('*') => {
        tok.token_type = token::TokenType::Asterisk;
        tok.literal = "*";
      },
      Some('!') => {
        match self.peek_char() {
          Some('=') => {
            self.read_char();
            tok.token_type = token::TokenType::NotEq;
            tok.literal = "!=";
          },
          Some(_) => {
            tok.token_type = token::TokenType::Bang;
            tok.literal = "!";
          },
          None => ()
        }
      },
      Some(',') => {
        tok.token_type = token::TokenType::Comma;
        tok.literal = ",";
      },
      Some('(') => {
        tok.token_type = token::TokenType::Lparen;
        tok.literal = "(";
      },
      Some(')') => {
        tok.token_type = token::TokenType::Rparen;
        tok.literal = ")";
      },
      Some('{') => {
        tok.token_type = token::TokenType::Lbrace;
        tok.literal = "{";
      },
      Some('}') => {
        tok.token_type = token::TokenType::Rbrace;
        tok.literal = "}";
      },
      Some('<') => {
        tok.token_type = token::TokenType::Lt;
        tok.literal = "<";
      },
      Some('>') => {
        tok.token_type = token::TokenType::Gt;
        tok.literal = ">";
      },
      Some(_) => {
        let ch = self.ch.unwrap();
        if is_letter(ch) {
          tok.literal = self.read_while(is_letter);
          tok.token_type = token::lookup_identifier(tok.literal);
          return tok;
        } else if is_digit(ch) {
          tok.literal = self.read_while(is_digit);
          tok.token_type = token::TokenType::Int;
          return tok;
        }
      },
      None => {
        tok.token_type = token::TokenType::Eof;
        tok.literal = ""; // TODO what's the correct literal representation of EOF??
      }
    };

    self.read_char();
    tok
  }

  fn read_char(&mut self) {
    self.ch = self.input.chars().nth(self.read_position);
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn peek_char(&self) -> Option<char> {
    self.input.chars().nth(self.read_position)
  }

  fn skip_whitespace(&mut self) {
    // TODO is there a cleaner, more idiomatic "rust-y" way to handle this?
    while let Some(ch) = self.ch {
      if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
        self.read_char();
      } else {
        break;
      }
    }
  }

  fn read_while(&mut self, pred: fn(char) -> bool) -> &str {
    let position = self.position;

    // TODO handle error? this is technically safe since I'm only calling
    // this from within a Some(_) pattern match, but doesn't hurt to handle the
    // error here.
    while pred(self.ch.unwrap()) {
      self.read_char();
    }

    &self.input[position..self.position]
  }
}

#[allow(dead_code)]
mod test {
  use super::*;

  struct TestCase<'a> {
    expected_type: token::TokenType,
    expected_literal: &'a str,
  }

  const INPUT: &str = r#"let    five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);

10 == 10;
10 != 9;

!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}
"#;

  // TODO handle non-ASCII chars/graphemes?
  #[test]
  fn test_next_token() {
    let test_cases = vec![
      TestCase{expected_type: token::TokenType::Let, expected_literal: "let"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "five"},
      TestCase{expected_type: token::TokenType::Assign, expected_literal: "="},
      TestCase{expected_type: token::TokenType::Int, expected_literal: "5"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},

      TestCase{expected_type: token::TokenType::Let, expected_literal: "let"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "ten"},
      TestCase{expected_type: token::TokenType::Assign, expected_literal: "="},
      TestCase{expected_type: token::TokenType::Int, expected_literal: "10"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},

      TestCase{expected_type: token::TokenType::Let, expected_literal: "let"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "add"},
      TestCase{expected_type: token::TokenType::Assign, expected_literal: "="},
      TestCase{expected_type: token::TokenType::Function, expected_literal: "fn"},
      TestCase{expected_type: token::TokenType::Lparen, expected_literal: "("},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "x"},
      TestCase{expected_type: token::TokenType::Comma, expected_literal: ","},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "y"},
      TestCase{expected_type: token::TokenType::Rparen, expected_literal: ")"},
      TestCase{expected_type: token::TokenType::Lbrace, expected_literal: "{"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "x"},
      TestCase{expected_type: token::TokenType::Plus, expected_literal: "+"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "y"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},
      TestCase{expected_type: token::TokenType::Rbrace, expected_literal: "}"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},

      TestCase{expected_type: token::TokenType::Let, expected_literal: "let"},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "result"},
      TestCase{expected_type: token::TokenType::Assign, expected_literal: "="},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "add"},
      TestCase{expected_type: token::TokenType::Lparen, expected_literal: "("},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "five"},
      TestCase{expected_type: token::TokenType::Comma, expected_literal: ","},
      TestCase{expected_type: token::TokenType::Ident, expected_literal: "ten"},
      TestCase{expected_type: token::TokenType::Rparen, expected_literal: ")"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},

      TestCase{expected_type: token::TokenType::Int, expected_literal: "10"},
      TestCase{expected_type: token::TokenType::Eq, expected_literal: "=="},
      TestCase{expected_type: token::TokenType::Int, expected_literal: "10"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},

      TestCase{expected_type: token::TokenType::Int, expected_literal: "10"},
      TestCase{expected_type: token::TokenType::NotEq, expected_literal: "!="},
      TestCase{expected_type: token::TokenType::Int, expected_literal: "9"},
      TestCase{expected_type: token::TokenType::Semicolon, expected_literal: ";"},
    ];
    let mut lexer = Lexer::new(INPUT);

    for test_case in test_cases {
      let tok = lexer.next_token();
      assert_eq!(tok.token_type, test_case.expected_type);
      assert_eq!(tok.literal, test_case.expected_literal);
    }
  }
}
