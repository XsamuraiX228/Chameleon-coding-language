use crate::{
    dialect::SyntaxDict,
    frontend::vm::{parser::bparser::Bparser, executor::vrmachine::VirtualMachine},
    frontend::{lexer::Lexer, classic::parser::Parser},
    runtime::interpreter::Interpreter,
};
pub mod diagnostic;
pub mod dialect;
pub mod frontend;
pub mod io;
pub mod runtime;
use std::fs;

// Добавь этот импорт в самый верх lib.rs, если его там нет:
use std::time::Instant;

// ============================================
// ЧЕСТНЫЙ БЕНЧМАРК РАНТАЙМА
// ============================================

pub fn run_pure_benchmark(raw_code: &str) -> Result<(), String> {
    let config = SyntaxDict::get_dict("ENGLISH");
    
    println!("\n==================================================");
    println!("      ⚙️  ПРИГОТОВЛЕНИЕ ТЕСТОВОЙ СРЕДЫ... ");
    println!("==================================================");

    // 1. Подготовка для классического интерпретатора
    let mut lexer_classic = Lexer::new(raw_code, &config, 1);
    let tokens_classic = lexer_classic.tokenize();
    let mut parser_classic = Parser::new(tokens_classic, &config);
    let ast = parser_classic.parse().map_err(|e| format!("Ошибка парсинга AST: {}", e))?;
    
    // Сканируем метки один раз снаружи
    let mut temp_interpreter = Interpreter::new();
    let marks = temp_interpreter.pre_scan_labels(&ast);

    // 2. Подготовка для виртуальной машины (VM)
    let mut lexer_vm = Lexer::new(raw_code, &config, 1);
    let tokens_vm = lexer_vm.tokenize();
    let mut parser_vm = Bparser::new(tokens_vm, &config);
    let raw_bytecode = parser_vm.start_byteparsing().map_err(|e| format!("Ошибка компиляции ВМ: {}", e))?;

    println!("✅ Все структуры данных готовы к бою. Начинаем замеры (10 прогонов).");
    println!("==================================================");
    println!("               RUNTIME BENCHMARK (x10)");
    println!("==================================================\n");

    // ----------------------------------------------------
    // ЗАМЕР: Классический интерпретатор (10 прогонов)
    // ----------------------------------------------------
    println!("📜 Classic Interpreter (AST Tree Walk) - 10 Runs:");
    let mut total_classic_time = std::time::Duration::ZERO;

    for i in 1..=10 {
        // Создаем чистый интерпретатор для каждого раунда
        let mut interpreter = Interpreter::new();
        
        let start_classic = Instant::now();
        let ast_res = std::hint::black_box(interpreter.execute(&ast, &marks));
        let elapsed = start_classic.elapsed();
        
        ast_res?; // Проверяем на ошибки
        total_classic_time += elapsed;
        println!("   Round {:2}: {:?}", i, elapsed);
    }
    let avg_classic = total_classic_time / 10;
    println!("   ➔ СРЕДНЕЕ ВРЕМЯ AST: {:?}\n", avg_classic);


    // ----------------------------------------------------
    // ЗАМЕР: Виртуальная Машина (10 прогонов)
    // ----------------------------------------------------
    println!("⚡ Virtual Machine (Flat u64 Bytecode) - 10 Runs:");
    let mut total_vm_time = std::time::Duration::ZERO;

    for i in 1..=10 {
        // Создаем чистую ВМ (сброшенный стек, pc=0, чистые глобалы)
        // Десериализация происходит ДО замера времени!
        let mut vm = VirtualMachine::new(raw_bytecode.clone());
        
        let start_vm = Instant::now();
        let vm_res = std::hint::black_box(vm.run_bytecode());
        let elapsed = start_vm.elapsed();
        
        vm_res?; // Проверяем на ошибки
        total_vm_time += elapsed;
        println!("   Round {:2}: {:?}", i, elapsed);
    }
    let avg_vm = total_vm_time / 10;
    println!("   ➔ СРЕДНЕЕ ВРЕМЯ VM:  {:?}\n", avg_vm);


    // ----------------------------------------------------
    // СТАБИЛЬНЫЕ РЕЗУЛЬТАТЫ СРАВНЕНИЯ
    // ----------------------------------------------------
    let classic_nanos = avg_classic.as_nanos() as f64;
    let vm_nanos = avg_vm.as_nanos() as f64;
    let speedup = classic_nanos / vm_nanos;

    println!("==================================================");
    println!("             🎉 ИТОГОВЫЙ СРЕДНИЙ РЕЗУЛЬТАТ");
    println!("==================================================\n");
    println!("   Классический AST (Среднее): {:?}", avg_classic);
    println!("   Новая ВМ (u64)    (Среднее): {:?}", avg_vm);
    println!("   🚀 Виртуальная машина стабильно быстрее в {:.2}x раз!", speedup);
    println!("\n==================================================");

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
                    println!(
                        "[Preprocessor]: Dictionary for language successfully connected: {}",
                        dict_name
                    );
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

/// VM Machine
/// Run the code (Preprocessor -> Lexer -> BParser -> Compiler)
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
                    println!(
                        "[VM Preprocessor]: Dictionary for language successfully connected: {}",
                        dict_name
                    );
                }
            }
            if let Some(pos) = raw_code.find('\n') {
                code_to_parse = &raw_code[pos + 1..];
            }
        }
    }

    // Creating lexer to read the whole file code and create a Vec<SpannedToken<'_>>
    let mut lexer = Lexer::new(code_to_parse, &config, line_counter);
    // lexer.debug_tokens();
    let tokens = lexer.tokenize();
    // Creating parser
    let mut parser = Bparser::new(tokens, &config);
    let raw_bytecode = parser.start_byteparsing().map_err(|e| format!("{}", e))?;
    parser.debug_dump();
    fs::write("program.bin", &raw_bytecode).expect("Failed to write bytecode");
    // Run our sliced code
    let mut vm = VirtualMachine::new(raw_bytecode);
    vm.run_bytecode()?;

    Ok(())
}

// ============================================
// ПРОСТОЙ БЕНЧМАРК
// ============================================

