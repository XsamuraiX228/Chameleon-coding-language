use crate::{
    frontend::vm::{compiler::bparser::Bparser, executor::vrmachine::VirtualMachine},
    frontend::lexer_v2::lexer_v2::{LexerV2, debug_tokens},
};
pub mod diagnostic;
pub mod dialect;
pub mod frontend;
pub mod io;

/// VM Machine
/// Run the code (Preprocessor -> Lexer -> Compiler -> Executor)
pub fn run_rvmpipeline(raw_code: &str) -> Result<(), String> {
    let mut lexer_v2 = LexerV2::new(raw_code);
    let (tokens, cfg) = lexer_v2.tokenizer();
    debug_tokens(&tokens);

    let mut parser = Bparser::new(tokens, &cfg);

    let raw_bytecode = parser.start_byteparsing().map_err(|e| format!("{}", e))?;
    parser.debug_dump();
    // fs::write("program.bin", &raw_bytecode).expect("Failed to write bytecode");
    let mut vm = VirtualMachine::new(raw_bytecode, parser.function_list);
    vm.run_bytecode()?;

    Ok(())
}
