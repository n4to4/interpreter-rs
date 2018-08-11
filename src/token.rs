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

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;

    #[test]
    fn next_token() {
        use self::TokenType::*;

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
}
