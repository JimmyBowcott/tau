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
