use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Sunset theme — warm amber/orange/red palette.
pub struct SunsetTheme;

impl Theme for SunsetTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(214),
            ArtColor::SunRay => ansi(222),
            ArtColor::CloudLight => ansi(216),
            ArtColor::CloudDark => ansi(173),
            ArtColor::RainDrop => ansi(167),
            ArtColor::SnowFlake => ansi(230),
            ArtColor::Lightning => ansi(227),
            ArtColor::FogMist => ansi(180),
            ArtColor::MoonBody => ansi(223),
            ArtColor::Star => ansi(229),
            ArtColor::Ground => ansi(130),
        }
    }

    fn title_color(&self) -> Color {
        ansi(216)
    }
    fn temp_color(&self) -> Color {
        ansi(214)
    }
    fn info_color(&self) -> Color {
        ansi(223)
    }
    fn dim_color(&self) -> Color {
        ansi(138)
    }
    fn border_color(&self) -> Color {
        ansi(130)
    }
    fn highlight_color(&self) -> Color {
        ansi(203)
    }
    fn cold_color(&self) -> Color {
        ansi(109)
    }

    fn chart_color(&self, normalized: f64) -> Color {
        if normalized < 0.33 {
            ansi(109)
        } else if normalized < 0.66 {
            ansi(214)
        } else {
            ansi(203)
        }
    }
}
