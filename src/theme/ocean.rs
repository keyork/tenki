use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Ocean theme — cool cyan/blue palette for dark terminals.
pub struct OceanTheme;

impl Theme for OceanTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(223),
            ArtColor::SunRay => ansi(152),
            ArtColor::CloudLight => ansi(153),
            ArtColor::CloudDark => ansi(110),
            ArtColor::RainDrop => ansi(45),
            ArtColor::SnowFlake => ansi(195),
            ArtColor::Lightning => ansi(229),
            ArtColor::FogMist => ansi(117),
            ArtColor::MoonBody => ansi(153),
            ArtColor::Star => ansi(159),
            ArtColor::Ground => ansi(31),
        }
    }

    fn title_color(&self) -> Color {
        ansi(81)
    }
    fn temp_color(&self) -> Color {
        ansi(117)
    }
    fn info_color(&self) -> Color {
        ansi(153)
    }
    fn dim_color(&self) -> Color {
        ansi(67)
    }
    fn border_color(&self) -> Color {
        ansi(60)
    }
    fn highlight_color(&self) -> Color {
        ansi(215)
    }
    fn cold_color(&self) -> Color {
        ansi(45)
    }

    fn chart_color(&self, normalized: f64) -> Color {
        if normalized < 0.33 {
            ansi(45)
        } else if normalized < 0.66 {
            ansi(117)
        } else {
            ansi(215)
        }
    }
}
