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
    pub fn new(lexer: Lexer<'a>) -> Result<Self, String> {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.current_token = parser.lexer.get_next_token()?;
        Ok(parser)
    }

    #[must_use = "Don't ignore err!"]
    fn eat(&mut self, token: Token) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token) != std::mem::discriminant(&token) {
            return Err(format!(
                "Unexpected token: `{:?}`, expected: `{:?}`",
                self.current_token, token
            ));
        }
        self.current_token = self.lexer.get_next_token()?;
        Ok(())
    }

    fn factor(&mut self) -> Result<AST, String> {
        match self.current_token {
            Token::Number(value) => {
                self.eat(Token::Number(value))?;
                Ok(AST::Num(value))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let node = self.expr()?;
                self.eat(Token::RParen)?;
                Ok(node)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn term(&mut self) -> Result<AST, String> {
        let mut node = self.factor()?;
        while let Token::Plus | Token::Minus | Token::Star | Token::Slash = self.current_token {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = AST::BinOp(Box::new(node), token, Box::new(self.factor()?));
        }
        Ok(node)
    }

    fn expr(&mut self) -> Result<AST, String> {
        let mut node = self.term()?;
        while let Token::Plus | Token::Minus = self.current_token {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = AST::BinOp(Box::new(node), token, Box::new(self.term()?));
        }
        Ok(node)
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        if let Token::Print = self.current_token {
            self.eat(Token::Print)?;
            Ok(AST::Print(Box::new(self.expr()?)))
        } else {
            self.expr()
        }
    }
}
