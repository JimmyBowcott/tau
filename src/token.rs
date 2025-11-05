#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Const,
    Fn,
    Return,
    If,
    Else,
    For,
    While,
    Print,
    Identifier(String),
    Number(f64),
    Colon,
    Equal,
    Semicolon,
    Plus,
    Minus,
    Star,
    Dot,
    Slash,
    Caret,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self {
            kind,
            line,
            column,
        }
    }
}
