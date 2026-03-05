use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Light-terminal theme with darker colors for readability on white backgrounds.
pub struct LightTheme;

impl Theme for LightTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(172),    // deep gold
            ArtColor::SunRay => ansi(214),     // orange-gold
            ArtColor::CloudLight => ansi(240), // dark grey
            ArtColor::CloudDark => ansi(235),  // very dark grey
            ArtColor::RainDrop => ansi(25),    // deep blue
            ArtColor::SnowFlake => ansi(153),  // light blue-grey
            ArtColor::Lightning => ansi(214),  // orange
            ArtColor::FogMist => ansi(243),    // medium grey
            ArtColor::MoonBody => ansi(243),   // grey (moon on light bg)
            ArtColor::Star => ansi(214),       // orange-gold
            ArtColor::Ground => ansi(94),      // dark earth
        }
    }

    fn title_color(&self) -> Color {
        ansi(25)
    } // dark blue
    fn temp_color(&self) -> Color {
        ansi(172)
    } // deep gold
    fn info_color(&self) -> Color {
        ansi(238)
    } // dark grey
    fn dim_color(&self) -> Color {
        ansi(244)
    } // medium grey
    fn border_color(&self) -> Color {
        ansi(247)
    } // light border
    fn highlight_color(&self) -> Color {
        ansi(160)
    } // deep red
    fn cold_color(&self) -> Color {
        ansi(25)
    } // dark blue

    fn chart_color(&self, normalized: f64) -> Color {
        if normalized < 0.33 {
            ansi(25)
        } else if normalized < 0.66 {
            ansi(172)
        } else {
            ansi(160)
        }
    }
}
