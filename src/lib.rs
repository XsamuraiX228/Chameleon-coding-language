use crate::{
    dialect::SyntaxDict, 
    frontend::{lexer::Lexer, parser::Parser,vrmachine::VirtualMachine, vmparser::Bparser},
    runtime::interpreter::Interpreter,
    
};
pub mod dialect;
pub mod frontend;
pub mod runtime;
pub mod io; 
pub mod diagnostic;
use std::fs;

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
    // lexer.debug_tokens();
    let tokens = lexer.tokenize();
    // Creating parser
    let mut parser = Bparser::new(tokens, &config);
    // First we create raw_bytecode - it's not optimized and it's Vec<u16>
    let raw_bytecode = parser.start_byteparsing().map_err(|e| format!("Parser Error: {}", e))?;
    parser.debug_dump();
    fs::write("program.bin", &raw_bytecode).expect("Failed to write bytecode");
    // Run our sliced code
    let mut vm = VirtualMachine::new(raw_bytecode);
    vm.run_bytecode()?;

    Ok(())
}

#[cfg(test)]
mod vm_tests {
    
    use crate::{
    dialect::SyntaxDict, 
    frontend::{lexer::Lexer,vrmachine::VirtualMachine, vmparser::Bparser},
};   // Твой словарь синтаксиса

    // Вспомогательная функция, имитирующая сквозную сборку и запуск
    fn run_source(source: &str) -> VirtualMachine {
        let l_c = 1;
        let config = SyntaxDict::get_dict("ENGLISH"); // Или как она у тебя инициализируется
        let mut lexer = Lexer::new(source, &config, l_c);
        let tokens = lexer.tokenize(); // Твой метод токенизации
        
        let mut parser = Bparser::new(tokens, &config);
        // start_byteparsing() скомпилирует, вызовет serialized() и вернет Vec<u8>
        let raw_binary = parser.start_byteparsing().expect("Something went wrong"); 
        
        let mut vm = VirtualMachine::new(raw_binary);
        vm.run_bytecode().unwrap();
        vm
    }

    #[test]
    fn test_math_operations() {
        // Проверяем базовый Пратт-парсер, приоритеты операторов и работу со стеком
        // (2 + 3 * 4) ^ 2 - 5 = 14^2 - 5 = 196 - 5 = 191
        let vm = run_source("LET result = (2 + 3 * 4) ^ 2 - 5");
        assert_eq!(vm.get_globals()[0], 191);

        // Проверяем деление, остаток от деления и унарный минус
        let vm = run_source("LET res = -10 % 3 + 20 / 4");
        // -10 MOD 3 = -1, 20 / 4 = 5. -1 + 5 = 4
        assert_eq!(vm.get_globals()[0], 4);
    }

    #[test]
    fn test_variables_and_store() {
        // Проверяем перезапись переменных и чтение их значений
        let source = "
            LET a = 10
            LET b = a * 2
            LET a = b + 5
        ";
        let vm = run_source(source);
        assert_eq!(vm.get_globals()[0], 25); // Слот 0 (переменная a)
        assert_eq!(vm.get_globals()[1], 20); // Слот 1 (переменная b)
    }

    #[test]
    fn test_if_else_branching() {
        // Проверяем истинное условие (должна выполниться ветка THEN)
        let vm = run_source("
            LET x = 0
            IF 5 > 3 THEN
                LET x = 1
            ELSE
                LET x = 2
            END
        ");
        assert_eq!(vm.get_globals()[0], 1);

        // Проверяем ложное условие (должна выполниться ветка ELSE)
        let vm = run_source("
            LET x = 0
            IF 10 == 20 THEN
                LET x = 1
            ELSE
                LET x = 2
            END
        ");
        assert_eq!(vm.get_globals()[0], 2);
    }

    #[test]
    fn test_while_loop() {
        // Классический цикл WHILE от 1 до 5 (сумма чисел)
        let source = "
            LET i = 1
            LET sum = 0
            WHILE i <= 5 THEN
                LET sum = sum + i
                LET i = i + 1
            WEND
        ";
        let vm = run_source(source);
        assert_eq!(vm.get_globals()[0], 6);   // i выросло до 6
        assert_eq!(vm.get_globals()[1], 15);  // sum = 1+2+3+4+5 = 15
    }

    #[test]
    fn test_for_loop_positive_step() {
        // Цикл FOR с явным положительным шагом STEP 2
        let source = "
            LET total = 0
            FOR i = 1 TO 10 STEP 2
                LET total = total + i
            NEXT
        ";
        // Итерации: i=1 (total=1), i=3 (total=4), i=5 (total=9), i=7 (total=16), i=9 (total=25)
        // На i=11 цикл завершается
        let vm = run_source(source);
        assert_eq!(vm.get_globals()[0], 25); // total
    }

    #[test]
    fn test_for_loop_negative_step() {
        // Твоя гордость — цикл с отрицательным шагом! Обратный отсчет.
        let source = "
            LET countdown = 0
            FOR i = 10 TO 1 STEP -3
                LET countdown = countdown + i
            NEXT
        ";
        // Итерации: i=10 (countdown=10), i=7 (countdown=17), i=4 (countdown=21), i=1 (countdown=22)
        // На i=-2 цикл понимает, что -2 < 1 (так как сработал GreaterEq) и выходит!
        let vm = run_source(source);
        assert_eq!(vm.get_globals()[0], 22); // countdown
    }

    #[test]
    fn test_nested_loops() {
        // Сложный тест на вложенность: генерация таблицы умножения/сетки координат
        // Проверит, что адреса переходов в backpatching не перетирают друг друга
        let source = "
            LET counter = 0
            FOR x = 1 TO 3
                FOR y = 1 TO 3
                    LET counter = counter + 1
                NEXT
            NEXT
        ";
        let vm = run_source(source);
        assert_eq!(vm.get_globals()[0], 9); // Цикл должен отработать ровно 3 * 3 = 9 раз
    }

    #[test]
    #[should_panic(expected = "Division by zero!")]
    fn test_runtime_error_div_by_zero() {
        // Проверяем, что виртуальная машина корректно перехватывает критические ошибки
        // Если run_bytecode() возвращает Err, мы делаем .unwrap(), вызывая панику для теста
        let l_c = 1;
        let config = SyntaxDict::get_dict("ENGLISH");
        let mut parser = Bparser::new(Lexer::new("LET x = 10 / 0", &config, l_c).tokenize(), &config);
        let mut vm = VirtualMachine::new(parser.start_byteparsing().unwrap());
        vm.run_bytecode().unwrap(); // Тут должно упасть с ошибкой рантайма
    }
}

