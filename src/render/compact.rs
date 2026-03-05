use super::{location_label, scene, truncate_visible, visible_len, RenderContext};
use crate::{
    art,
    model::{feels_desc, wind_direction_label, WeatherCondition},
    units::{fmt_precip, fmt_temp, fmt_wind},
};
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use std::io;

const SCENE_WIDTH: usize = 26;
const GAP: &str = "  │  ";
const INFO_WIDTH: usize = 44;

pub fn render(ctx: &RenderContext) -> io::Result<()> {
    let out = &mut io::stdout();
    let d = ctx.data;
    let condition = WeatherCondition::from_code(d.current.weather_code);
    let art = art::get_art(condition, d.current.is_day);
    let height = art.len();
    let bg = scene::generate(
        condition,
        d.current.is_day,
        SCENE_WIDTH,
        height,
        compact_seed(ctx),
        ctx.theme,
    );
    let range = d
        .daily
        .first()
        .map(|day| {
            format!(
                "High {}  Low {}",
                fmt_temp(day.temp_max, ctx.units),
                fmt_temp(day.temp_min, ctx.units)
            )
        })
        .unwrap_or_default();
    let units = if matches!(ctx.units, crate::units::Units::Metric) {
        "metric"
    } else {
        "imperial"
    };

    let info_rows = [
        location_label(d),
        format!("{} {}", condition.icon(), condition.description()),
        format!(
            "{}  feels {} ({})",
            fmt_temp(d.current.temperature, ctx.units),
            fmt_temp(d.current.feels_like, ctx.units),
            feels_desc(d.current.feels_like),
        ),
        format!(
            "Wind {} {}  Hum {:.0}%",
            fmt_wind(d.current.wind_speed, ctx.units),
            wind_direction_label(d.current.wind_direction),
            d.current.humidity,
        ),
        format!("Rain {}", fmt_precip(d.current.precipitation, ctx.units)),
        range,
        truncate_visible(&format!("Timezone {}", d.location.timezone), INFO_WIDTH),
        format!("Units {}", units),
    ];

    let art_w = crate::art::pieces::ART_WIDTH;
    let art_col = (SCENE_WIDTH.saturating_sub(art_w)) / 2;

    for (row, info_row) in info_rows.iter().enumerate().take(height) {
        let bg_text = bg.lines.get(row).map(|s| s.as_str()).unwrap_or("");
        let left_bg: String = bg_text.chars().take(art_col).collect();
        let right_bg: String = bg_text.chars().skip(art_col + art_w).collect();

        crossterm::execute!(
            out,
            SetForegroundColor(bg.color),
            Print(&left_bg),
            ResetColor,
        )?;

        if let Some(art_line) = art.get(row) {
            for seg in art_line {
                crossterm::execute!(
                    out,
                    SetForegroundColor(ctx.theme.art_color(seg.color)),
                    Print(&seg.text),
                    ResetColor,
                )?;
            }
        } else {
            crossterm::execute!(out, Print(" ".repeat(art_w)))?;
        }

        crossterm::execute!(
            out,
            SetForegroundColor(bg.color),
            Print(&right_bg),
            ResetColor,
            SetForegroundColor(ctx.theme.border_color()),
            Print(GAP),
            ResetColor,
        )?;

        let info = truncate_visible(info_row, INFO_WIDTH);
        let color = match row {
            0 => ctx.theme.title_color(),
            1..=5 => ctx.theme.info_color(),
            _ => ctx.theme.dim_color(),
        };
        crossterm::execute!(out, SetForegroundColor(color), Print(&info), ResetColor,)?;

        let padding = INFO_WIDTH.saturating_sub(visible_len(&info));
        if padding > 0 {
            crossterm::execute!(out, Print(" ".repeat(padding)))?;
        }
        crossterm::execute!(out, Print("\n"))?;
    }

    Ok(())
}

fn compact_seed(ctx: &RenderContext) -> u64 {
    let d = ctx.data;
    d.location.latitude.to_bits()
        ^ d.location.longitude.to_bits().rotate_left(11)
        ^ d.current.weather_code as u64
        ^ d.current.wind_speed.to_bits().rotate_left(23)
}
