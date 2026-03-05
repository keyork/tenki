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

fn plain(text: &str, color: ArtColor) -> ArtLine {
    vec![seg!(text, color)]
}

fn empty() -> ArtLine {
    vec![seg!("                      ", ArtColor::CloudLight)]
}

// ── Clear Sky (day) ──────────────────────────────────────────────────────────
//
//      \  ·  |  ·  /
//    ·  '───────'  ·
//  ─  (  ° · °  )  ─
//    ·  '───────'  ·
//      /  ·  |  ·  \
//
pub fn clear_sky_day() -> ArtPiece {
    vec![
        line(vec![seg!("     \\  ·  |  ·  /    ", ArtColor::SunRay)]),
        line(vec![
            seg!("   ·  ", ArtColor::SunRay),
            seg!("'───────'", ArtColor::SunRay),
            seg!("  ·    ", ArtColor::SunRay),
        ]),
        line(vec![
            seg!("  ─  ", ArtColor::SunRay),
            seg!("(  ° · °  )", ArtColor::SunCore),
            seg!("  ─   ", ArtColor::SunRay),
        ]),
        line(vec![
            seg!("   ·  ", ArtColor::SunRay),
            seg!("'───────'", ArtColor::SunRay),
            seg!("  ·    ", ArtColor::SunRay),
        ]),
        line(vec![seg!("     /  ·  |  ·  \\    ", ArtColor::SunRay)]),
        empty(),
        empty(),
        empty(),
    ]
}

// ── Clear Sky (night) ────────────────────────────────────────────────────────
pub fn clear_sky_night() -> ArtPiece {
    vec![
        line(vec![seg!("  ·          ·        ", ArtColor::Star)]),
        line(vec![
            seg!("      ", ArtColor::Star),
            seg!(".─────.  ", ArtColor::MoonBody),
            seg!("  ·   ", ArtColor::Star),
        ]),
        line(vec![
            seg!("  ·  /", ArtColor::MoonBody),
            seg!("  ° °  ", ArtColor::MoonBody),
            seg!("\\  ·  ", ArtColor::MoonBody),
        ]),
        line(vec![
            seg!("    /", ArtColor::MoonBody),
            seg!("  °   °  ", ArtColor::MoonBody),
            seg!("\\    ", ArtColor::MoonBody),
        ]),
        line(vec![
            seg!("    \\", ArtColor::MoonBody),
            seg!("  °   °  ", ArtColor::MoonBody),
            seg!("/    ", ArtColor::MoonBody),
        ]),
        line(vec![
            seg!("  ·  \\", ArtColor::MoonBody),
            seg!("  ° °  ", ArtColor::MoonBody),
            seg!("/  ·  ", ArtColor::MoonBody),
        ]),
        line(vec![
            seg!(" ·    ", ArtColor::Star),
            seg!("'─────'", ArtColor::MoonBody),
            seg!("    ·  ", ArtColor::Star),
        ]),
        empty(),
    ]
}

// ── Partly Cloudy (day) ──────────────────────────────────────────────────────
pub fn partly_cloudy_day() -> ArtPiece {
    vec![
        line(vec![seg!("    \\  /              ", ArtColor::SunRay)]),
        line(vec![
            seg!("  _ /\"\"", ArtColor::SunRay),
            seg!(".-.", ArtColor::CloudLight),
            seg!("           ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("    \\_ ", ArtColor::SunRay),
            seg!("(   ).", ArtColor::CloudLight),
            seg!("          ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("       ", ArtColor::CloudLight),
            seg!("(___(__)", ArtColor::CloudLight),
            seg!("         ", ArtColor::CloudLight),
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
        line(vec![seg!("  ·                   ", ArtColor::Star)]),
        line(vec![
            seg!("   (", ArtColor::MoonBody),
            seg!(") ", ArtColor::MoonBody),
            seg!(".-.", ArtColor::CloudLight),
            seg!("            ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("    ", ArtColor::MoonBody),
            seg!("  ", ArtColor::MoonBody),
            seg!("(   ).", ArtColor::CloudLight),
            seg!("         ", ArtColor::CloudLight),
        ]),
        line(vec![
            seg!("       ", ArtColor::CloudLight),
            seg!("(___(__)", ArtColor::CloudLight),
            seg!("        ", ArtColor::CloudLight),
        ]),
        line(vec![seg!("  ·               ·   ", ArtColor::Star)]),
        empty(),
        empty(),
        empty(),
    ]
}

// ── Overcast ─────────────────────────────────────────────────────────────────
pub fn overcast() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudDark)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudDark)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudDark)]),
        line(vec![seg!("   .--.               ", ArtColor::CloudDark)]),
        line(vec![seg!(" -(      )-           ", ArtColor::CloudDark)]),
        line(vec![seg!("(_______)__)          ", ArtColor::CloudDark)]),
        empty(),
    ]
}

