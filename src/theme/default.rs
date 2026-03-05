use super::{ansi, Theme};
use crate::art::colors::ArtColor;
use crossterm::style::Color;

/// Default dark-terminal theme.
pub struct DefaultTheme;

impl Theme for DefaultTheme {
    fn art_color(&self, c: ArtColor) -> Color {
        match c {
            ArtColor::SunCore => ansi(220),    // bright gold
            ArtColor::SunRay => ansi(228),     // light gold
            ArtColor::CloudLight => ansi(250), // bright grey
            ArtColor::CloudDark => ansi(245),  // mid grey
            ArtColor::RainDrop => ansi(111),   // light blue
            ArtColor::SnowFlake => ansi(255),  // bright white
            ArtColor::Lightning => ansi(226),  // bright yellow
            ArtColor::FogMist => ansi(249),    // fog grey
            ArtColor::MoonBody => ansi(230),   // warm white
            ArtColor::Star => ansi(228),       // warm yellow
            ArtColor::Ground => ansi(130),     // earth brown
        }
    }

    fn title_color(&self) -> Color {
        ansi(117)
    } // sky blue
    fn temp_color(&self) -> Color {
        ansi(220)
    } // warm gold
    fn info_color(&self) -> Color {
        ansi(250)
    } // silver grey
    fn dim_color(&self) -> Color {
        ansi(244)
    } // dark grey
    fn border_color(&self) -> Color {
        ansi(239)
    } // deep border
    fn highlight_color(&self) -> Color {
        ansi(203)
    } // coral (high temp)
    fn cold_color(&self) -> Color {
        ansi(117)
    } // ice blue (low temp)

    fn chart_color(&self, normalized: f64) -> Color {
        // cold → warm gradient across ANSI palette
        if normalized < 0.33 {
            ansi(117) // cold blue
        } else if normalized < 0.66 {
            ansi(220) // warm gold
        } else {
            ansi(203) // hot coral
        }
    }
}
