#[derive(Debug,PartialEq)]
pub enum TokenType {
  Illegal,
  Eof,
  Ident,
  Int,
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  Comma,
  Semicolon,
  Lt,
  Gt,
  Eq,
  NotEq,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return
}

impl TokenType {
  pub fn value(&self) -> &str {
    match *self {
      TokenType::Illegal   => "ILLEGAL",
      TokenType::Eof       => "EOF",
      TokenType::Ident     => "IDENT",
      TokenType::Int       => "INT",
      TokenType::Assign    => "=",
      TokenType::Plus      => "+",
      TokenType::Minus     => "-",
      TokenType::Bang      => "!",
      TokenType::Asterisk  => "*",
      TokenType::Slash     => "/",
      TokenType::Comma     => ",",
      TokenType::Semicolon => ";",
      TokenType::Lt        => "<",
      TokenType::Gt        => ">",
      TokenType::Eq        => "==",
      TokenType::NotEq     => "!=",
      TokenType::Lparen    => "(",
      TokenType::Rparen    => ")",
      TokenType::Lbrace    => "{",
      TokenType::Rbrace    => "}",
      TokenType::Function  => "FUNCTION",
      TokenType::Let       => "LET",
      TokenType::True      => "TRUE",
      TokenType::False     => "FALSE",
      TokenType::If        => "IF",
      TokenType::Else      => "ELSE",
      TokenType::Return    => "RETURN",
    }
  }
}

pub fn lookup_identifier(ident: &str) -> TokenType {
  match ident {
    "fn" => TokenType::Function,
    "let" => TokenType::Let,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "return" => TokenType::Return,
    _ => TokenType::Ident,
  }
}

pub struct Token<'a> {
  pub token_type: TokenType,
  pub literal: &'a str
}

