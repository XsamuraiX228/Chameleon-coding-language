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

    fn default_french() -> Self {
        let mut keywords = HashMap::new();
        
        // Переменные и операции
        keywords.insert("SOIT".to_string(), KeyWordType::Let);       // Вместо LET (Пусть...)
        keywords.insert("AFFICHER".to_string(), KeyWordType::Print); // Вместо PRINT (Показать/Вывести)
        keywords.insert("LIRE".to_string(), KeyWordType::Input);     // Вместо INPUT (Считать/Ввести)
        
        // Условия
        keywords.insert("SI".to_string(), KeyWordType::If);          // Вместо IF
        keywords.insert("ALORS".to_string(), KeyWordType::Then);     // Вместо THEN
        keywords.insert("SINON".to_string(), KeyWordType::Else);     // Вместо ELSE
        
        // Циклы
        keywords.insert("TANT_QUE".to_string(), KeyWordType::While); // Вместо WHILE (Пока...)
        keywords.insert("FIN_TANT".to_string(), KeyWordType::Wend);  // Вместо WEND (Конец ПОКА)
        keywords.insert("POUR".to_string(), KeyWordType::For);       // Вместо FOR
        keywords.insert("A".to_string(), KeyWordType::To);           // Вместо TO (До...)
        keywords.insert("PAS".to_string(), KeyWordType::Step);       // Вместо STEP (Шаг)
        keywords.insert("SUIVANT".to_string(), KeyWordType::Next);   // Вместо NEXT (Следующий)
        
        // Прыжки и утилиты
        keywords.insert("ALLER_A".to_string(), KeyWordType::Goto);   // Вместо GOTO
        keywords.insert("ALEATOIRE".to_string(), KeyWordType::Random); // Вместо RANDOM
        keywords.insert("FIN".to_string(), KeyWordType::End);        // Вместо END
        
        Self { keywords }
    }

    // ==================== EMOJI (Emoji Language) ====================
    fn emoji_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations ✍️
        keywords.insert("✍️".to_string(), KeyWordType::Let);      // Writing hand
        keywords.insert("🖨️".to_string(), KeyWordType::Print);    // Printer
        keywords.insert("⌨️".to_string(), KeyWordType::Input);    // Keyboard
        
        // Conditionals ❓
        keywords.insert("❓".to_string(), KeyWordType::If);        // Question mark
        keywords.insert("➡️".to_string(), KeyWordType::Then);      // Right arrow
        keywords.insert("↩️".to_string(), KeyWordType::Else);      // Return arrow
        
        // Loops 🔄
        keywords.insert("🔄".to_string(), KeyWordType::While);     // Arrows in circle
        keywords.insert("⏹️".to_string(), KeyWordType::Wend);      // Stop button
        keywords.insert("🔢".to_string(), KeyWordType::For);       // Numbers
        keywords.insert("📍".to_string(), KeyWordType::To);        // Pin
        keywords.insert("👣".to_string(), KeyWordType::Step);      // Footprints
        keywords.insert("⏭️".to_string(), KeyWordType::Next);      // Next track
        
        // Jumps & Utilities 🚀
        keywords.insert("🚀".to_string(), KeyWordType::Goto);      // Rocket
        keywords.insert("🎲".to_string(), KeyWordType::Random);    // Dice
        keywords.insert("🏁".to_string(), KeyWordType::End);       // Checkered flag
        
        Self { keywords }
    }

    // ==================== CRAB RAVE 🦀 ====================
    fn crab_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations 🦀
        keywords.insert("🦀".to_string(), KeyWordType::Let);       // Crab
        keywords.insert("📢".to_string(), KeyWordType::Print);     // Megaphone
        keywords.insert("⚓".to_string(), KeyWordType::Input);     // Anchor
        
        // Conditionals 🌊
        keywords.insert("🌊".to_string(), KeyWordType::If);        // Wave
        keywords.insert("🚢".to_string(), KeyWordType::Then);      // Ship
        keywords.insert("🐚".to_string(), KeyWordType::Else);      // Shell
        
        // Loops ♻️
        keywords.insert("♻️".to_string(), KeyWordType::While);     // Recycle
        keywords.insert("🛑".to_string(), KeyWordType::Wend);      // Stop sign
        keywords.insert("🦞".to_string(), KeyWordType::For);       // Lobster
        keywords.insert("🎯".to_string(), KeyWordType::To);        // Target
        keywords.insert("🦶".to_string(), KeyWordType::Step);      // Foot
        keywords.insert("🔜".to_string(), KeyWordType::Next);      // Soon arrow
        
        // Jumps & Utilities 🚀
        keywords.insert("🚀".to_string(), KeyWordType::Goto);      // Rocket
        keywords.insert("🎲".to_string(), KeyWordType::Random);    // Dice
        keywords.insert("🏁".to_string(), KeyWordType::End);       // Checkered flag
        
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

    // ==================== ELF (Elvish / Tolkien) ====================
    fn elf_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations
        keywords.insert("tEst".to_string(), KeyWordType::Let);       // Elvish "write" / carve
        keywords.insert("linna".to_string(), KeyWordType::Print);    // Elvish "sing" / recite
        keywords.insert("lasta".to_string(), KeyWordType::Input);    // Elvish "listen" / hear
        
        // Conditionals
        keywords.insert("ae".to_string(), KeyWordType::If);          // Elvish "when" / if
        keywords.insert("sui".to_string(), KeyWordType::Then);       // Elvish "then" / therefore
        keywords.insert("ab".to_string(), KeyWordType::Else);        // Elvish "but" / else
        
        // Loops
        keywords.insert("rena".to_string(), KeyWordType::While);     // Elvish "circle" / cycle
        keywords.insert("metta".to_string(), KeyWordType::Wend);     // Elvish "end" / finish
        keywords.insert("mena".to_string(), KeyWordType::For);       // Elvish "go" / travel
        keywords.insert("ten'".to_string(), KeyWordType::To);        // Elvish "toward" / to
        keywords.insert("pela".to_string(), KeyWordType::Step);      // Elvish "walk" / step
        keywords.insert("apha".to_string(), KeyWordType::Next);      // Elvish "follow" / next
        
        // Jumps & Utilities
        keywords.insert("mene".to_string(), KeyWordType::Goto);      // Elvish "depart" / goto
        keywords.insert("ambar".to_string(), KeyWordType::Random);   // Elvish "destiny" / random
        keywords.insert("tele".to_string(), KeyWordType::End);       // Elvish "complete" / end
        
        Self { keywords }
    }

    // ==================== PIRATE (Pirate Speak) ====================
    fn pirate_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Variables & Operations
        keywords.insert("SET".to_string(), KeyWordType::Let);        // Set sail!
        keywords.insert("SHOUT".to_string(), KeyWordType::Print);    // Shout from the crow's nest
        keywords.insert("PLUNDER".to_string(), KeyWordType::Input);  // Plunder the treasure
        
        // Conditionals
        keywords.insert("IF".to_string(), KeyWordType::If);          // If ye dare...
        keywords.insert("THEN".to_string(), KeyWordType::Then);      // Then walk the plank!
        keywords.insert("ELSE".to_string(), KeyWordType::Else);      // Or else...
        
        // Loops
        keywords.insert("WHILE".to_string(), KeyWordType::While);    // While the seas be rough...
        keywords.insert("WEND".to_string(), KeyWordType::Wend);      // End o' the storm
        keywords.insert("FOR".to_string(), KeyWordType::For);        // For each piece o' eight
        keywords.insert("TO".to_string(), KeyWordType::To);          // To the horizon
        keywords.insert("STEP".to_string(), KeyWordType::Step);      // Step lively!
        keywords.insert("NEXT".to_string(), KeyWordType::Next);      // Next port
        
        // Jumps & Utilities
        keywords.insert("GO".to_string(), KeyWordType::Goto);        // Go to port!
        keywords.insert("DICE".to_string(), KeyWordType::Random);    // Roll the bones
        keywords.insert("BURY".to_string(), KeyWordType::End);       // Bury the treasure
        
        Self { keywords }
    }

    fn mix_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Переменные берем из Rust-краба, ввод/вывод из английского и русского
        keywords.insert("🦀".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        
        // Условия делаем японскими
        keywords.insert("もし".to_string(), KeyWordType::If);
        keywords.insert("ならば".to_string(), KeyWordType::Then);
        keywords.insert("違う".to_string(), KeyWordType::Else);
        
        // Циклы: старт по-русски, границы по-английски, шаг крабовый, закрытие японское
        keywords.insert("ДЛЯ".to_string(), KeyWordType::For);
        keywords.insert("TO".to_string(), KeyWordType::To);
        keywords.insert("👣".to_string(), KeyWordType::Step);
        keywords.insert("次".to_string(), KeyWordType::Next);
        
        // Утилиты
        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("乱数".to_string(), KeyWordType::Random);
        keywords.insert("СТОП".to_string(), KeyWordType::End);
        
        Self { keywords }
    }

    fn kumir_style() -> Self {
        let mut keywords = HashMap::new();
        
        // Переменные и операции
        keywords.insert("знач".to_string(), KeyWordType::Let);       // Вместо LET (или "присвоить")
        keywords.insert("вывод".to_string(), KeyWordType::Print);    // Вместо PRINT
        keywords.insert("ввод".to_string(), KeyWordType::Input);     // Вместо INPUT
        
        // Условия
        keywords.insert("если".to_string(), KeyWordType::If);        // Вместо IF
        keywords.insert("то".to_string(), KeyWordType::Then);        // Вместо THEN
        keywords.insert("иначе".to_string(), KeyWordType::Else);     // Вместо ELSE
        
        // Циклы
        keywords.insert("нц_пока".to_string(), KeyWordType::While);  // Начало цикла пока (WHILE)
        keywords.insert("кц".to_string(), KeyWordType::Wend);        // Конец цикла (WEND)
        keywords.insert("нц_для".to_string(), KeyWordType::For);     // Начало цикла для (FOR)
        keywords.insert("до".to_string(), KeyWordType::To);          // До (TO)
        keywords.insert("шаг".to_string(), KeyWordType::Step);       // Шаг (STEP)
        keywords.insert("кц_для".to_string(), KeyWordType::Next);    // Конец цикла для (NEXT)
        
        // Переходы и утилиты
        keywords.insert("переход".to_string(), KeyWordType::Goto);   // Вместо GOTO
        keywords.insert("случайное".to_string(), KeyWordType::Random); // Вместо RANDOM
        keywords.insert("все".to_string(), KeyWordType::End);        // Конец условия (END в Кумире — это "все")
        
        Self { keywords }
    }

    fn default_hindi() -> Self {
        let mut keywords = HashMap::new();
        
        // वेरिएबल्स और ऑपरेशंस (Variables & Operations)
        keywords.insert("मानो".to_string(), KeyWordType::Let);       // LET (मानो x = 10)
        keywords.insert("दिखाओ".to_string(), KeyWordType::Print);    // PRINT (दिखाओ x)
        keywords.insert("लो".to_string(), KeyWordType::Input);        // INPUT (लो x)
        
        // शर्तें (Conditionals)
        keywords.insert("यदि".to_string(), KeyWordType::If);          // IF
        keywords.insert("तो".to_string(), KeyWordType::Then);        // THEN
        keywords.insert("वरना".to_string(), KeyWordType::Else);       // ELSE
        
        // लूप्स (Loops)
        keywords.insert("जबतक".to_string(), KeyWordType::While);     // WHILE
        keywords.insert("अन्तजब".to_string(), KeyWordType::Wend);     // WEND (End While)
        keywords.insert("शुरू".to_string(), KeyWordType::For);       // FOR
        keywords.insert("तक".to_string(), KeyWordType::To);          // TO
        keywords.insert("कदम".to_string(), KeyWordType::Step);       // STEP
        keywords.insert("अगला".to_string(), KeyWordType::Next);       // NEXT
        
        // जम्प्स और यूटिलिटीज (Jumps & Utilities)
        keywords.insert("जाओ".to_string(), KeyWordType::Goto);        // GOTO
        keywords.insert("रैंडम".to_string(), KeyWordType::Random);    // RANDOM
        keywords.insert("अन्त".to_string(), KeyWordType::End);        // END
        
        Self { keywords }
    }

    fn python_style() -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("LET".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("INPUT".to_string(), KeyWordType::Input);

        keywords.insert("IF".to_string(), KeyWordType::If);
        keywords.insert("THEN".to_string(), KeyWordType::Then);
        keywords.insert("ELSE".to_string(), KeyWordType::Else);

        keywords.insert("WHILE".to_string(), KeyWordType::While);
        keywords.insert("ENDWHILE".to_string(), KeyWordType::Wend);

        keywords.insert("FOR".to_string(), KeyWordType::For);
        keywords.insert("IN".to_string(), KeyWordType::To);
        keywords.insert("STEP".to_string(), KeyWordType::Step);
        keywords.insert("ENDFOR".to_string(), KeyWordType::Next);

        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("RANDOM".to_string(), KeyWordType::Random);
        keywords.insert("QUIT".to_string(), KeyWordType::End);

        Self { keywords }
    }

    fn friend_style() -> Self {
        let mut keywords = HashMap::new();

        // Variables & Operations
        keywords.insert("SET".to_string(), KeyWordType::Let);
        keywords.insert("CARE".to_string(), KeyWordType::Print);
        keywords.insert("GAIN".to_string(), KeyWordType::Input);

        // Conditionals
        keywords.insert("MEAN".to_string(), KeyWordType::If);
        keywords.insert("DEFINE".to_string(), KeyWordType::Then);
        keywords.insert("CONCEAL".to_string(), KeyWordType::Else);

        // Loops
        keywords.insert("ROOT".to_string(), KeyWordType::While);
        keywords.insert("REAR".to_string(), KeyWordType::Wend);

        keywords.insert("MAJOR".to_string(), KeyWordType::For);
        keywords.insert("MINOR".to_string(), KeyWordType::To);
        keywords.insert("SPLIT".to_string(), KeyWordType::Step);
        keywords.insert("LOCKET".to_string(), KeyWordType::Next);

        // Jumps & Utilities
        keywords.insert("CORE".to_string(), KeyWordType::Goto);
        keywords.insert("TEMP".to_string(), KeyWordType::Random);
        keywords.insert("HIGH".to_string(), KeyWordType::End);

        Self { keywords }
    }

    pub fn get_dict(name_of_dict: &str) -> SyntaxDict {
        match name_of_dict {
            "RUSSIAN" => Self::russian_style(),
            "EMOJI" => Self::emoji_style(),
            "CRAB" => Self::crab_style(),
            "JAPANESE" => Self::japanese_style(),
            "ELF" => Self::elf_style(),
            "MIX" => Self::mix_style(),
            "KUMIR" => Self::kumir_style(),
            "FRENCH" => Self::default_french(),
            "HINDI" => Self::default_hindi(),
            "PYTHON" => Self::python_style(),
            "FREN" => Self::friend_style(),
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