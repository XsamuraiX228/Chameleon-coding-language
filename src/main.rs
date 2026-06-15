use basic_lexer::{io::scanner::{
    load_code,
    scan_code,}, 
    // run_pipeline,
    run_rvmpipeline,
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

    // run_pipeline(&code)?;
    if let Err(pipeline_error) = run_rvmpipeline(&code) {
        // Ипользуем eprintln! (печать в поток ошибок), выводится чистый текст БЕЗ кавычек!
        println!("{}", pipeline_error); 
    }
    Ok(())
}

