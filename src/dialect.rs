use crate::frontend::token::{FuncKeyWord, KeyWordType,VarType};
use std::collections::HashMap;

pub enum Dict {
    Russian,
    English,
    Crab,
    Emoji,
}

pub struct SyntaxDict {
    pub keywords: HashMap<String, KeyWordType>,
    pub func_keywords: HashMap<String, FuncKeyWord>,
    pub type_keywords: HashMap<String, VarType>,
    pub bool_keywords: HashMap<String, bool>
}

#[allow(dead_code)]
impl SyntaxDict {
    fn default_english() -> Self {
        let mut keywords = HashMap::new();
        let mut func_keywords = HashMap::new();
        let mut type_keywords = HashMap::new();
        let mut bool_keywords = HashMap::new();

        // Types 
        type_keywords.insert("int".to_string(), VarType::Int);
        type_keywords.insert("float".to_string(), VarType::Float);
        type_keywords.insert("bool".to_string(), VarType::Bool);
        type_keywords.insert("string".to_string(), VarType::String);

        // Variables & Operations
        keywords.insert("let".to_string(), KeyWordType::Let);
        keywords.insert("print".to_string(), KeyWordType::Print);
        keywords.insert("println".to_string(), KeyWordType::Println);
        keywords.insert("input".to_string(), KeyWordType::Input);

        // Conditionals
        keywords.insert("if".to_string(), KeyWordType::If);
        keywords.insert("then".to_string(), KeyWordType::Then);
        keywords.insert("else".to_string(), KeyWordType::Else);

        // Loops
        keywords.insert("while".to_string(), KeyWordType::While);
        keywords.insert("wend".to_string(), KeyWordType::Wend);
        keywords.insert("for".to_string(), KeyWordType::For);
        keywords.insert("to".to_string(), KeyWordType::To);
        keywords.insert("step".to_string(), KeyWordType::Step);
        keywords.insert("next".to_string(), KeyWordType::Next);

        // Jumps & Utilities
        keywords.insert("end".to_string(), KeyWordType::End);

        // Bool operators
        keywords.insert("and".to_string(), KeyWordType::And);
        keywords.insert("or".to_string(), KeyWordType::Or);
        keywords.insert("not".to_string(), KeyWordType::Not);

        // Math Functions
        func_keywords.insert("sin".to_string(), FuncKeyWord::Sin);
        func_keywords.insert("cos".to_string(), FuncKeyWord::Cos);
        func_keywords.insert("abs".to_string(), FuncKeyWord::Abs);
        func_keywords.insert("sqrt".to_string(), FuncKeyWord::Sqrt);
        func_keywords.insert("random".to_string(), FuncKeyWord::Random);
        func_keywords.insert("min".to_string(), FuncKeyWord::Min);
        func_keywords.insert("max".to_string(), FuncKeyWord::Max);

        // UserFunctions definitions
        keywords.insert("fn".to_string(), KeyWordType::Func);
        keywords.insert("return".to_string(), KeyWordType::Return);

        // TRUE and FALSE
        bool_keywords.insert("true".to_string(), true);
        bool_keywords.insert("false".to_string(), false);

        SyntaxDict {
            keywords,
            func_keywords,
            type_keywords,
            bool_keywords,
        }
    }

