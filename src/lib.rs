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
    // 1. Настройка диалекта по умолчанию
    let mut config = SyntaxDict::get_dict("ENGLISH");
    
    // Указатель на часть кода, которую будем парсить, и счетчик строк для ошибок
    let mut code_to_parse = raw_code;
    let mut line_counter = 1;
    
    // Препроцессинг: Ищем директиву #mode в первой строке
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
            // Отрезаем директиву #mode, чтобы лексер её не обрабатывал
            if let Some(pos) = raw_code.find('\n') {
                code_to_parse = &raw_code[pos + 1..];
            }
        }
    }

    // 2. Создаем лексер с учетом выбранного диалекта и смещения строк
    let mut lexer = Lexer::new(code_to_parse, &config, line_counter);
    // lexer.debug_tokens(); // Раскомментируй при отладке токенов
    let tokens = lexer.tokenize();
    
    // 3. Компилируем токены в байт-код через ByteParser
    let mut rvm_parser = ByteParser::new(tokens, &config);
    
    // Компилируем и превращаем кастомную ошибку парсера в String через .map_err
    let bytecode = rvm_parser.byteparse().map_err(|e| format!("Parser Error: {}", e))?;
    
    // Вывод отладочной информации компилятора (константы, переменные, байт-код)
    rvm_parser.debug();
    
    // 4. Инициализируем и запускаем нашу гипер-оптимизированную unsafe ВМ
    let mut vm = VirtualMachine::new(bytecode, &rvm_parser.constants, rvm_parser.variables.len());
    vm.run_bytecode()?;
    
    Ok(())
}

use std::time::{Instant, Duration};

pub fn fair_benchmark() {
    // БОЛЬШАЯ программа (100000 итераций)
    let russian_program = "
        #mode \"RUSSIAN\"
        ПУСТЬ СУММА = 0
        ПУСТЬ И = 1
        ПОКА И <= 10000000 ТО
            ПУСТЬ СУММА = СУММА + И
            ПУСТЬ И = И + 1
        КОНЕЦ_ПОКА
        ПЕЧАТЬ СУММА
    ";
        
    let iterations = 10; // повторяем 10 раз для усреднения
    
    let mut classic_times = Vec::new();
    let mut vm_times = Vec::new();
    
    // Классический интерпретатор
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