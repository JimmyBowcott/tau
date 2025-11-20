use crate::token::{Token, TokenKind};

pub struct Lexer {
    position: usize,
    line: usize,
    column: usize,
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
            line: 1,
            column: 0,
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
            self.set_line_col(c);
            c
        }
    }

    fn set_line_col(&mut self, c: Option<char>) {
        if let Some(chr) = c {
            if chr == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.peek() {
            self.advance();
            if c == '\n' {
                break;
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '#' {
                self.skip_line();
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

    fn match_identifier(&self, identifier: String) -> Option<TokenKind> {
        match identifier.as_str() {
            "let" => Some(TokenKind::Let),
            "const" => Some(TokenKind::Const),
            "fn" => Some(TokenKind::Fn),
            "return" => Some(TokenKind::Return),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "for" => Some(TokenKind::For),
            "while" => Some(TokenKind::While),
            "print" => Some(TokenKind::Print),
            _ => Some(TokenKind::Identifier(identifier)),
        }
    }

    fn lex_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.advance() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(next) = self.advance() {
                        s.push(match next {
                            'n' => '\n',
                            't' => '\t',
                            '"' => '"',
                            '\\' => '\\',
                            other => other,
                        });
                    }
                }
                other => s.push(other),
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace_and_comments();
        let ch = self.advance()?;

        if let Some(kind) = match ch {
            ':' => Some(TokenKind::Colon),
            '=' => Some(TokenKind::Equal),
            ';' => Some(TokenKind::Semicolon),
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Star),
            '.' => Some(TokenKind::Dot),
            '/' => Some(TokenKind::Slash),
            '^' => Some(TokenKind::Caret),
            '(' => Some(TokenKind::LParen),
            ')' => Some(TokenKind::RParen),
            '{' => Some(TokenKind::LBrace),
            '}' => Some(TokenKind::RBrace),
            '"' => {
                let s = self.lex_string();
                Some(TokenKind::String(s))
            }
            c if c.is_ascii_digit() => {
                let num = self.scan_number(c);
                Some(TokenKind::Number(num.parse().unwrap()))
            }
            c if c.is_alphabetic() => {
                let identifier = self.scan_identifier(c);
                self.match_identifier(identifier)
            }
            _ => None,
        } {
            Some(Token::new(kind, self.line, self.column))
        } else {
            None
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
        lexer.skip_whitespace_and_comments();
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
        assert_eq!(lexer.match_identifier("let".into()), Some(TokenKind::Let));
        assert_eq!(
            lexer.match_identifier("print".into()),
            Some(TokenKind::Print)
        );
    }

    #[test]
    fn test_match_identifier_non_keyword() {
        let lexer = Lexer::new("");
        assert_eq!(
            lexer.match_identifier("force".into()),
            Some(TokenKind::Identifier("force".into()))
        );
    }

    fn check_tokens(input: &str, expected: Vec<TokenKind>) {
        let mut lexer = Lexer::new(input);
        for kind in expected {
            let recieved = lexer.next_token().expect("Expected a token");
            assert_eq!(recieved.kind, kind);
        }
    }

    #[test]
    fn next_token_single_char() {
        let expected = vec![
            TokenKind::Equal,
            TokenKind::Semicolon,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Caret,
            TokenKind::Colon,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::RBrace,
        ];
        check_tokens("= ; + - * / ^ : ( ) { }", expected);
    }

    #[test]
    fn next_token_numbers() {
        check_tokens(
            "42 3.14 1.5e-3",
            vec![
                TokenKind::Number(42.0),
                TokenKind::Number(3.14),
                TokenKind::Number(1.5e-3),
            ],
        );
    }

    #[test]
    fn next_token_identifiers() {
        check_tokens(
            "let force print",
            vec![
                TokenKind::Let,
                TokenKind::Identifier("force".into()),
                TokenKind::Print,
            ],
        );
    }

    #[test]
    fn skips_comments() {
        check_tokens(
            r#"# full line comment
            1 2 3 # This is a comment"#,
            vec![
                TokenKind::Number(1.0),
                TokenKind::Number(2.0),
                TokenKind::Number(3.0),
            ],
        );
    }

    #[test]
    fn test_string_literals() {
        check_tokens(
            r#"let s = "hello world"; print("line\nbreak");"#,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("s".into()),
                TokenKind::Equal,
                TokenKind::String("hello world".into()),
                TokenKind::Semicolon,
                TokenKind::Print,
                TokenKind::LParen,
                TokenKind::String("line\nbreak".into()),
                TokenKind::RParen,
                TokenKind::Semicolon,
            ],
        );
    }

    #[test]
    fn test_comment_inside_string() {
        check_tokens(
            r#"let s = "this is # not a comment";"#,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("s".into()),
                TokenKind::Equal,
                TokenKind::String("this is # not a comment".into()),
                TokenKind::Semicolon,
            ],
        );
    }

    #[test]
    fn test_escape_sequences() {
        check_tokens(
            r#"let s = "line1\nline2\tend\"quote\"";"#,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("s".into()),
                TokenKind::Equal,
                TokenKind::String("line1\nline2\tend\"quote\"".into()),
                TokenKind::Semicolon,
            ],
        );
    }
}
