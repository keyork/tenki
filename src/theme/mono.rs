use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Monochrome grayscale theme — differentiated by brightness only.
pub struct MonoTheme;

impl Theme for MonoTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(255),
            ArtColor::SunRay => ansi(252),
            ArtColor::CloudLight => ansi(250),
            ArtColor::CloudDark => ansi(243),
            ArtColor::RainDrop => ansi(246),
            ArtColor::SnowFlake => ansi(255),
            ArtColor::Lightning => ansi(255),
            ArtColor::FogMist => ansi(248),
            ArtColor::MoonBody => ansi(253),
            ArtColor::Star => ansi(250),
            ArtColor::Ground => ansi(240),
        }
    }

    fn title_color(&self) -> Color {
        ansi(255)
    }
    fn temp_color(&self) -> Color {
        ansi(252)
    }
    fn info_color(&self) -> Color {
        ansi(248)
    }
    fn dim_color(&self) -> Color {
        ansi(242)
    }
    fn border_color(&self) -> Color {
        ansi(238)
    }
    fn highlight_color(&self) -> Color {
        ansi(255)
    }
    fn cold_color(&self) -> Color {
        ansi(248)
    }

    fn chart_color(&self, normalized: f64) -> Color {
        let v = 235 + (normalized * 18.0) as u8;
        ansi(v.min(253))
    }
}
