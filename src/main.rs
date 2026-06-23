use basic_lexer::io::scanner::{load_code, scan_code};
use basic_lexer::{run_rvmpipeline};
fn main() -> Result<(), String> {
    let content_to_load = scan_code("examples")?;

    let path = match content_to_load.first() {
        Some(p) => p,
        None => { 
            println!("No files with extension found in folder 'FILES' .bsa");
            unreachable!()
        }
    };

    let code = match load_code(path) {
        Ok(code) => code,
        Err(_) => {
            println!("Error in scaning the text from file");
            unreachable!()
        }
    };

    run_rvmpipeline(&code)?;

    // let program = r#"
    //     LET I = 0
    //     LET SUM = 0
    //     WHILE I <= 1000000 THEN
    //         LET SUM = SUM + I 
    //         LET I = I + 1
    //     WEND
    //     PRINT SUM
    // "#;

    // run_pure_benchmark(program)?;
    
    Ok(())
}

