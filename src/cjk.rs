use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Returns the terminal display width of a string in columns (handling CJK full-width).
#[must_use]
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Truncates a string to fit within `max_width` columns, adding an ellipsis if truncated.
/// Ensures no CJK characters are cut in half.
#[must_use]
pub fn truncate_to_width(s: &str, max_width: usize, ellipsis: &str) -> String {
    let current_width = display_width(s);
    if current_width <= max_width {
        return s.to_string();
    }

    let ellipsis_width = display_width(ellipsis);
    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let target_width = max_width - ellipsis_width;
    let mut accumulated_width = 0;
    let mut result = String::new();

    for c in s.chars() {
        let char_width = UnicodeWidthChar::width(c).unwrap_or(0);
        if accumulated_width + char_width > target_width {
            break;
        }
        result.push(c);
        accumulated_width += char_width;
    }

    result.push_str(ellipsis);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width("你好"), 4); // Each Chinese char is 2 columns
        assert_eq!(display_width("中国 China"), 10); // 4 + 1 + 5 = 10
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("你好世界", 6, ".."), "你好..");
        assert_eq!(truncate_to_width("hello world", 8, "..."), "hello...");
        assert_eq!(truncate_to_width("中文", 10, "..."), "中文");
    }
}
