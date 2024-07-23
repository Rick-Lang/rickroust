use crate::parser::AST;
use crate::lexer::Token;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, node: &AST) -> i32 {
        match node {
            AST::BinOp(left, op, right) => {
                let lhs = self.interpret(left);
                let rhs = self.interpret(right);
                match op {
                    Token::Plus => lhs + rhs,
                    Token::Minus => lhs - rhs,
                    Token::Star => lhs * rhs,
                    Token::Slash => lhs / rhs,
                    _ => panic!("Unexpected binary operator"),
                }
            }
            AST::Num(value) => *value,
            AST::Print(expr) => {
                let value = self.interpret(expr);
                println!("{}", value);
                value
            }
        }
    }
}
