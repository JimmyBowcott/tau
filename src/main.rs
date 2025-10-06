use lexer::Lexer;

mod lexer;

fn main() {
    let mut lexer = Lexer::new("let velocity: m/s = 10.0;");
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}
