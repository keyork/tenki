use super::colors::ArtColor;

/// A single colored text segment within an art line.
#[derive(Debug, Clone)]
pub struct ArtSegment {
    pub text: String,
    pub color: ArtColor,
}

/// One row of the ASCII art (multiple colored segments).
pub type ArtLine = Vec<ArtSegment>;

/// The complete ASCII art piece (multiple rows).
pub type ArtPiece = Vec<ArtLine>;

macro_rules! seg {
    ($t:expr, $c:expr) => {
        ArtSegment {
            text: $t.to_string(),
            color: $c,
        }
    };
}

fn line(segments: Vec<ArtSegment>) -> ArtLine {
    segments
}

pub const ART_WIDTH: usize = 22;

fn mono(text: &str, color: ArtColor) -> ArtLine {
    let mut out: String = text.chars().take(ART_WIDTH).collect();
    let pad = ART_WIDTH.saturating_sub(out.chars().count());
    if pad > 0 {
        out.push_str(&" ".repeat(pad));
    }
    vec![seg!(out, color)]
}

fn empty() -> ArtLine {
    mono("", ArtColor::CloudLight)
}

// ── Clear Sky (day) ──────────────────────────────────────────────────────────
pub fn clear_sky_day() -> ArtPiece {
    vec![
        mono("    \\   /", ArtColor::SunRay),
        mono("     .-.", ArtColor::SunRay),
        mono("  -- (   ) --", ArtColor::SunCore),
        mono("     `-'", ArtColor::SunRay),
        mono("    /   \\", ArtColor::SunRay),
        empty(),
        empty(),
        empty(),
    ]
}

// ── Clear Sky (night) ────────────────────────────────────────────────────────
pub fn clear_sky_night() -> ArtPiece {
    vec![
        mono("   .      .      *", ArtColor::Star),
        mono("      _..._", ArtColor::MoonBody),
        mono("    .::::::.", ArtColor::MoonBody),
        mono("   :::::::::", ArtColor::MoonBody),
        mono("   `:::::::'", ArtColor::MoonBody),
        mono("      `'::'", ArtColor::MoonBody),
        mono(" *      .      .", ArtColor::Star),
        empty(),
    ]
}

// ── Partly Cloudy (day) ──────────────────────────────────────────────────────
pub fn partly_cloudy_day() -> ArtPiece {
    vec![
        mono("   \\  /", ArtColor::SunRay),
        line(vec![
            seg!(" _ /\"\"", ArtColor::SunRay),
            seg!(".-.             ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("   \\_", ArtColor::SunRay),
            seg!("(   ).           ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("   /", ArtColor::SunRay),
            seg!("(___(__)          ", ArtColor::CloudLight),
        ]),
        empty(),
        empty(),
        empty(),
        empty(),
    ]
}

// ── Partly Cloudy (night) ────────────────────────────────────────────────────
pub fn partly_cloudy_night() -> ArtPiece {
    vec![
        mono("   .      *", ArtColor::Star),
        line(vec![
            seg!("     _..._  ", ArtColor::MoonBody),
            seg!(".--.      ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("   .::::. `.", ArtColor::MoonBody),
            seg!(" (   ).   ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("  ::::::.  :", ArtColor::MoonBody),
            seg!(" (___(__) ", ArtColor::CloudLight),
        ]),
        mono("   `:::::' .'", ArtColor::MoonBody),
        mono("     `'::-'", ArtColor::MoonBody),
        mono(" *        .", ArtColor::Star),
        empty(),
    ]
}

// ── Overcast ─────────────────────────────────────────────────────────────────
pub fn overcast() -> ArtPiece {
    vec![
        empty(),
        mono("      .--.  .--.", ArtColor::CloudDark),
        mono("   .-(    )(    ).", ArtColor::CloudDark),
        mono("  (___.__)(___.__)", ArtColor::CloudDark),
        mono("   .-(____)(____)-.", ArtColor::CloudDark),
        mono("  (___.__)(___.__)", ArtColor::CloudDark),
        mono("     `-..-''-..-'", ArtColor::CloudDark),
        empty(),
    ]
}

// ── Fog ──────────────────────────────────────────────────────────────────────
pub fn fog() -> ArtPiece {
    vec![
        empty(),
        mono(" _ - _ - _ - _ -", ArtColor::FogMist),
        mono("  _ - _ - _ - _", ArtColor::FogMist),
        mono(" _ - _ - _ - _ -", ArtColor::FogMist),
        mono("  _ - _ - _ - _", ArtColor::FogMist),
        mono(" _ - _ - _ - _ -", ArtColor::FogMist),
        mono("  _ - _ - _ - _", ArtColor::FogMist),
        empty(),
    ]
}

// ── Light Drizzle ─────────────────────────────────────────────────────────────
pub fn light_drizzle() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudLight),
        mono("  .-(    ).", ArtColor::CloudLight),
        mono(" (___.__)__)", ArtColor::CloudLight),
        mono("  .  .  .  .", ArtColor::RainDrop),
        mono("   .  .  .", ArtColor::RainDrop),
        mono("  .  .  .  .", ArtColor::RainDrop),
        empty(),
    ]
}

// ── Light Rain ───────────────────────────────────────────────────────────────
pub fn light_rain() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudDark),
        mono("  .-(    ).", ArtColor::CloudDark),
        mono(" (___.__)__)", ArtColor::CloudDark),
        mono("  /  /  /  /", ArtColor::RainDrop),
        mono("   /  /  /", ArtColor::RainDrop),
        mono("  /  /  /  /", ArtColor::RainDrop),
        empty(),
    ]
}

