use super::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

struct Statement {}
impl Node for Statement {
    fn token_literal(&self) -> String {
        unimplemented!()
    }
}

struct Expression {}
impl Node for Expression {
    fn token_literal(&self) -> String {
        unimplemented!()
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
