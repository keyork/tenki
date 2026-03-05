pub mod default;
pub mod light;
pub mod mono;

use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// A theme maps semantic color tokens to concrete ANSI 256 colors.
pub trait Theme {
    fn art_color(&self, c: ArtColor) -> Color;
    fn title_color(&self) -> Color;
    fn temp_color(&self) -> Color;
    fn info_color(&self) -> Color;
    fn dim_color(&self) -> Color;
    fn border_color(&self) -> Color;
    #[allow(dead_code)]
    fn highlight_color(&self) -> Color;
    #[allow(dead_code)]
    fn cold_color(&self) -> Color;
    fn chart_color(&self, normalized: f64) -> Color;
}

/// Resolve theme name to a boxed Theme.
pub fn resolve(name: &str) -> Box<dyn Theme> {
    match name {
        "light" => Box::new(light::LightTheme),
        "mono" => Box::new(mono::MonoTheme),
        _ => Box::new(default::DefaultTheme),
    }
}

pub fn ansi(n: u8) -> Color {
    Color::AnsiValue(n)
}
