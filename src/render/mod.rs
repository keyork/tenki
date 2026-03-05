pub mod card;
pub mod chart;
pub mod compact;
pub mod fullscreen;
pub mod oneline;
pub mod scene;

use crate::{model::WeatherData, theme::Theme, units::Units};

pub struct RenderContext<'a> {
    pub data: &'a WeatherData,
    pub theme: &'a dyn Theme,
    pub units: Units,
    pub show_chart: bool,
    pub show_forecast: bool,
    pub animate: bool,
}

pub(crate) fn location_label(data: &WeatherData) -> String {
    let country = data.location.country.trim();
    if country.is_empty() {
        data.location.name.clone()
    } else {
        format!("{}, {}", data.location.name, country)
    }
}

/// Visible character width (strips ANSI escapes, counts wide chars as 2).
pub(crate) fn visible_len(s: &str) -> usize {
    let mut len = 0usize;
    let mut in_esc = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_esc = true;
            continue;
        }
        if in_esc {
            if c == 'm' {
                in_esc = false;
            }
            continue;
        }
        len += if is_wide(c) { 2 } else { 1 };
    }
    len
}

pub(crate) fn is_wide(c: char) -> bool {
    matches!(c,
        '\u{1100}'..='\u{115F}' |
        '\u{2E80}'..='\u{303E}' |
        '\u{3040}'..='\u{33FF}' |
        '\u{3400}'..='\u{4DBF}' |
        '\u{4E00}'..='\u{9FFF}' |
        '\u{A000}'..='\u{A4CF}' |
        '\u{AC00}'..='\u{D7AF}' |
        '\u{F900}'..='\u{FAFF}' |
        '\u{FE10}'..='\u{FE1F}' |
        '\u{FE30}'..='\u{FE4F}' |
        '\u{FF00}'..='\u{FF60}' |
        '\u{FFE0}'..='\u{FFE6}' |
        '\u{1F300}'..='\u{1F9FF}'
    )
}

pub(crate) fn pad_right(s: &str, width: usize) -> String {
    let len = visible_len(s);
    if len >= width {
        truncate_visible(s, width)
    } else {
        format!("{s}{}", " ".repeat(width - len))
    }
}

pub(crate) fn pad_center(s: &str, width: usize) -> String {
    let len = visible_len(s);
    if len >= width {
        truncate_visible(s, width)
    } else {
        let left = (width - len) / 2;
        let right = width - len - left;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
    }
}

pub(crate) fn truncate_visible(s: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let len = visible_len(s);
    if len <= width {
        return s.to_string();
    }

    if width <= 3 {
        return ".".repeat(width);
    }

    let mut out = String::new();
    let mut used = 0usize;
    let target = width - 3;

    for c in s.chars() {
        let char_w = if is_wide(c) { 2 } else { 1 };
        if used + char_w > target {
            break;
        }
        out.push(c);
        used += char_w;
    }

    while used < target {
        out.push(' ');
        used += 1;
    }
    out.push_str("...");
    out
}
