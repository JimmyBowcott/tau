use tau::{lexer::Lexer, token::Token};

#[test]
fn test_simple_tokens() {
    let input = "let x = 42;";
    let tokens: Vec<_> = Lexer::new(input).collect();

    assert_eq!(
        tokens,
        vec![
            Token::new(Let),
            Token::new(Identifier(r)),
            Token::new(Equal),
            Token::new(Number(r)),
            Token::new(Semicolon),
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
            Token::new(Identifier(r)),
            Token::new(Equal),
            Token::new(Number(r)),
            Token::new(Semicolon),
            Token::new(Identifier(r)),
            Token::new(Equal),
            Token::new(Number(r)),
            Token::new(Semicolon),
        ]
    );
}

#[test]
fn test_symbols() {
    let input = "+ - * / ^ : ; ( ) { }";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(Plus), Token::new(Minus), Token::new(Star), Token::new(Slash), Token::new(Caret),
        Token::new(Colon), Token::new(Semicolon), Token::new(LParen), Token::new(RParen),
        Token::new(LBrace), Token::new(RBrace),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_keywords() {
    let input = "let print force";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(Let),
        Token::new(Print),
        Token::new(Identifier(r)),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_complex_expression() {
    let input = "let velocity: m/s = 10.0 / (2 * t);";
    let tokens: Vec<_> = Lexer::new(input).collect();

    let expected = vec![
        Token::new(Let),
        Token::new(Identifier(r)),
        Token::new(Colon),
        Token::new(Identifier(r)),
        Token::new(Slash),
        Token::new(Identifier(r)),
        Token::new(Equal),
        Token::new(Number(r)),
        Token::new(Slash),
        Token::new(LParen),
        Token::new(Number(r)),
        Token::new(Star),
        Token::new(Identifier(r)),
        Token::new(RParen),
        Token::new(Semicolon),
    ];

    assert_eq!(tokens, expected);
}
