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
fn match_paired<'a>(line: &'a str, marker: &str) -> Option<(&'a str, &'a str)> {
    let m_len = marker.len();

    // 1. Ищем самый первый маркер слева направо
    if let Some(start_idx) = line.find(marker) {
        let after_first = &line[start_idx + m_len..];

        // 2. Ищем первый попавшийся закрывающий маркер после него
        if let Some(end_idx) = after_first.find(marker) {
            let content = &after_first[..end_idx];
            // Хвост строки, который идет СРАЗУ после закрывающего маркера
            let remaining_tail = &after_first[end_idx + m_len..];

            // Возвращаем контент и остаток строки для следующей итерации
            return Some((content, remaining_tail));
        }
    }
    None
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
    if let Some((content, _tail)) = match_paired(line, "**") {
        return LineMarkup::Bold {
            content: content.to_string(),
            marker: "**".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "~~") {
        return LineMarkup::Strikethrough {
            content: content.to_string(),
            marker: "~~".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "*") {
        return LineMarkup::Italic {
            content: content.to_string(),
            marker: "*".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "_") {
        return LineMarkup::Italic {
            content: content.to_string(),
            marker: "_".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "^") {
        return LineMarkup::Superscript {
            content: content.to_string(),
            marker: "^".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "~") {
        return LineMarkup::Subscript {
            content: content.to_string(),
            marker: "~".to_string(),
        };
    }
    if let Some((content, _tail)) = match_paired(line, "`") {
        return LineMarkup::Code {
            content: content.to_string(),
            marker: "`".to_string(),
        };
    }

    // Если ничего не подошло – обычный текст
    LineMarkup::Plain(line.to_string())
}
