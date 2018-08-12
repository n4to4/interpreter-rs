#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,   // 1343456

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

use self::TokenType::*;

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        _ => IDENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (ASSIGN, "="),
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
        ];

        let mut l = Lexer::new(String::from(input));
        for (to, le) in tests.iter() {
            let tok = l.next_token();

            assert_eq!(tok.typ, *to);
            assert_eq!(tok.literal, *le);
        }
    }

    #[test]
    fn next_token2() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
"#;

        let tests = vec![
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            (BANG, "!"),
            (MINUS, "-"),
            (SLASH, "/"),
            (ASTERISK, "*"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (GT, ">"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (EOF, ""),
        ];

        let mut l = Lexer::new(String::from(input));
        for (to, le) in tests.iter() {
            let tok = l.next_token();

            assert_eq!(tok.typ, *to);
            assert_eq!(tok.literal, *le);
        }
    }
}
