#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(String),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}

impl Token {
    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            _ => Token::IDENT(ident),
        }
    }
}
