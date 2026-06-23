use crate::{
    frontend::vm::{compiler::bparser::Bparser, executor::vrmachine::VirtualMachine},
    frontend::{classic::parser::Parser, lexer::Lexer},
    runtime::interpreter::Interpreter,
};
pub mod diagnostic;
pub mod dialect;
pub mod frontend;
pub mod io;
pub mod runtime;
use std::fs;

// use std::time::Instant;

// pub fn run_pure_benchmark(raw_code: &str) -> Result<(), String> {
//     println!("\n==================================================");
//     println!("      ⚙️  ПРИГОТОВЛЕНИЕ ТЕСТОВОЙ СРЕДЫ... ");
//     println!("==================================================");

//     // 1. Подготовка для классического интерпретатора
//     let mut lexer_classic = Lexer::new(raw_code);
//     let (tokens_classic, config) = lexer_classic.tokenize();
//     let mut parser_classic = Parser::new(tokens_classic, &config);
//     let ast = parser_classic
//         .parse()
//         .map_err(|e| format!("Ошибка парсинга AST: {}", e))?;

//     // Сканируем метки один раз снаружи
//     let mut temp_interpreter = Interpreter::new();
//     let marks = temp_interpreter.pre_scan_labels(&ast);

//     // 2. Подготовка для виртуальной машины (VM)
//     let mut lexer_vm = Lexer::new(raw_code);
//     let (tokens_vm, config) = lexer_vm.tokenize();
//     let mut parser_vm = Bparser::new(tokens_vm, &config);
//     let raw_bytecode = parser_vm
//         .start_byteparsing()
//         .map_err(|e| format!("Ошибка компиляции ВМ: {}", e))?;

//     println!("✅ Все структуры данных готовы к бою. Начинаем замеры (10 прогонов).");
//     println!("==================================================");
//     println!("               RUNTIME BENCHMARK (x10)");
//     println!("==================================================\n");

//     // ----------------------------------------------------
//     // ЗАМЕР: Классический интерпретатор (10 прогонов)
//     // ----------------------------------------------------
//     println!("📜 Classic Interpreter (AST Tree Walk) - 10 Runs:");
//     let mut total_classic_time = std::time::Duration::ZERO;

//     for i in 1..=10 {
//         // Создаем чистый интерпретатор для каждого раунда
//         let mut interpreter = Interpreter::new();

//         let start_classic = Instant::now();
//         let ast_res = std::hint::black_box(interpreter.execute(&ast, &marks));
//         let elapsed = start_classic.elapsed();

//         ast_res?; // Проверяем на ошибки
//         total_classic_time += elapsed;
//         println!("   Round {:2}: {:?}", i, elapsed);
//     }
//     let avg_classic = total_classic_time / 10;
//     println!("   ➔ СРЕДНЕЕ ВРЕМЯ AST: {:?}\n", avg_classic);

//     // ----------------------------------------------------
//     // ЗАМЕР: Виртуальная Машина (10 прогонов)
//     // ----------------------------------------------------
//     println!("⚡ Virtual Machine (Flat u64 Bytecode) - 10 Runs:");
//     let mut total_vm_time = std::time::Duration::ZERO;

//     for i in 1..=10 {
//         // Создаем чистую ВМ (сброшенный стек, pc=0, чистые глобалы)
//         // Десериализация происходит ДО замера времени!
//         let mut vm = VirtualMachine::new(raw_bytecode.clone(), parser_vm.function_list);

//         let start_vm = Instant::now();
//         let vm_res = std::hint::black_box(vm.run_bytecode());
//         let elapsed = start_vm.elapsed();

//         vm_res?; // Проверяем на ошибки
//         total_vm_time += elapsed;
//         println!("   Round {:2}: {:?}", i, elapsed);
//     }
//     let avg_vm = total_vm_time / 10;
//     println!("   ➔ СРЕДНЕЕ ВРЕМЯ VM:  {:?}\n", avg_vm);

//     // ----------------------------------------------------
//     // СТАБИЛЬНЫЕ РЕЗУЛЬТАТЫ СРАВНЕНИЯ
//     // ----------------------------------------------------
//     let classic_nanos = avg_classic.as_nanos() as f64;
//     let vm_nanos = avg_vm.as_nanos() as f64;
//     let speedup = classic_nanos / vm_nanos;

//     println!("==================================================");
//     println!("             🎉 ИТОГОВЫЙ СРЕДНИЙ РЕЗУЛЬТАТ");
//     println!("==================================================\n");
//     println!("   Классический AST (Среднее): {:?}", avg_classic);
//     println!("   Новая ВМ (u64)    (Среднее): {:?}", avg_vm);
//     println!(
//         "   🚀 Виртуальная машина стабильно быстрее в {:.2}x раз!",
//         speedup
//     );
//     println!("\n==================================================");

//     Ok(())
// }

/// Run the code (Preprocessor -> Lexer -> Parser -> Interprenter)
pub fn run_pipeline(raw_code: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(raw_code);

    let (tokens, cfg) = lexer.tokenize();

    let mut parser = Parser::new(tokens, cfg);

    let mut interpreter = Interpreter::new();
    match parser.parse() {
        Ok(ast) => {
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
/// Run the code (Preprocessor -> Lexer -> Compiler -> Executor)
pub fn run_rvmpipeline(raw_code: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(raw_code);
    // lexer.debug_tokens();
    let (tokens, cfg) = lexer.tokenize();

    let mut parser = Bparser::new(tokens, &cfg);

    let raw_bytecode = parser.start_byteparsing().map_err(|e| format!("{}", e))?;
    parser.debug_dump();
    fs::write("program.bin", &raw_bytecode).expect("Failed to write bytecode");

    let mut vm = VirtualMachine::new(raw_bytecode, parser.function_list);
    vm.run_bytecode()?;

    Ok(())
}
