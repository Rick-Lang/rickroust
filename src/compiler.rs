use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::parser::AST;
use crate::lexer::Token;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    module: &'ctx Module<'ctx>,
    execution_engine: &'ctx ExecutionEngine<'ctx>,
    fn_value: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(
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

    pub fn compile(&mut self, node: &AST) -> Option<BasicValueEnum<'ctx>> {
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

    pub fn create_main_function(&mut self) {
        let int_type = self.context.i32_type();
        let fn_type = int_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);
        self.fn_value = Some(function);
    }

    pub fn finish_main_function(&self) {
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
    }
}
