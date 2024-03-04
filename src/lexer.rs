//enums containing all labels of all Keywords
#[derive(Debug, PartialEq)]
pub enum Keyword {
    Return,
    Int,
}

// enum containing the Literals along with initialising values
#[derive(Debug, PartialEq)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

// enum containing all tokentypes, everythign that can be
// recognised by lexer should be contained here
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single Length Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,

    // Literals and Keywords are Nested enums
    Literal(Literal),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>, //for having the lexed representations
}

pub struct Lexer {
    // src will be for now a stream of bytes coming from a file or string.into_bytes()
    src: Vec<u8>,
    tkns: Vec<Token>,
}

impl Default for Lexer {
    fn default() -> Lexer {
        Lexer {
            src: Vec::new(),
            tkns: Vec::new(),
        }
    }
}

impl Lexer {
    pub fn lex(&mut self, v: Vec<u8>) -> Result<(), ()> {
        self.src = v;

        for b in &self.src {
            // matching all the single length tokens
            match b {
                b'(' => self.tkns.push(Token {
                    lexeme: vec![b'('],
                    ty: TokenType::LeftParen,
                }),
                b')' => self.tkns.push(Token {
                    lexeme: vec![b')'],
                    ty: TokenType::RightParen,
                }),
                b'}' => self.tkns.push(Token {
                    lexeme: vec![b'}'],
                    ty: TokenType::RightBrace,
                }),
                b'{' => self.tkns.push(Token {
                    lexeme: vec![b'{'],
                    ty: TokenType::LeftBrace,
                }),
                b';' => self.tkns.push(Token {
                    lexeme: vec![b';'],
                    ty: TokenType::Semicolon,
                }),
                // match the literals under all the rest
                b => {
                    if b.is_ascii_digit() {
                        ()
                    }
                }
            }
        }

        Ok(())
    }

    pub fn print_tkns(&mut self) -> Result<(), ()> {
        println!("{:?}", self.tkns);
        Ok(())
    }
}

fn is_decimal_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

//b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'0'

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn newlines_testcase() -> Result<(), ()> {
        let bytes: Vec<u8> = String::from("\nint\nmain\n(\n)\n{\nreturn\n0\n;\n}").into_bytes();
        let mut lex = Lexer::default();

        lex.lex(bytes)?;

        assert_eq!(
            lex.tkns,
            [
                Token {
                    lexeme: vec![b'('],
                    ty: TokenType::LeftParen
                },
                Token {
                    lexeme: vec![b')'],
                    ty: TokenType::RightParen
                },
                Token {
                    lexeme: vec![b'{'],
                    ty: TokenType::LeftBrace
                },
                Token {
                    lexeme: vec![b';'],
                    ty: TokenType::Semicolon
                },
                Token {
                    lexeme: vec![b'}'],
                    ty: TokenType::RightBrace
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn integer_parsing() -> Result<(), ()> {
        let bytes = String::from("1").into_bytes();
        let mut lex = Lexer::default();

        lex.lex(bytes)?;

        assert_eq!(
            lex.tkns,
            [Token {
                lexeme: vec![1],
                ty: TokenType::Literal(Literal::Number(1.0))
            }],
        );

        Ok(())
    }
}
