#[derive(Debug)]
pub enum TokenType {
    // Single Param Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
}

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
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
    pub fn read_tokenize(&mut self, v: Vec<u8>) -> Result<(), ()> {
        self.src = v;

        for b in &self.src {
            match b {
                b'(' => self.tkns.push(Token {
                    ty: TokenType::LeftParen,
                }),
                b')' => self.tkns.push(Token {
                    ty: TokenType::RightParen,
                }),
                b'}' => self.tkns.push(Token {
                    ty: TokenType::RightBrace,
                }),
                b'{' => self.tkns.push(Token {
                    ty: TokenType::LeftBrace,
                }),
                b';' => self.tkns.push(Token {
                    ty: TokenType::Semicolon,
                }),
                _ => (),
            }
        }

        Ok(())
    }

    pub fn print_tkns(&mut self) -> Result<(), ()> {
        println!("{:?}", self.tkns);
        Ok(())
    }
}
