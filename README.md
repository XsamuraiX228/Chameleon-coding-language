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
│   └── parser.rs        # Pratt Parser engine: returns Result<Vec<SpannedStatement>, ErrorHandler>
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
To execute a custom program, write you file in examples and type in console:
```bash
cargo run
```
Example of a file:
```rust
#mode "ENGLISH"
LET X = 1
WHILE X <= 10 THEN
    IF X % 2 == 0 THEN
        PRINT X
        PRINT " is even"
    ELSE
        PRINT X
        PRINT " is odd"
    END
    IF X == 5 THEN
        PRINT "Halfway there!"
    END
    LET X = X + 1
WEND
PRINT "Done!"
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
