use super::{
    chart, location_label, pad_right, scene, truncate_visible, visible_len, RenderContext,
};
use crate::{
    art,
    model::{feels_desc, wind_arrow, wind_direction_label, WeatherCondition},
    units::{fmt_precip, fmt_temp, fmt_wind},
};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::{self, Write};

const CARD_WIDTH: usize = 78;
const SCENE_WIDTH: usize = 26;
const INFO_WIDTH: usize = 44;
const MAIN_HEIGHT: usize = 9;

pub fn render(ctx: &RenderContext) -> io::Result<()> {
    let out = &mut io::stdout();
    let d = ctx.data;
    let condition = WeatherCondition::from_code(d.current.weather_code);
    let art = art::get_art(condition, d.current.is_day);
    let bg = scene::generate(
        condition,
        d.current.is_day,
        SCENE_WIDTH,
        MAIN_HEIGHT,
        scene_seed(ctx),
        ctx.theme,
    );

    print_top_border(out, ctx)?;
    print_header(out, ctx, condition)?;
    print_rule(out, ctx, "Current Conditions")?;
    print_main_block(out, ctx, &art, &bg, condition)?;

    if ctx.show_chart && !ctx.data.hourly.is_empty() {
        print_rule(out, ctx, "24h Temperature")?;
        print_chart(out, ctx)?;
    }

    if ctx.show_forecast && d.daily.len() > 1 {
        print_rule(out, ctx, "Next 2 Days")?;
        print_forecast(out, ctx)?;
    }

    print_bottom_border(out, ctx)?;
    Ok(())
}

fn print_top_border<W: Write>(out: &mut W, ctx: &RenderContext) -> io::Result<()> {
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.border_color()),
        Print(format!("╭{}╮\n", "─".repeat(CARD_WIDTH - 2))),
        ResetColor,
    )
}

fn print_bottom_border<W: Write>(out: &mut W, ctx: &RenderContext) -> io::Result<()> {
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.border_color()),
        Print(format!("╰{}╯\n", "─".repeat(CARD_WIDTH - 2))),
        ResetColor,
    )
}

fn print_rule<W: Write>(out: &mut W, ctx: &RenderContext, title: &str) -> io::Result<()> {
    let title = format!(" {} ", title);
    let filler = CARD_WIDTH.saturating_sub(2 + visible_len(&title));
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.border_color()),
        Print("├"),
        Print(&title),
        Print("─".repeat(filler)),
        Print("┤\n"),
        ResetColor,
    )
}

fn print_header<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    condition: WeatherCondition,
) -> io::Result<()> {
    let d = ctx.data;
    let content_w = CARD_WIDTH - 4;
    let right = format!("{} {}", condition.icon(), condition.description());
    let right_w = visible_len(&right);
    let left = truncate_visible(&location_label(d), content_w.saturating_sub(right_w + 2));
    let gap = content_w.saturating_sub(visible_len(&left) + right_w);

    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.border_color()),
        Print("│ "),
        ResetColor,
        SetForegroundColor(ctx.theme.title_color()),
        Print(&left),
        ResetColor,
        Print(" ".repeat(gap)),
        SetForegroundColor(ctx.theme.info_color()),
        Print(&right),
        ResetColor,
        SetForegroundColor(ctx.theme.border_color()),
        Print(" │\n"),
        ResetColor,
    )
}