// ── Fog ──────────────────────────────────────────────────────────────────────
//   Layered ≈ waves for a misty atmosphere
pub fn fog() -> ArtPiece {
    vec![
        empty(),
        plain("   ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈    ", ArtColor::FogMist),
        plain("  ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈   ", ArtColor::FogMist),
        plain("   ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈    ", ArtColor::FogMist),
        plain("  ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈   ", ArtColor::FogMist),
        plain("   ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈    ", ArtColor::FogMist),
        plain("  ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈ ≈   ", ArtColor::FogMist),
        empty(),
    ]
}

// ── Light Drizzle ─────────────────────────────────────────────────────────────
pub fn light_drizzle() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudLight)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudLight)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudLight)]),
        line(vec![seg!("   ´ ´ ´ ´            ", ArtColor::RainDrop)]),
        line(vec![seg!("  ´ ´ ´ ´             ", ArtColor::RainDrop)]),
        empty(),
        empty(),
    ]
}

// ── Light Rain ───────────────────────────────────────────────────────────────
pub fn light_rain() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudDark)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudDark)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudDark)]),
        line(vec![seg!("  ╱ ╱ ╱ ╱            ", ArtColor::RainDrop)]),
        line(vec![seg!(" ╱ ╱ ╱ ╱             ", ArtColor::RainDrop)]),
        empty(),
        empty(),
    ]
}

// ── Heavy Rain ───────────────────────────────────────────────────────────────
pub fn heavy_rain() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudDark)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudDark)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudDark)]),
        line(vec![seg!(" ‖╱‖╱‖╱‖╱           ", ArtColor::RainDrop)]),
        line(vec![seg!(" ╱‖╱‖╱‖╱‖           ", ArtColor::RainDrop)]),
        line(vec![seg!(" ‖╱‖╱‖╱‖╱           ", ArtColor::RainDrop)]),
        empty(),
    ]
}

// ── Thunderstorm ─────────────────────────────────────────────────────────────
//   Uses ϟ (U+03DF Greek small letter koppa) for lightning — not emoji
pub fn thunderstorm() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudDark)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudDark)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudDark)]),
        line(vec![
            seg!("  ", ArtColor::RainDrop),
            seg!("ϟ", ArtColor::Lightning),
            seg!("╱ ╱", ArtColor::RainDrop),
            seg!("ϟ", ArtColor::Lightning),
            seg!("╱          ", ArtColor::RainDrop),
        ]),
        line(vec![
            seg!(" ╱ ╱", ArtColor::RainDrop),
            seg!("ϟ", ArtColor::Lightning),
            seg!("╱ ╱             ", ArtColor::RainDrop),
        ]),
        line(vec![
            seg!("  ", ArtColor::RainDrop),
            seg!("ϟ", ArtColor::Lightning),
            seg!("╱ ╱", ArtColor::RainDrop),
            seg!("ϟ", ArtColor::Lightning),
            seg!("╱          ", ArtColor::RainDrop),
        ]),
        empty(),
    ]
}

// ── Light Snow ───────────────────────────────────────────────────────────────
//   ❄ (U+2744 SNOWFLAKE) — Dingbats block, not emoji
pub fn light_snow() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudLight)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudLight)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudLight)]),
        line(vec![seg!("  ❄  ❄  ❄             ", ArtColor::SnowFlake)]),
        line(vec![seg!("    ❄  ❄  ❄           ", ArtColor::SnowFlake)]),
        line(vec![seg!("  ❄  ❄  ❄             ", ArtColor::SnowFlake)]),
        empty(),
    ]
}

// ── Heavy Snow ───────────────────────────────────────────────────────────────
pub fn heavy_snow() -> ArtPiece {
    vec![
        empty(),
        line(vec![seg!("      .--.            ", ArtColor::CloudDark)]),
        line(vec![seg!("   .-(    ).          ", ArtColor::CloudDark)]),
        line(vec![seg!("  (___.__)__)         ", ArtColor::CloudDark)]),
        line(vec![seg!(" ❄ ❄ ❄ ❄ ❄ ❄          ", ArtColor::SnowFlake)]),
        line(vec![seg!("  ❄ ❄ ❄ ❄ ❄           ", ArtColor::SnowFlake)]),
        line(vec![seg!(" ❄ ❄ ❄ ❄ ❄ ❄          ", ArtColor::SnowFlake)]),
        line(vec![seg!("  ❄ ❄ ❄ ❄ ❄           ", ArtColor::SnowFlake)]),
    ]
}
