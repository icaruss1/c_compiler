mod lexer;

use std::fs;

fn main() -> Result<(), ()> {
    let bytes = fs::read("./c_files/newlines.c").unwrap();

    let mut lex = lexer::Lexer::default();
    lex.lex(bytes)?;

    lex.print_tkns()?;

    Ok(())
}
