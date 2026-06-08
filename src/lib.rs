use crate::{
    dialect::SyntaxDict, 
    frontend::{lexer::Lexer, parser::Parser, vmparser::ByteParser, vrmachine::VirtualMachine},
    runtime::interpreter::Interpreter,
    
};
pub mod dialect;
pub mod frontend;
pub mod runtime;
pub mod io; 
pub mod diagnostic;

use std::fs::File;
use std::io::Write;

pub fn save_bytecode_to_file(bytecode: &[u16], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let mut byte_buffer = Vec::with_capacity(bytecode.len() * 2);

    for &word in bytecode {
        // Convert u16 to bytes (using Little Endian, or use to_be_bytes for Big Endian)
        let bytes = word.to_le_bytes(); 
        byte_buffer.push(bytes[0]);
        byte_buffer.push(bytes[1]);
    }

    file.write_all(&byte_buffer)?;
    Ok(())
}

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

pub fn run_rvmpipeline(raw_code: &str) -> Result<(), String> {
    let mut config = SyntaxDict::get_dict("ENGLISH");
    
    
    let mut code_to_parse = raw_code;
    let mut line_counter = 1;

    // Check the first line to get the dialect for our language
    if let Some(first_line) = raw_code.lines().next() {
        let trimmed = first_line.trim();
        if trimmed.starts_with("#mode") {
            line_counter += 1;
            if let (Some(start_quote), Some(end_quote)) = (trimmed.find('"'), trimmed.rfind('"')) {
                if start_quote != end_quote {
                    let dict_name = &trimmed[start_quote + 1..end_quote]; 
                    config = SyntaxDict::get_dict(dict_name); 
                    println!("[VM Preprocessor]: Dictionary for language successfully connected: {}", dict_name);
                }
            }
            if let Some(pos) = raw_code.find('\n') {
                code_to_parse = &raw_code[pos + 1..];
            }
        }
    }

    // Creating lexer to read the whole file code and create a Vec<SpannedToken<'_>>
    let mut lexer = Lexer::new(code_to_parse, &config, line_counter);
    let tokens = lexer.tokenize();
    
    // Creating parser
    let mut parser = ByteParser::new(tokens, &config);

    // First we create raw_bytecode - it's not optimized and it's Vec<u16>
    let raw_bytecode = parser.byteparse().map_err(|e| format!("Parser Error: {}", e))?;

    // Second we call an optimize_and_map_addresses function to change some long and repetitive instructions
    // for shorter ones, while also creating an address map which will help to set the correct jump
    // points for out Jump and JumpIfFalse opcodes
    let (optimized_bytecode, addr_map) = ByteParser::optimize_and_map_addresses(&raw_bytecode);

    // patch_addresses one more time move through the whole array and set correct address
    let patched_bytecode = ByteParser::patch_addresses(optimized_bytecode, &addr_map);

    // Finally convert our Vec<u16> -> Vec<u8>
    let slicer = ByteParser::finalize_to_u8_simple(&patched_bytecode);
    
    // Run our sliced code
    let mut vm = VirtualMachine::new(slicer, parser.constants, parser.variables.len());
    vm.run_bytecode()?;

    Ok(())
}



use std::time::{Instant, Duration};

pub fn fair_benchmark() {
    let russian_program = "
        LET SUMMA = 0
        LET I = 1
        WHILE I <= 10000000 THEN
            LET SUMMA = SUMMA + I
            LET I = I + 1
        WEND
    ";
        
    let iterations = 10; 
    
    let mut classic_times = Vec::new();
    let mut vm_times = Vec::new();
    
    // Classic
    println!("Running Classic interpreter...");
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = run_pipeline(russian_program);
        classic_times.push(start.elapsed());
    }
    
    // VM
    println!("Running VM...");
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = run_rvmpipeline(russian_program);
        vm_times.push(start.elapsed());
    }
    
    let classic_avg = classic_times.iter().sum::<Duration>() / iterations as u32;
    let vm_avg = vm_times.iter().sum::<Duration>() / iterations as u32;
    
    println!("\n=== RESULTS ===");
    println!("Classic: {:?}", classic_avg);
    println!("VM:      {:?}", vm_avg);
    println!("VM is {:.2}x {}", 
        (classic_avg.as_nanos() as f64 / vm_avg.as_nanos() as f64).abs(),
        if classic_avg > vm_avg { "faster" } else { "slower" }
    );
}