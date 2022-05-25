use crate::{token::Token, utils::Util};

struct Lexer<'a> {
    input: &'a String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a String) -> Lexer<'a> {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = match self.input.chars().nth(self.read_position) {
                Some(ch) => ch,
                None => '\0',
            }
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn read(&mut self, f: fn(char) -> bool) -> String {
        let position = self.position;
        while f(self.ch) {
            self.read_char();
        }

        return String::from(&self.input[position..self.position]);
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                _ => break,
            }
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }

        return match self.input.chars().nth(self.read_position) {
            Some(ch) => ch,
            None => '\0',
        };
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        println!("ATTENTION HERE");
        println!("VALUE: {}", self.ch);
        let token = match self.ch {
            '=' => {
                let peek_char = self.peek_char();
                self.read_char();
                if peek_char == '=' {
                    self.read_char();
                    return Token::EQ;
                }
                return Token::ASSIGN;
            }
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '-' => Token::MINUS,
            '!' => {
                let peek_char = self.peek_char();
                self.read_char();
                if peek_char == '=' {
                    self.read_char();
                    return Token::NOTEQ;
                }
                return Token::BANG;
            }
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            '\0' => Token::EOF,
            'A'..='Z' | 'a'..='z' => {
                let identifier = self.read(Util::is_letter);
                return Token::lookup_ident(identifier);
            }
            '0'..='9' => return Token::INT(self.read(Util::is_number)),
            _ => Token::ILLEGAL,
        };

        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token::{self, *};
    use std::vec;

    fn test_tokens(expected_result: Vec<Token>, mut lexer: Lexer) {
        for expected_token in expected_result {
            let parsed_token = lexer.next_token();

            assert_eq!(
                parsed_token, expected_token,
                "testing the equality of token: {:?} and parsedToken: {:?}",
                expected_token, parsed_token
            );

            match (&parsed_token, &expected_token) {
                (INT(x), INT(y)) => assert_eq!(
                    x, y,
                    "testing the equality of the content of the token {:?} and parsedToken: {:?}",
                    expected_token, parsed_token
                ),
                _ => assert!(true),
            };
        }
    }

    #[test]
    fn test_next_token_success() {
        let input = String::from("=(){},;");
        let expected_result = vec![
            ASSIGN, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF,
        ];

        let lexer = Lexer::new(&input);

        test_tokens(expected_result, lexer)
    }

    #[test]
    fn test_next_token_valid_code() {
        let input = String::from(
            "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ",
        );
        let expected_result = vec![
            LET,
            IDENT(String::from("five")),
            ASSIGN,
            INT(String::from("5")),
            SEMICOLON,
            LET,
            IDENT(String::from("ten")),
            ASSIGN,
            INT(String::from("10")),
            SEMICOLON,
            LET,
            IDENT(String::from("add")),
            ASSIGN,
            FUNCTION,
            LPAREN,
            IDENT(String::from("x")),
            COMMA,
            IDENT(String::from("y")),
            RPAREN,
            LBRACE,
            IDENT(String::from("x")),
            PLUS,
            IDENT(String::from("y")),
            SEMICOLON,
            RBRACE,
            SEMICOLON,
            LET,
            IDENT(String::from("result")),
            ASSIGN,
            IDENT(String::from("add")),
            LPAREN,
            IDENT(String::from("five")),
            COMMA,
            IDENT(String::from("ten")),
            RPAREN,
            SEMICOLON,
            BANG,
            MINUS,
            SLASH,
            ASTERISK,
            INT(String::from("5")),
            SEMICOLON,
            INT(String::from("5")),
            LT,
            INT(String::from("10")),
            GT,
            INT(String::from("5")),
            SEMICOLON,
            IF,
            LPAREN,
            INT(String::from("5")),
            LT,
            INT(String::from("10")),
            RPAREN,
            LBRACE,
            RETURN,
            TRUE,
            SEMICOLON,
            RBRACE,
            ELSE,
            LBRACE,
            RETURN,
            FALSE,
            SEMICOLON,
            RBRACE,
            INT(String::from("10")),
            EQ,
            INT(String::from("10")),
            SEMICOLON,
            INT(String::from("10")),
            NOTEQ,
            INT(String::from("9")),
            SEMICOLON,
            EOF,
        ];

        let lexer = Lexer::new(&input);

        test_tokens(expected_result, lexer)
    }
}
