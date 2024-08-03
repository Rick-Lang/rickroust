#[derive(Debug, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Print,
    LParen,
    RParen,
    EOF,
}

pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        while let Some(c) = self.current_char {
            match c {
                '0'..='9' => {
                    return match self.number() {
                        Some(n) => Ok(n),
                        _ => Err(format!("Expected digit, found: `{:?}`", self.current_char)),
                    }
                }
                '+' => {
                    self.advance();
                    return Ok(Token::Plus);
                }
                '-' => {
                    self.advance();
                    return Ok(Token::Minus);
                }
                '*' => {
                    self.advance();
                    return Ok(Token::Star);
                }
                '/' => {
                    self.advance();
                    return Ok(Token::Slash);
                }
                '(' => {
                    self.advance();
                    return Ok(Token::LParen);
                }
                ')' => {
                    self.advance();
                    return Ok(Token::RParen);
                }
                'p' => {
                    self.advance();
                    if self.current_char == Some('r') {
                        self.advance();
                        if self.current_char == Some('i') {
                            self.advance();
                            if self.current_char == Some('n') {
                                self.advance();
                                if self.current_char == Some('t') {
                                    self.advance();
                                    return Ok(Token::Print);
                                }
                            }
                        }
                    }
                    return Err(format!(
                        "Expected \"print\", found: `{:?}`",
                        self.current_char
                    ));
                }
                // whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                    continue;
                }
                _ => return Err(format!("Unexpected character: {}", c)),
            }
        }
        Ok(Token::EOF)
    }

    fn number(&mut self) -> Option<Token> {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if !c.is_ascii_digit() {
                break;
            }
            // Order is irrelevant,
            // but (potentially) allocating first
            // is a "fail-fast" strategy.
            // So the program only invests time doing stuff,
            // if there's enough memory
            result.push(c);
            self.advance();
        }
        result.parse::<i32>().ok().map(Token::Number)
    }
}
