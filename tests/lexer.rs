use tau::{lexer::Lexer, token::Token};

#[test]
fn test_simple_tokens() {
    let input = "let x = 42;";
    let tokens: Vec<_> = Lexer::new(input).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("x".into()),
            Token::Equal,
            Token::Number(42.0),
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_numbers_and_identifiers() {
    let input = "mass = 1.5E-3; velocity = 3.14;";
    let tokens: Vec<_> = Lexer::new(input).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Identifier("mass".into()),
            Token::Equal,
            Token::Number(1.5e-3),
            Token::Semicolon,
            Token::Identifier("velocity".into()),
            Token::Equal,
            Token::Number(3.14),
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_symbols() {
    let input = "+ - * / ^ : ; ( ) { }";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::Plus, Token::Minus, Token::Star, Token::Slash, Token::Caret,
        Token::Colon, Token::Semicolon, Token::LParen, Token::RParen,
        Token::LBrace, Token::RBrace,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_keywords() {
    let input = "let print force";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::Let,
        Token::Print,
        Token::Identifier("force".into()),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_complex_expression() {
    let input = "let velocity: m/s = 10.0 / (2 * t);";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::Let,
        Token::Identifier("velocity".into()),
        Token::Colon,
        Token::Identifier("m".into()),
        Token::Slash,
        Token::Identifier("s".into()),
        Token::Equal,
        Token::Number(10.0),
        Token::Slash,
        Token::LParen,
        Token::Number(2.0),
        Token::Star,
        Token::Identifier("t".into()),
        Token::RParen,
        Token::Semicolon,
    ];

    assert_eq!(tokens, expected);
}
