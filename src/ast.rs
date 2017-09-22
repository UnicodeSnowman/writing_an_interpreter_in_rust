use token;

trait Node {
    fn token_literal(&self) -> &str;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<Statement>>
}

impl Program {
    fn token_literal(&self) -> &str {
        match self.statements.len() {
            0 => "",
            _ => self.statements[0].token_literal()
        }
    }
}

// Statements...
pub struct LetStatement<'a> {
    token: token::Token<'a>,
    name: Identifier<'a>,
    value: Box<Expression>
}

impl<'a> Node for LetStatement<'a> {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}

impl<'a> Statement for LetStatement<'a> {
    fn statement_node(&self) {}
}

struct Identifier<'a> {
    token: token::Token<'a>,
    value: &'a str
}

impl<'a> Node for Identifier<'a> {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}

impl<'a> Statement for Identifier<'a> {
    fn statement_node(&self) {}
}
