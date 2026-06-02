use crate::{
    dialect::SyntaxDict, 
    frontend::{lexer::Lexer, parser::Parser},
    runtime::interpreter::Interpreter,
};
pub mod dialect;
pub mod frontend;
pub mod runtime;
pub mod io; 
pub mod diagnostic;

/// Run the code (Preprocessor -> Lexer -> Parser -> Interprenter)
pub fn run_pipeline(raw_code: &str) -> Result<(), String> {
    // 1. Looking for #mode and set dialect::SyntaxDict
    let mut config = SyntaxDict::get_dict("ENGLISH");
    
    // Variable-pointer to the part of the parsing code
    let mut code_to_parse = raw_code;
    let mut line_counter = 1;
    if let Some(first_line) = raw_code.lines().next() {
        let trimmed = first_line.trim();
        if trimmed.starts_with("#mode") {
            line_counter += 1;
            if let (Some(start_quote), Some(end_quote)) = (trimmed.find('"'), trimmed.rfind('"')) {
                if start_quote != end_quote {
                    let dict_name = &trimmed[start_quote + 1..end_quote]; 
                    config = SyntaxDict::get_dict(dict_name); 
                    println!("[Preprocessor]: Dictionary for language successfully connected: {}", dict_name);
                }
            }
            if let Some(pos) = raw_code.find('\n') {
                code_to_parse = &raw_code[pos + 1..];
            }
        }
    }

    // 2. Create lexer
    let mut lexer = Lexer::new(code_to_parse, &config, line_counter);
    // lexer.debug_tokens();
    let tokens = lexer.tokenize();
    // 3. Create parser
    let mut parser = Parser::new(tokens, &config);
    // 4. Create interprenter
    let mut interpreter = Interpreter::new();
    match parser.parse() {
        Ok(ast) => {
            // run interpreter
            let marks = interpreter.pre_scan_labels(&ast);
            interpreter.execute(&ast, &marks)?;
        }
        Err(err_string) => {
            eprintln!("Ошибка: {}", err_string);
        }
    }
    Ok(())
}