use std::collections::HashMap;
use crate::frontend::token::KeyWordType;


pub enum Dict {
    Russian,
    English,
    Crab,
    Emoji,
}

pub struct SyntaxDict {
    pub keywords: HashMap<String, KeyWordType>
}

#[allow(dead_code)]
impl SyntaxDict {
    fn default_english() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations
        keywords.insert("LET".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("INPUT".to_string(), KeyWordType::Input);
        
        // Conditionals
        keywords.insert("IF".to_string(), KeyWordType::If);
        keywords.insert("THEN".to_string(), KeyWordType::Then);
        keywords.insert("ELSE".to_string(), KeyWordType::Else);
        
        // Loops
        keywords.insert("WHILE".to_string(), KeyWordType::While);
        keywords.insert("WEND".to_string(), KeyWordType::Wend);
        keywords.insert("FOR".to_string(), KeyWordType::For);
        keywords.insert("TO".to_string(), KeyWordType::To);
        keywords.insert("STEP".to_string(), KeyWordType::Step);
        keywords.insert("NEXT".to_string(), KeyWordType::Next);
        
        // Jumps & Utilities
        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("RANDOM".to_string(), KeyWordType::Random);
        keywords.insert("END".to_string(), KeyWordType::End);

        // Bool operators
        keywords.insert("AND".to_string(), KeyWordType::And);
        keywords.insert("OR".to_string(), KeyWordType::Or);
        keywords.insert("NOT".to_string(), KeyWordType::Not);
        
        Self { keywords }
    }

    // ==================== RUSSIAN (Русский) ====================
    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations
        keywords.insert("ПУСТЬ".to_string(), KeyWordType::Let);
        keywords.insert("ПЕЧАТЬ".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        
        // Conditionals
        keywords.insert("ЕСЛИ".to_string(), KeyWordType::If);
        keywords.insert("ТО".to_string(), KeyWordType::Then);
        keywords.insert("ИНАЧЕ".to_string(), KeyWordType::Else);
        
        // Loops
        keywords.insert("ПОКА".to_string(), KeyWordType::While);
        keywords.insert("КОНЕЦ_ПОКА".to_string(), KeyWordType::Wend);
        keywords.insert("ДЛЯ".to_string(), KeyWordType::For);
        keywords.insert("ДО".to_string(), KeyWordType::To);
        keywords.insert("ШАГ".to_string(), KeyWordType::Step);
        keywords.insert("СЛЕДУЮЩИЙ".to_string(), KeyWordType::Next);
        
        // Jumps & Utilities
        keywords.insert("ИДИ".to_string(), KeyWordType::Goto);
        keywords.insert("РАНДОМ".to_string(), KeyWordType::Random);
        keywords.insert("СТОП".to_string(), KeyWordType::End);
        
        Self { keywords }
    }
    // ==================== JAPANESE (日本語) ====================
    fn japanese_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations
        keywords.insert("代入".to_string(), KeyWordType::Let);      // dainyuu - assign
        keywords.insert("表示".to_string(), KeyWordType::Print);    // hyouji - display
        keywords.insert("入力".to_string(), KeyWordType::Input);    // nyuuryoku - input
        
        // Conditionals
        keywords.insert("もし".to_string(), KeyWordType::If);        // moshi - if
        keywords.insert("ならば".to_string(), KeyWordType::Then);    // naraba - then
        keywords.insert("違う".to_string(), KeyWordType::Else);      // chigau - else/different
        
        // Loops
        keywords.insert("間".to_string(), KeyWordType::While);       // aida - while/during
        keywords.insert("繰り返す".to_string(), KeyWordType::Wend);  // kurikaesu - repeat
        keywords.insert("為".to_string(), KeyWordType::For);        // tame - for
        keywords.insert("まで".to_string(), KeyWordType::To);        // made - until/to
        keywords.insert("歩数".to_string(), KeyWordType::Step);      // hosuu - step
        keywords.insert("次".to_string(), KeyWordType::Next);        // tsugi - next
        
        // Jumps & Utilities
        keywords.insert("行け".to_string(), KeyWordType::Goto);      // ike - go
        keywords.insert("乱数".to_string(), KeyWordType::Random);    // ransuu - random
        keywords.insert("終了".to_string(), KeyWordType::End);       // shuuryou - end/quit
        
        Self { keywords }
    }

    pub fn get_dict(name_of_dict: &str) -> SyntaxDict {
        match name_of_dict {
            "RUSSIAN" => Self::russian_style(),
            "JAPANESE" => Self::japanese_style(),
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