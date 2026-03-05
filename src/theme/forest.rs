use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Forest theme — earthy green palette with warm highlights.
pub struct ForestTheme;

impl Theme for ForestTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(220),
            ArtColor::SunRay => ansi(190),
            ArtColor::CloudLight => ansi(151),
            ArtColor::CloudDark => ansi(108),
            ArtColor::RainDrop => ansi(71),
            ArtColor::SnowFlake => ansi(194),
            ArtColor::Lightning => ansi(227),
            ArtColor::FogMist => ansi(145),
            ArtColor::MoonBody => ansi(186),
            ArtColor::Star => ansi(223),
            ArtColor::Ground => ansi(94),
        }
    }

    fn title_color(&self) -> Color {
        ansi(114)
    }
    fn temp_color(&self) -> Color {
        ansi(220)
    }
    fn info_color(&self) -> Color {
        ansi(151)
    }
    fn dim_color(&self) -> Color {
        ansi(102)
    }
    fn border_color(&self) -> Color {
        ansi(65)
    }
    fn highlight_color(&self) -> Color {
        ansi(208)
    }
    fn cold_color(&self) -> Color {
        ansi(71)
    }

    fn chart_color(&self, normalized: f64) -> Color {
        if normalized < 0.33 {
            ansi(71)
        } else if normalized < 0.66 {
            ansi(150)
        } else {
            ansi(208)
        }
    }
}
