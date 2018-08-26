use ast::{self, AsStatement, Program};
use lexer::Lexer;
use token::{self, Token};

struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(l: &'a mut Lexer) -> Self {
        let tok1 = l.next_token().clone();
        let tok2 = l.next_token().clone();

        Parser {
            l: l,
            cur_token: tok1,
            peek_token: tok2,
        }
    }

    fn next_token(&mut self) {
        let tok = self.peek_token.clone();
        let next = self.l.next_token();

        self.cur_token = tok;
        self.peek_token = next;
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = ast::Program::new();

        while self.cur_token.typ != token::TokenType::EOF {
            println!("cur_token: {:?}", self.cur_token);

            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<ast::Statement>> {
        match self.cur_token.typ {
            token::TokenType::LET => self.parse_let_statement().map(|x| x.as_box_statement()),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let let_token = self.cur_token.clone();

        if !self.expect_peek(token::TokenType::IDENT) {
            return None;
        }

        let ident = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        let stmt = ast::LetStatement {
            token: let_token,
            name: ident,
            //value: None,
        };

        if !self.expect_peek(token::TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn cur_token_is(&self, t: token::TokenType) -> bool {
        self.cur_token.typ == t
    }
    fn peek_token_is(&self, t: token::TokenType) -> bool {
        self.peek_token.typ == t
    }
    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::*;

    use std::any::Any;

    fn print_if_string(s: &Any) {
        if let Some(string) = s.downcast_ref::<String>() {
            println!("It's a string({}): '{}'", string.len(), string);
        } else {
            println!("Not a string...");
        }
    }

    #[test]
    fn let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;"#;

        let mut l = Lexer::new(String::from(input));
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().expect("parse_program() failed");
        assert_eq!(program.statements.len(), 3);

        for (i, ident) in vec!["x", "y", "foobar"].iter().enumerate() {
            let stmt = &program.statements[i];
            assert_let_statement(stmt, &ident);
        }
    }

    fn assert_let_statement(s: &Box<Statement>, name: &str) {
        assert_eq!(s.token_literal(), "let");
        let stmt = s
            .downcast_ref::<ast::LetStatement>()
            .expect("downcast as LetStatement");

        assert_eq!(stmt.name.value, name);
        assert_eq!(stmt.name.token_literal(), name);
    }
}
