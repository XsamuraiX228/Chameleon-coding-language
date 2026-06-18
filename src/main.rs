use basic_lexer::{
    // io::scanner::{
    // load_code,
    // scan_code,}, 
    run_pipeline,
    run_rvmpipeline,
    
};

use std::time::Instant;


fn main() {
    let program = r#"
        LET SUM = 0
        LET I = 1
        WHILE I <= 1000000 THEN
            LET SUM = SUM + I
            LET I = I + 1
        WEND
        PRINT SUM
    "#;

    println!("");
    println!("==================================================");
    println!("              BENCHMARK");
    println!("==================================================");
    println!("");

    // Классический интерпретатор
    println!("📜 Classic Interpreter (AST):");
    let start = Instant::now();
    let _ = run_pipeline(program);
    let classic_time = start.elapsed();
    println!("   {:?}", classic_time);
    println!("");

    // VM
    println!("⚡ Virtual Machine (Bytecode):");
    let start = Instant::now();
    let _ = run_rvmpipeline(program);
    let vm_time = start.elapsed();
    println!("   {:?}", vm_time);
    println!("");

    // Результат
    let speedup = classic_time.as_nanos() as f64 / vm_time.as_nanos() as f64;
    println!("==================================================");
    println!("              RESULT");
    println!("==================================================");
    println!("");
    println!("   Classic: {:?}", classic_time);
    println!("   VM:      {:?}", vm_time);
    println!("   VM faster {:.2}x times", speedup);
    println!("");
}

