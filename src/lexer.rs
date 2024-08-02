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

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            match c {
                '0'..='9' => return self.number(),
                '+' => {
                    self.advance();
                    return Token::Plus;
                }
                '-' => {
                    self.advance();
                    return Token::Minus;
                }
                '*' => {
                    self.advance();
                    return Token::Star;
                }
                '/' => {
                    self.advance();
                    return Token::Slash;
                }
                '(' => {
                    self.advance();
                    return Token::LParen;
                }
                ')' => {
                    self.advance();
                    return Token::RParen;
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
                                    return Token::Print;
                                }
                            }
                        }
                    }
                    panic!("Expected \"print\", found: `{:?}`", self.current_char);
                }
                // whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                    continue;
                }
                _ => panic!("Unexpected character: {}", c),
            }
        }
        Token::EOF
    }

    fn number(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(result.parse::<i32>().unwrap())
    }
}
