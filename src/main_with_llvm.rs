use std::collections::HashMap;
use std::io::{self, Write};
use std::str::Chars;

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};

#[derive(Debug)]
enum Token {
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

struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
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

    fn get_next_token(&mut self) -> Token {
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
                }
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
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(result.parse::<i32>().unwrap())
    }
}

#[derive(Debug)]
enum AST {
    BinOp(Box<AST>, Token, Box<AST>),
    Num(i32),
    Print(Box<AST>),
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
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

    fn parse(&mut self) -> AST {
        if let Token::Print = self.current_token {
            self.eat(Token::Print);
            AST::Print(Box::new(self.expr()))
        } else {
            self.expr()
        }
    }
}

struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    module: &'ctx Module<'ctx>,
    execution_engine: &'ctx ExecutionEngine<'ctx>,
    fn_value: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
        execution_engine: &'ctx ExecutionEngine<'ctx>,
    ) -> Self {
        Compiler {
            context,
            builder,
            module,
            execution_engine,
            fn_value: None,
        }
    }

    fn compile(&mut self, node: &AST) -> Option<BasicValueEnum<'ctx>> {
        match node {
            AST::BinOp(left, op, right) => {
                let lhs = self.compile(left).unwrap().into_int_value();
                let rhs = self.compile(right).unwrap().into_int_value();

                match op {
                    Token::Plus => Some(self.builder.build_int_add(lhs, rhs, "tmpadd").into()),
                    Token::Minus => Some(self.builder.build_int_sub(lhs, rhs, "tmpsub").into()),
                    Token::Star => Some(self.builder.build_int_mul(lhs, rhs, "tmpmul").into()),
                    Token::Slash => Some(self.builder.build_int_signed_div(lhs, rhs, "tmpdiv").into()),
                    _ => panic!("Unexpected binary operator"),
                }
            }
            AST::Num(value) => {
                let int_type = self.context.i32_type();
                Some(int_type.const_int(*value as u64, false).into())
            }
            AST::Print(expr) => {
                let value = self.compile(expr).unwrap().into_int_value();
                let printf = self.module.get_function("printf").unwrap();
                let format_str = self.builder.build_global_string_ptr("%d\n", "format_str");

                self.builder
                    .build_call(
                        printf,
                        &[format_str.as_pointer_value().into(), value.into()],
                        "printf_call",
                    )
                    .try_as_basic_value()
                    .left()
            }
        }
    }

    fn create_main_function(&mut self) {
        let int_type = self.context.i32_type();
        let fn_type = int_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);
        self.fn_value = Some(function);
    }

    fn finish_main_function(&self) {
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
    }
}

fn main() {
    Target::initialize_all(&InitializationConfig::default());

    let context = Context::create();
    let module = context.create_module("calc");
    let builder = context.create_builder();
    let execution_engine = module.create_execution_engine().unwrap();

    // Add the printf function declaration to the module
    let int_type = context.i32_type();
    let printf_type = int_type.fn_type(&[int_type.ptr_type(inkwell::AddressSpace::Generic).into()], true);
    module.add_function("printf", printf_type, None);

    // Read the input
    print!("Enter your code: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Tokenize and parse the input
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    // Compile the AST to LLVM IR
    let mut compiler = Compiler::new(&context, &builder, &module, &execution_engine);
    compiler.create_main_function();
    compiler.compile(&ast);
    compiler.finish_main_function();

    // Print the LLVM IR
    module.print_to_stderr();

    // Execute the LLVM IR
    unsafe {
        execution_engine.run_function_as_main(module.get_function("main").unwrap(), &[]);
    }
}
