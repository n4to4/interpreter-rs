pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

use token::{TokenType::*, *};

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some(ch @ '=') => new_token(ASSIGN, ch),
            Some(ch @ ';') => new_token(SEMICOLON, ch),
            Some(ch @ '(') => new_token(LPAREN, ch),
            Some(ch @ ')') => new_token(RPAREN, ch),
            Some(ch @ ',') => new_token(COMMA, ch),
            Some(ch @ '+') => new_token(PLUS, ch),
            Some(ch @ '{') => new_token(LBRACE, ch),
            Some(ch @ '}') => new_token(RBRACE, ch),
            Some(_) | None => Token {
                typ: EOF,
                literal: String::from(""),
            },
        };
        self.read_char();
        tok
    }
}

fn new_token(typ: TokenType, ch: char) -> Token {
    Token {
        typ: typ,
        literal: ch.to_string(),
    }
}
