use super::ast::Program;
use super::lexer::Lexer;
use super::token::Token;

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

    fn parse_program(&mut self) -> Option<&Program> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_statement() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;"#;

        let mut l = Lexer::new(String::from(input));
        let mut p = Parser::new(&mut l);
        let _program = p.parse_program().unwrap();
    }
}
