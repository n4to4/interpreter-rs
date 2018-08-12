pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

use token::{self, Token, TokenType, TokenType::*};

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
        self.skip_whitespace();

        let tok = match self.ch {
            Some(ch @ '=') => new_token(ASSIGN, ch),
            Some(ch @ '+') => new_token(PLUS, ch),
            Some(ch @ '-') => new_token(MINUS, ch),
            Some(ch @ '!') => new_token(BANG, ch),
            Some(ch @ '/') => new_token(SLASH, ch),
            Some(ch @ '*') => new_token(ASTERISK, ch),
            Some(ch @ '<') => new_token(LT, ch),
            Some(ch @ '>') => new_token(GT, ch),
            Some(ch @ ';') => new_token(SEMICOLON, ch),
            Some(ch @ '(') => new_token(LPAREN, ch),
            Some(ch @ ')') => new_token(RPAREN, ch),
            Some(ch @ ',') => new_token(COMMA, ch),
            Some(ch @ '{') => new_token(LBRACE, ch),
            Some(ch @ '}') => new_token(RBRACE, ch),
            Some(ch @ _) => {
                if is_letter(ch) {
                    let lit = self.read_identifier();
                    return Token {
                        typ: token::lookup_ident(&lit),
                        literal: lit,
                    };
                } else if is_digit(ch) {
                    return Token {
                        typ: INT,
                        literal: self.read_number(),
                    };
                } else {
                    new_token(ILLEGAL, ch)
                }
            }
            None => Token {
                typ: EOF,
                literal: String::from(""),
            },
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.map_or(false, is_letter) {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.map_or(false, is_digit) {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map_or(false, |ch| {
            ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
        }) {
            self.read_char();
        }
    }
}

fn new_token(typ: TokenType, ch: char) -> Token {
    Token {
        typ: typ,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