// ── Heavy Rain ───────────────────────────────────────────────────────────────
pub fn heavy_rain() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudDark),
        mono("  .-(    ).", ArtColor::CloudDark),
        mono(" (___.__)__)", ArtColor::CloudDark),
        mono("  // // // //", ArtColor::RainDrop),
        mono("  || || || ||", ArtColor::RainDrop),
        mono("  // // // //", ArtColor::RainDrop),
        mono("  || || || ||", ArtColor::RainDrop),
    ]
}

// ── Thunderstorm ─────────────────────────────────────────────────────────────
pub fn thunderstorm() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudDark),
        mono("  .-(    ).", ArtColor::CloudDark),
        mono(" (___.__)__)", ArtColor::CloudDark),
        mono("  // // // //", ArtColor::RainDrop),
        mono("     /\\/\\  /\\/\\", ArtColor::Lightning),
        mono("  || || || ||", ArtColor::RainDrop),
        mono("      \\/    \\/", ArtColor::Lightning),
    ]
}

// ── Light Snow ───────────────────────────────────────────────────────────────
pub fn light_snow() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudLight),
        mono("  .-(    ).", ArtColor::CloudLight),
        mono(" (___.__)__)", ArtColor::CloudLight),
        mono("    *   *   *", ArtColor::SnowFlake),
        mono("      *   *", ArtColor::SnowFlake),
        mono("    *   *   *", ArtColor::SnowFlake),
        empty(),
    ]
}

// ── Heavy Snow ───────────────────────────────────────────────────────────────
pub fn heavy_snow() -> ArtPiece {
    vec![
        empty(),
        mono("     .--.", ArtColor::CloudDark),
        mono("  .-(    ).", ArtColor::CloudDark),
        mono(" (___.__)__)", ArtColor::CloudDark),
        mono("   * * * * * * *", ArtColor::SnowFlake),
        mono("  * * * * * * * *", ArtColor::SnowFlake),
        mono("   * * * * * * *", ArtColor::SnowFlake),
        mono("  * * * * * * * *", ArtColor::SnowFlake),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn width(line: &ArtLine) -> usize {
        line.iter().map(|seg| seg.text.chars().count()).sum()
    }

    fn all_pieces() -> Vec<ArtPiece> {
        vec![
            clear_sky_day(),
            clear_sky_night(),
            partly_cloudy_day(),
            partly_cloudy_night(),
            overcast(),
            fog(),
            light_drizzle(),
            light_rain(),
            heavy_rain(),
            thunderstorm(),
            light_snow(),
            heavy_snow(),
        ]
    }

    #[test]
    fn every_piece_has_expected_height() {
        for piece in all_pieces() {
            assert_eq!(piece.len(), 8);
        }
    }

    #[test]
    fn every_line_matches_art_width() {
        for piece in all_pieces() {
            for line in piece {
                assert_eq!(width(&line), ART_WIDTH);
            }
        }
    }
}
