use downcast_rs::Downcast;

use super::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + Downcast + AsStatement {
    fn statement_node(&self);
}
impl_downcast!(Statement);

pub trait Expression: Node + Downcast {
    fn expression_node(&self);
}
impl_downcast!(Expression);

//
// Program
//

pub struct Program {
    pub statements: Vec<Box<Statement>>,
}
impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
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

//
// Identifier
//

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

//
// Let
//

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    //pub value: Option<Box<Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

//
// Upcast
//

pub trait AsStatement {
    fn as_statement(&self) -> &Statement;
    fn as_box_statement(self) -> Box<Statement>;
}
impl<T: Statement> AsStatement for T {
    fn as_statement(&self) -> &Statement {
        self
    }

    fn as_box_statement(self) -> Box<Statement> {
        Box::new(self)
    }
}
