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
    LET,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    IF,
    ELSE,
    TRUE,
    FALSE,
    RETURN,
    EQ,
    NOTEQ,
}

impl Token {
    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "return" => Token::RETURN,
            _ => Token::IDENT(ident),
        }
    }
}
