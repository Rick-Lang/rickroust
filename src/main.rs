mod lexer;
mod parser;
// mod compiler;
mod interpreter;

use std::io::{self, Write};

// use inkwell::context::Context;
// use inkwell::targets::{InitializationConfig, Target};

fn main() {
    /*
    Target::initialize_all(&InitializationConfig::default());

    let context = Context::create();
    let module = context.create_module("calc");
    let builder = context.create_builder();
    let execution_engine = module.create_execution_engine().unwrap();

    // Add the printf function declaration to the module
    let int_type = context.i32_type();
    let printf_type = int_type.fn_type(&[int_type.ptr_type(inkwell::AddressSpace::Generic).into()], true);
    module.add_function("printf", printf_type, None);
    */

    // Read the input
    print!(">>>");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Tokenize and parse the input
    let lexer = lexer::Lexer::new(&input);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();

    /*
    // Compile the AST to LLVM IR
    let mut compiler = compiler::Compiler::new(&context, &builder, &module, &execution_engine);
    compiler.create_main_function();
    compiler.compile(&ast);
    compiler.finish_main_function();
    

    // Print the LLVM IR
    module.print_to_stderr();

    // Execute the LLVM IR
    unsafe {
        execution_engine.run_function_as_main(module.get_function("main").unwrap(), &[]);
    }
    */

    // Interpret the AST
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&ast);
}