fn print_main_block<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    art: &[Vec<crate::art::pieces::ArtSegment>],
    bg: &scene::Scene,
    condition: WeatherCondition,
) -> io::Result<()> {
    let d = ctx.data;
    let temp_color = if d.current.temperature < 0.0 {
        ctx.theme.cold_color()
    } else if d.current.temperature > 30.0 {
        ctx.theme.highlight_color()
    } else {
        ctx.theme.temp_color()
    };

    let now = fmt_temp(d.current.temperature, ctx.units);
    let feels = format!(
        "{} ({})",
        fmt_temp(d.current.feels_like, ctx.units),
        feels_desc(d.current.feels_like),
    );
    let range = d
        .daily
        .first()
        .map(|day| {
            format!(
                "↑{}  ↓{}",
                fmt_temp(day.temp_max, ctx.units),
                fmt_temp(day.temp_min, ctx.units)
            )
        })
        .unwrap_or_else(|| "-".to_string());
    let wind = format!(
        "{}  {} {}",
        fmt_wind(d.current.wind_speed, ctx.units),
        wind_direction_label(d.current.wind_direction),
        wind_arrow(d.current.wind_direction),
    );
    let filled = ((d.current.humidity / 100.0) * 8.0).round() as usize;
    let bar = format!(
        "{}{}",
        "█".repeat(filled.min(8)),
        "░".repeat(8usize.saturating_sub(filled.min(8)))
    );
    let humidity = format!("[{}] {:.0}%", bar, d.current.humidity);
    let precip = fmt_precip(d.current.precipitation, ctx.units);
    let units = if matches!(ctx.units, crate::units::Units::Metric) {
        "metric"
    } else {
        "imperial"
    };
    let coords = format!("{:.2}, {:.2}", d.location.latitude, d.location.longitude);

    let info_rows = [
        metric(
            "Condition",
            condition.description(),
            INFO_WIDTH,
            ctx.theme.info_color(),
        ),
        metric("Now", &now, INFO_WIDTH, temp_color),
        metric("Feels", &feels, INFO_WIDTH, ctx.theme.info_color()),
        metric("Range", &range, INFO_WIDTH, ctx.theme.dim_color()),
        metric("Wind", &wind, INFO_WIDTH, ctx.theme.info_color()),
        metric("Humidity", &humidity, INFO_WIDTH, ctx.theme.info_color()),
        metric("Precip", &precip, INFO_WIDTH, ctx.theme.info_color()),
        metric("Units", units, INFO_WIDTH, ctx.theme.dim_color()),
        metric("Coords", &coords, INFO_WIDTH, ctx.theme.dim_color()),
    ];

    let art_w = crate::art::pieces::ART_WIDTH;
    let art_col = (SCENE_WIDTH.saturating_sub(art_w)) / 2;
    let art_row_start = (MAIN_HEIGHT.saturating_sub(art.len())) / 2;

    for row in 0..MAIN_HEIGHT {
        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.border_color()),
            Print("│ "),
            ResetColor,
        )?;

        let bg_text = bg.lines.get(row).map(|s| s.as_str()).unwrap_or("");
        let left_bg: String = bg_text.chars().take(art_col).collect();
        let right_bg: String = bg_text.chars().skip(art_col + art_w).collect();

        crossterm::execute!(
            out,
            SetForegroundColor(bg.color),
            Print(&left_bg),
            ResetColor,
        )?;

        if (art_row_start..art_row_start + art.len()).contains(&row) {
            for seg in &art[row - art_row_start] {
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
            Print(" │ "),
            ResetColor,
        )?;

        let (label, value, value_color) = &info_rows[row];
        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.dim_color()),
            Print(label),
            ResetColor,
            Print(" "),
            SetForegroundColor(*value_color),
            Print(value),
            ResetColor,
        )?;

        let used = visible_len(label) + 1 + visible_len(value);
        if INFO_WIDTH > used {
            crossterm::execute!(out, Print(" ".repeat(INFO_WIDTH - used)))?;
        }

        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.border_color()),
            Print(" │\n"),
            ResetColor,
        )?;
    }

    Ok(())
}

fn print_chart<W: Write>(out: &mut W, ctx: &RenderContext) -> io::Result<()> {
    let mut buf = Vec::new();
    chart::render_chart_wide(&mut buf, &ctx.data.hourly, ctx.theme, CARD_WIDTH - 6)
        .map_err(io::Error::other)?;
    let chart_str = String::from_utf8_lossy(&buf);

    for line in chart_str.lines() {
        let line = pad_right(line, CARD_WIDTH - 4);
        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.border_color()),
            Print("│ "),
            ResetColor,
            Print(&line),
            SetForegroundColor(ctx.theme.border_color()),
            Print(" │\n"),
            ResetColor,
        )?;
    }

    Ok(())
}

fn print_forecast<W: Write>(out: &mut W, ctx: &RenderContext) -> io::Result<()> {
    for day in ctx.data.daily.iter().skip(1).take(2) {
        let condition = WeatherCondition::from_code(day.weather_code);
        let left = pad_right(&day.date, 12);
        let center = pad_right(
            &format!("{} {}", condition.icon(), condition.description()),
            18,
        );
        let right = truncate_visible(
            &format!(
                "↑{}  ↓{}  ☂ {}",
                fmt_temp(day.temp_max, ctx.units),
                fmt_temp(day.temp_min, ctx.units),
                fmt_precip(day.precip_sum, ctx.units)
            ),
            CARD_WIDTH - 4 - 12 - 2 - 18 - 2,
        );

        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.border_color()),
            Print("│ "),
            ResetColor,
            SetForegroundColor(ctx.theme.dim_color()),
            Print(&left),
            ResetColor,
            Print("  "),
            SetForegroundColor(ctx.theme.info_color()),
            Print(&center),
            ResetColor,
            Print("  "),
            SetForegroundColor(ctx.theme.temp_color()),
            Print(&right),
            ResetColor,
        )?;

        let used = 12 + 2 + 18 + 2 + visible_len(&right);
        if CARD_WIDTH - 4 > used {
            crossterm::execute!(out, Print(" ".repeat(CARD_WIDTH - 4 - used)))?;
        }

        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.border_color()),
            Print(" │\n"),
            ResetColor,
        )?;
    }

    Ok(())
}

fn metric(label: &str, value: &str, total_w: usize, color: Color) -> (String, String, Color) {
    let label = pad_right(label, 9);
    let value = truncate_visible(value, total_w.saturating_sub(10));
    (label, value, color)
}

fn scene_seed(ctx: &RenderContext) -> u64 {
    let d = ctx.data;
    (d.current.weather_code as u64) << 48
        ^ d.location.latitude.to_bits()
        ^ d.location.longitude.to_bits().rotate_left(9)
        ^ d.current.temperature.to_bits().rotate_left(17)
        ^ d.current.humidity.to_bits().rotate_left(29)
}
