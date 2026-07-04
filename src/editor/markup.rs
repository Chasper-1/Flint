pub enum LineMarkup {
    Heading { content: String, marker: String },
    Bold { content: String, marker: String },
    Italic { content: String, marker: String },
    Strikethrough { content: String, marker: String },
    Superscript { content: String, marker: String },
    Subscript { content: String, marker: String },
    Code { content: String, marker: String },
    Plain(String),
}

/// 1. Функция ищет конкретный маркер СТРОГО в начале строки
fn match_start<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    if line.starts_with(marker) {
        Some(&line[marker.len()..])
    } else {
        None
    }
}

/// 2. Функция проверяет, обернута ли строка в парный маркер с двух сторон в любом месте
fn match_paired<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let m_len = marker.len();
    // Проверяем, что строка длиннее двух маркеров и начинается/заканчивается на них
    if line.len() > m_len * 2 && line.starts_with(marker) && line.ends_with(marker) {
        // Возвращаем строго ОДИН срез текста внутри маркеров (это &str, а не кортеж!)
        Some(&line[m_len..line.len() - m_len])
    } else {
        None
    }
}

/// Анализирует строку и возвращает тип разметки (без дублирования кода)
pub fn parse_line(line: &str) -> LineMarkup {
    // Проверяем блочные элементы (только в начале)
    if let Some(content) = match_start(line, "# ") {
        return LineMarkup::Heading {
            content: content.to_string(),
            marker: "# ".to_string(),
        };
    }

    // Проверяем парные элементы (срезаются со старта и конца всей строки)
    // Важно: проверяем "**" и "~~" раньше одиночных "*" и "~"
    if let Some(content) = match_paired(line, "**") {
        return LineMarkup::Bold {
            content: content.to_string(),
            marker: "**".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "~~") {
        return LineMarkup::Strikethrough {
            content: content.to_string(),
            marker: "~~".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "*") {
        return LineMarkup::Italic {
            content: content.to_string(),
            marker: "*".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "_") {
        return LineMarkup::Italic {
            content: content.to_string(),
            marker: "_".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "^") {
        return LineMarkup::Superscript {
            content: content.to_string(),
            marker: "^".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "~") {
        return LineMarkup::Subscript {
            content: content.to_string(),
            marker: "~".to_string(),
        };
    }
    if let Some(content) = match_paired(line, "`") {
        return LineMarkup::Code {
            content: content.to_string(),
            marker: "`".to_string(),
        };
    }

    // Если ничего не подошло – обычный текст
    LineMarkup::Plain(line.to_string())
}