    // ==================== RUSSIAN ====================
    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        let mut func_keywords = HashMap::new();
        let mut type_keywords = HashMap::new();
        let mut bool_keywords = HashMap::new();
        // Types
        type_keywords.insert("цел".to_string(), VarType::Int);
        type_keywords.insert("вещ".to_string(), VarType::Float);
        type_keywords.insert("бул".to_string(), VarType::Bool);
        type_keywords.insert("строка".to_string(), VarType::String);
        // Variables & Operations
        keywords.insert("пусть".to_string(), KeyWordType::Let);
        keywords.insert("печать".to_string(), KeyWordType::Print);
        keywords.insert("печатьнс".to_string(), KeyWordType::Println);
        keywords.insert("ввод".to_string(), KeyWordType::Input);
        // Conditionals
        keywords.insert("если".to_string(), KeyWordType::If);
        keywords.insert("то".to_string(), KeyWordType::Then);
        keywords.insert("иначе".to_string(), KeyWordType::Else);
        // Loops
        keywords.insert("пока".to_string(), KeyWordType::While);
        keywords.insert("конец_пока".to_string(), KeyWordType::Wend);
        keywords.insert("для".to_string(), KeyWordType::For);
        keywords.insert("до".to_string(), KeyWordType::To);
        keywords.insert("шаг".to_string(), KeyWordType::Step);
        keywords.insert("следующий".to_string(), KeyWordType::Next);
        // Jumps & Utilities
        keywords.insert("стоп".to_string(), KeyWordType::End);
        // Bool operators
        keywords.insert("и".to_string(), KeyWordType::And);
        keywords.insert("или".to_string(), KeyWordType::Or);
        keywords.insert("не".to_string(), KeyWordType::Not);
        // Math Functions
        func_keywords.insert("sin".to_string(), FuncKeyWord::Sin);
        func_keywords.insert("cos".to_string(), FuncKeyWord::Cos);
        func_keywords.insert("abs".to_string(), FuncKeyWord::Abs);
        func_keywords.insert("sqrt".to_string(), FuncKeyWord::Sqrt);
        func_keywords.insert("random".to_string(), FuncKeyWord::Random);
        func_keywords.insert("min".to_string(), FuncKeyWord::Min);
        func_keywords.insert("max".to_string(), FuncKeyWord::Max);
        // UserFunctions definitions
        keywords.insert("функция".to_string(), KeyWordType::Func);
        keywords.insert("вернуть".to_string(), KeyWordType::Return);
        // TRUE and FALSE
        bool_keywords.insert("правда".to_string(), true);
        bool_keywords.insert("ложь".to_string(), false);
    
        Self {
            keywords,
            func_keywords,
            type_keywords,
            bool_keywords,
        }
    }
    // // ==================== JAPANESE (日本語) ====================
    // fn japanese_style() -> Self {
    //     let mut keywords = HashMap::new();
    //     let mut func_keywords = HashMap::new();
    //     let mut type_keywords = HashMap::new();

    //     // Types
    //     type_keywords.insert("INT".to_string(), VarType::Int);
    //     type_keywords.insert("FLOAT".to_string(), VarType::Float);
    //     type_keywords.insert("BOOL".to_string(), VarType::Bool);
    //     type_keywords.insert("STRING".to_string(), VarType::String);

    //     // Variables & Operations
    //     keywords.insert("代入".to_string(), KeyWordType::Let); 
    //     keywords.insert("表示".to_string(), KeyWordType::Print); 
    //     keywords.insert("入力".to_string(), KeyWordType::Input); 

    //     // Conditionals
    //     keywords.insert("もし".to_string(), KeyWordType::If); 
    //     keywords.insert("ならば".to_string(), KeyWordType::Then); 
    //     keywords.insert("違う".to_string(), KeyWordType::Else); 

    //     // Loops
    //     keywords.insert("間".to_string(), KeyWordType::While); 
    //     keywords.insert("繰り返す".to_string(), KeyWordType::Wend); 
    //     keywords.insert("為".to_string(), KeyWordType::For); 
    //     keywords.insert("まで".to_string(), KeyWordType::To); 
    //     keywords.insert("歩数".to_string(), KeyWordType::Step); 
    //     keywords.insert("次".to_string(), KeyWordType::Next); 

    //     // Jumps & Utilities
    //     keywords.insert("行け".to_string(), KeyWordType::Goto); 
    //     keywords.insert("終了".to_string(), KeyWordType::End);
    //     // shuuryou - end/quit
    //     // Math Functions
    //     func_keywords.insert("SIN".to_string(), FuncKeyWord::Sin);
    //     func_keywords.insert("COS".to_string(), FuncKeyWord::Cos);
    //     func_keywords.insert("ABS".to_string(), FuncKeyWord::Abs);
    //     func_keywords.insert("SQRT".to_string(), FuncKeyWord::Sqrt);
    //     func_keywords.insert("RANDOM".to_string(), FuncKeyWord::Random);
    //     func_keywords.insert("MIN".to_string(), FuncKeyWord::Min);
    //     func_keywords.insert("MAX".to_string(), FuncKeyWord::Max);

    //     Self {
    //         keywords,
    //         func_keywords,
    //         type_keywords,
    //     }
    // }

    pub fn get_dict(name_of_dict: &str) -> SyntaxDict {
        match name_of_dict {
            "English" => Self::default_english(),
            "Russian" => Self::russian_style(),
            // "JAPANESE" => Self::japanese_style(),
            _ => Self::default_english(),
        }
    }

    pub fn get_kw_word(&self, name_of_kw: KeyWordType) -> String {
        for (string_name, kw_type) in &self.keywords {
            if *kw_type == name_of_kw {
                return string_name.clone();
            }
        }
        format!("{:?}", name_of_kw)
    }
}
