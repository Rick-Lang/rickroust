use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum AST {
    BinOp(Box<AST>, Token, Box<AST>),
    Num(i32),
    Print(Box<AST>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.current_token = parser.lexer.get_next_token();
        parser
    }

    fn eat(&mut self, token: Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&token) {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token: {:?}, expected: {:?}", self.current_token, token);
        }
    }

    fn factor(&mut self) -> AST {
        match self.current_token {
            Token::Number(value) => {
                self.eat(Token::Number(value));
                AST::Num(value)
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let node = self.expr();
                self.eat(Token::RParen);
                node
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn term(&mut self) -> AST {
        let mut node = self.factor();
        while let Token::Star | Token::Slash = self.current_token {
            let token = self.current_token.clone();
            self.eat(token.clone());
            node = AST::BinOp(Box::new(node), token, Box::new(self.factor()));
        }
        node
    }

    fn expr(&mut self) -> AST {
        let mut node = self.term();
        while let Token::Plus | Token::Minus = self.current_token {
            let token = self.current_token.clone();
            self.eat(token.clone());
            node = AST::BinOp(Box::new(node), token, Box::new(self.term()));
        }
        node
    }

    pub fn parse(&mut self) -> AST {
        if let Token::Print = self.current_token {
            self.eat(Token::Print);
            AST::Print(Box::new(self.expr()))
        } else {
            self.expr()
        }
    }
}
