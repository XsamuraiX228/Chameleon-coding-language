use basic_lexer::{io::scanner::{
    load_code,
    scan_code,}, 
    run_pipeline
};

fn main() -> Result<(), String> {
    // Find files in dir FILES
    let content_to_load = match scan_code("examples") {
        Ok(files) => files,
        Err(e) => {
            return Err(format!("[Scanning error]: {}", e));
        }
    };

    // 2. Get file.bsa
    let path = match content_to_load.first() {
        Some(p) => p,
        None => { 
            return Err(format!("[Error]: No files with extension found in folder 'FILES' .bsa")); 
        }
    };

    // 3. Loading the code from the file
    let code = match load_code(path) {
        Ok(text) => text,
        Err(e) => { 
            return Err(format!("[Error reading file {:?}]: {}", path, e)); 
        }
    };
    
    run_pipeline(&code)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_program {
        ($name:ident, $program:expr) => {
            #[test]
            fn $name() {
                let result = run_pipeline($program);
                assert!(result.is_ok(), "Error: {:?}", result.err());
            }
        };
    }

    macro_rules! test_error {
        ($name:ident, $program:expr) => {
            #[test]
            fn $name() {
                let result = run_pipeline($program);
                assert!(result.is_err(), "Expected error, but program executed with succes!");
            }
        };
    }

    
    test_program!(test_all_language_constructs, r#"
        #mode "ENGLISH"
        LET X = 10
        // Симулируем ввод, если твой INPUT умеет работать со значениями по умолчанию, 
        // либо просто проверяем парсинг и выполнение:
        LET Y = 50 
        IF X >= Y THEN
            PRINT X
        ELSE
            PRINT Y
        END

        WHILE X <= 100 THEN
            PRINT X
            LET X = X + 10
        WEND

        FOR START = 10 TO Y
            PRINT X
            NEXT
    "#);

    
    test_program!(test_math_basic, "LET X = 1 + 2 * 3");
    test_program!(test_math_parens, "LET X = (1 + 2) * 3");

    
    test_program!(test_if_then, "
        LET X = 10
        IF X > 5 THEN
            LET Y = 1
        END
    ");
    test_program!(test_if_else, "
        LET X = 2
        IF X > 5 THEN
            LET Y = 1
        ELSE
            LET Y = 2
        END
    ");

    
    test_program!(test_goto_basic, "
        GOTO skip
        LET X = 1
        :skip
        LET Y = 2
    ");
    test_program!(test_goto_loop, "
        LET X = 1
        :start
        LET X = X + 1
        IF X <= 3 THEN GOTO start
        END
    ");

    
    test_program!(test_random, "
        RANDOM X 1 100
    ");

    
    test_error!(test_division_by_zero, "
        LET X = 10 / 0
    ");
    test_error!(test_undefined_variable, "
        PRINT UNDEFINED
    ");
    test_error!(test_goto_missing_label, "
        GOTO missing_label
    ");

    
    test_program!(test_fibonacci, "
        LET A = 0
        LET B = 1
        LET N = 10
        LET I = 0
        
        WHILE I < N THEN
            LET C = A + B
            LET A = B
            LET B = C
            LET I = I + 1
        WEND
    ");

    test_program!(test_factorial_loop, "
        LET N = 5
        LET FACT = 1
        LET I = 1
        WHILE I <= N THEN
            LET FACT = FACT * I
            LET I = I + 1
        WEND
    ");
    

    
    test_program!(test_complex_if_inside_while, r#"
        #mode "ENGLISH"
        LET COUNTER = 1
        LET EVENS_SUM = 0
        LET ODDS_SUM = 0
        
        WHILE COUNTER <= 10 THEN
            LET IS_EVEN = COUNTER % 2
            IF IS_EVEN == 0 THEN
                LET EVENS_SUM = EVENS_SUM + COUNTER
            ELSE
                LET ODDS_SUM = ODDS_SUM + COUNTER
            END
            LET COUNTER = COUNTER + 1
        WEND
    "#);


    test_program!(test_complex_while_inside_for, r#"
        #mode "ENGLISH"
        LET TOTAL_STEPS = 0
        LET LIMIT = 3
        
        FOR OUT_ROW = 1 TO LIMIT
            LET IN_COL = 1
            WHILE IN_COL <= 3 THEN
                LET TOTAL_STEPS = TOTAL_STEPS + 1
                LET IN_COL = IN_COL + 1
            WEND
        NEXT
    "#);


    test_program!(test_extreme_triple_nesting, r#"
        #mode "ENGLISH"
        LET INSANE_COUNTER = 0
        LET MAX_ROWS = 2
        
        FOR I = 1 TO MAX_ROWS
            LET J = 1
            WHILE J <= 2 THEN
                LET CHECK_VALUE = (I + J) % 2
                IF CHECK_VALUE == 0 THEN
                    LET INSANE_COUNTER = INSANE_COUNTER + 1
                ELSE
                    LET INSANE_COUNTER = INSANE_COUNTER + 2
                END
                LET J = J + 1
            WEND
        NEXT
    "#);


    test_program!(test_pratt_math_inside_deep_nested_if, r#"
        #mode "ENGLISH"
        LET STATUS = 0
        LET X = 5
        LET Y = 10
        
        IF X < Y THEN
            IF (Y - X) == 5 THEN
                // Проверяем работу приоритетов и унарных/постфиксных операторов: 3! + (2 ^ 3) = 6 + 8 = 14
                LET STATUS = 3! + (2 ^ 3)
            END
        END
    "#);


    test_program!(test_nested_goto_escape_from_for, r#"
        #mode "ENGLISH"
        LET ESCAPED = 0
        LET ITERATIONS = 0
        
        FOR K = 1 TO 100
            LET ITERATIONS = ITERATIONS + 1
            IF K == 5 THEN
                LET ESCAPED = 1
                GOTO :out_of_heavy_loop
            END
        NEXT
        
        :out_of_heavy_loop
        LET FINAL_CHECK = ESCAPED
    "#);
}
