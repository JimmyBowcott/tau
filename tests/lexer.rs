use tau::{lexer::Lexer, token::{Token, TokenKind}};

#[test]
fn test_simple_tokens() {
    let input = "let x = 42;";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(TokenKind::Let, 0, 0),
        Token::new(TokenKind::Identifier("x".into()), 0, 0),
        Token::new(TokenKind::Equal, 0, 0),
        Token::new(TokenKind::Number(42.0), 0, 0),
        Token::new(TokenKind::Semicolon, 0, 0),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_numbers_and_identifiers() {
    let input = "mass = 1.5E-3; velocity = 3.14;";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(TokenKind::Identifier("mass".into()), 0, 0),
        Token::new(TokenKind::Equal, 0, 0),
        Token::new(TokenKind::Number(1.5e-3), 0, 0),
        Token::new(TokenKind::Semicolon, 0, 0),
        Token::new(TokenKind::Identifier("velocity".into()), 0, 0),
        Token::new(TokenKind::Equal, 0, 0),
        Token::new(TokenKind::Number(3.14), 0, 0),
        Token::new(TokenKind::Semicolon, 0, 0),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_symbols() {
    let input = "+ - * / ^ : ; ( ) { }";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(TokenKind::Plus, 0, 0),
        Token::new(TokenKind::Minus, 0, 0),
        Token::new(TokenKind::Star, 0, 0),
        Token::new(TokenKind::Slash, 0, 0),
        Token::new(TokenKind::Caret, 0, 0),
        Token::new(TokenKind::Colon, 0, 0),
        Token::new(TokenKind::Semicolon, 0, 0),
        Token::new(TokenKind::LParen, 0, 0),
        Token::new(TokenKind::RParen, 0, 0),
        Token::new(TokenKind::LBrace, 0, 0),
        Token::new(TokenKind::RBrace, 0, 0),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_keywords() {
    let input = "let print force";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(TokenKind::Let, 0, 0),
        Token::new(TokenKind::Print, 0, 0),
        Token::new(TokenKind::Identifier("force".into()), 0, 0),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_complex_expression() {
    let input = "let velocity: m/s = 10.0 / (2 * t);";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(TokenKind::Let, 0, 0),
        Token::new(TokenKind::Identifier("velocity".into()), 0, 0),
        Token::new(TokenKind::Colon, 0, 0),
        Token::new(TokenKind::Identifier("m".into()), 0, 0),
        Token::new(TokenKind::Slash, 0, 0),
        Token::new(TokenKind::Identifier("s".into()), 0, 0),
        Token::new(TokenKind::Equal, 0, 0),
        Token::new(TokenKind::Number(10.0), 0, 0),
        Token::new(TokenKind::Slash, 0, 0),
        Token::new(TokenKind::LParen, 0, 0),
        Token::new(TokenKind::Number(2.0), 0, 0),
        Token::new(TokenKind::Star, 0, 0),
        Token::new(TokenKind::Identifier("t".into()), 0, 0),
        Token::new(TokenKind::RParen, 0, 0),
        Token::new(TokenKind::Semicolon, 0, 0),
    ];

    assert_eq!(tokens, expected);
}
