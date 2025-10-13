use crate::token::Token;

pub struct Lexer {
    position: usize,
    chars: Vec<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            position: 0,
            chars: input.chars().collect(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.chars.len() {
            None
        } else {
            let c = self.peek();
            self.position += 1;
            c
        }
    }

    fn next_word(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self, c: char) -> String {
        let mut res = String::new();
        res.push(c);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '-' || c == '+' {
                res.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        res
    }

    fn scan_identifier(&mut self, c: char) -> String {
        let mut res = String::new();
        res.push(c);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                res.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        res
    }

    fn match_identifier(&self, identifier: String) -> Option<Token> {
        match identifier.as_str() {
            "let" => Some(Token::Let),
            "const" => Some(Token::Const),
            "fn" => Some(Token::Fn),
            "return" => Some(Token::Return),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "for" => Some(Token::For),
            "while" => Some(Token::While),
            "print" => Some(Token::Print),
            _ => {
                Some(Token::Identifier(identifier))
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.next_word();
        let ch = self.advance()?;

        match ch {
            ':' => Some(Token::Colon),
            '=' => Some(Token::Equal),
            ';' => Some(Token::Semicolon),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '.' => Some(Token::Dot),
            '/' => Some(Token::Slash),
            '^' => Some(Token::Caret),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            c if c.is_ascii_digit() => {
                let num = self.scan_number(c);
                Some(Token::Number(num.parse().unwrap()))
            }
            c if c.is_alphabetic() => {
                let identifier = self.scan_identifier(c);
                self.match_identifier(identifier)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let lexer = Lexer::new("abc");
        assert_eq!(lexer.peek(), Some('a'));
    }

    #[test]
    fn test_advance() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.advance(), Some('a'));
        assert_eq!(lexer.advance(), Some('b'));
        assert_eq!(lexer.advance(), Some('c'));
        assert_eq!(lexer.advance(), None);
    }

    #[test]
    fn test_next_word() {
        let mut lexer = Lexer::new("   abc");
        lexer.next_word();
        assert_eq!(lexer.peek(), Some('a'));
    }

    #[test]
    fn test_scan_number_integer() {
        let mut lexer = Lexer::new("42");
        lexer.advance();
        assert_eq!(lexer.scan_number('4'), "42");
    }

    #[test]
    fn test_scan_number_float() {
        let mut lexer = Lexer::new("3.14");
        lexer.advance();
        assert_eq!(lexer.scan_number('3'), "3.14");
    }

    #[test]
    fn test_scan_number_scientific() {
        let mut lexer = Lexer::new("1.5e-3");
        lexer.advance();
        assert_eq!(lexer.scan_number('1'), "1.5e-3");
    }

    #[test]
    fn test_scan_identifier_simple() {
        let mut lexer = Lexer::new("velocity");
        lexer.advance();
        assert_eq!(lexer.scan_identifier('v'), "velocity");
    }

    #[test]
    fn test_scan_identifier_with_underscore() {
        let mut lexer = Lexer::new("mass_density");
        lexer.advance();
        assert_eq!(lexer.scan_identifier('m'), "mass_density");
    }

    #[test]
    fn test_match_identifier_keywords() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.match_identifier("let".into()), Some(Token::Let));
        assert_eq!(lexer.match_identifier("print".into()), Some(Token::Print));
    }

    #[test]
    fn test_match_identifier_non_keyword() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.match_identifier("force".into()), Some(Token::Identifier("force".into())));
    }

    #[test]
    fn test_next_token_single_char() {
        let mut lexer = Lexer::new("= ; + - * / ^ : ( ) { }");
        let expected = vec![
            Token::Equal, Token::Semicolon, Token::Plus, Token::Minus, Token::Star,
            Token::Slash, Token::Caret, Token::Colon, Token::LParen, Token::RParen,
            Token::LBrace, Token::RBrace,
        ];

        for tok in expected {
            assert_eq!(lexer.next_token(), Some(tok));
        }
    }

    #[test]
    fn test_next_token_number() {
        let mut lexer = Lexer::new("42 3.14 1.5e-3");
        assert_eq!(lexer.next_token(), Some(Token::Number(42.0)));
        assert_eq!(lexer.next_token(), Some(Token::Number(3.14)));
        assert_eq!(lexer.next_token(), Some(Token::Number(1.5e-3)));
    }

    #[test]
    fn test_next_token_identifier() {
        let mut lexer = Lexer::new("let force print");
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("force".into())));
        assert_eq!(lexer.next_token(), Some(Token::Print));
    }
}
