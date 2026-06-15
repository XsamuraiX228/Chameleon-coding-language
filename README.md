Hello there!
# 🦎 Chameleon
A programming language with morphic syntax mapping, implemented entirely in Rust.

## 🏗️ Architecture

```
examples/                # Code snippets and example scripts (.chm / crab-style 🦀)
src/
├── main.rs              # App entry point: reads files, catches global errors, and prints them
├── lib.rs               # Orchestrator pipeline: Lexer -> Parser -> Interpreter
├── dialect.rs           # Core keyword dictionaries (Chameleon, Crab-Style definitions)
├── diagnostic/          # Interprenter diagnostics & reporting
│   ├── mod.rs           
│   └── errors.rs        # ErrorHandler struct, ErrorKind enum, and Display formatting
├── frontend/            # Source code analysis module
│   ├── mod.rs           
│   ├── token.rs         # Lexer tokens and SpannedToken (token + line)
│   ├── lexer.rs         # converts raw source strings into Vec<Token<'a>>
│   ├── ast.rs           # Abstract Syntax Tree structures and SpannedStatement
│   ├── parser.rs        # Pratt Parser engine: returns Result<Vec<SpannedStatement>, ErrorHandler>
│   ├── vmparser.rs
│   └── vmmachine.rs
├── io/
│   ├── mod.rs
|   └── scanner.rs       # Scan dirs for files with base .bsa and read
└── runtime/             # Core execution engine
    ├── mod.rs           
    └── interpreter.rs   # Walks the Vec<SpannedStatement>, generating ErrorKind::Runtime
```

## 🛠️ Getting Started

### Prerequisites
Make sure you have [Rust and Cargo](https://rustup.rs/) installed.

### Running a Script
Here is the code, which shows all current instruments which language provide:
```bash
cargo run
```
Example of a file:
```rust
// BLOCK 1: (LET)

LET int_var = 10
LET float_var = 45.67
LET bool_var1 = TRUE
LET bool_var2 = FALSE
LET text_var = "HELLO WORLD"

// Check PRINT 
PRINT text_var
PRINT int_var
PRINT float_var


// BLOCK 2: (++ / --)

int_var++
float_var--
PRINT int_var    // 11
PRINT float_var  // 44.67


// BLOCK 3: Math

LET math_res = (int_var * 2) + 100 % 3
PRINT math_res

LET pi_approx = 314159
LET radius = 5
LET circumference_approx = (2 * pi_approx * radius) / 100000
PRINT circumference_approx


// BLOCK 4: logic (IF / ELSE / AND / OR)


IF int_var == 11 AND bool_var1 THEN
    PRINT "Strict logic stage 1: PASSED"
ELSE
    PRINT "Strict logic stage 1: FAILED"
END

IF bool_var2 OR 5 > 10 THEN
    PRINT "Strict logic stage 2: FAILED"
ELSE
    PRINT "Strict logic stage 2: PASSED"
END


// BLOCK 5: Conditions with ELSE blocks

LET score = 85
IF score >= 90 THEN
    PRINT "Grade: A"
ELSE
    IF score >= 80 THEN
        PRINT "Grade: B"  
    ELSE
        PRINT "Grade: C"
    END
END


// BLOCK 6: Loops (WHILE и FOR)


LET iterator = 1.0
WHILE iterator <= 3.0 THEN
    PRINT iterator
    LET iterator = iterator + 0.5
WEND


LET sum_accumulator = 0
FOR N = 1 TO 5
    PRINT N
    LET sum_accumulator = sum_accumulator + N
NEXT
PRINT sum_accumulator // 15
```
Writing '#mode "DICT NAME" is specific, because lexer needs to understand what Dictionary is used at the moment'

## 🧩 How Custom Syntax (Dialects) Works

The core feature of this interpreter is its ability to support completely fluid, user-defined programming syntaxes (dialects) — including localization into other languages or mapping commands entirely to emojis.
Adding a new language or variant requires zero changes to the parser engine. You just expand the dictionary registry in `dialect.rs`:

```rust
// Inside dialect.rs
let mut english = HashMap::new();
english.insert("LET", KeyWordType::Let);
english.insert("PRINT", KeyWordType::Print);
english.insert("WHILE", KeyWordType::While);
english.insert("WEND", KeyWordType::Wend);

let mut emoji_mode = HashMap::new();
emoji_mode.insert("📦", KeyWordType::Let);
emoji_mode.insert("📢", KeyWordType::Print);
emoji_mode.insert("🔄", KeyWordType::While);
emoji_mode.insert("🛑", KeyWordType::Wend);

## 📄 License

MIT
