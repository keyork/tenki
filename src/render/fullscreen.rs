use super::{
    chart, location_label, pad_center, pad_right, scene, truncate_visible, visible_len,
    RenderContext,
};
use crate::{
    art,
    model::{feels_desc, wind_arrow, wind_direction_label, WeatherCondition},
    units::{fmt_precip, fmt_temp, fmt_wind},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

const FIXED_ROWS: usize = 16;
const MIN_SCENE_ROWS: usize = 6;
const SEP: &str = "━";
const SHOWCASE_DURATION: Duration = Duration::from_secs(5);

pub fn render(ctx: &RenderContext) -> io::Result<()> {
    render_with_timeout(ctx, None)
}

pub fn render_showcase(ctx: &RenderContext) -> io::Result<()> {
    render_with_timeout(ctx, Some(SHOWCASE_DURATION))
}

fn render_with_timeout(ctx: &RenderContext, auto_exit_after: Option<Duration>) -> io::Result<()> {
    let out = &mut io::stdout();

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    terminal::enable_raw_mode()?;
    crossterm::execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run_loop(out, ctx, seed, auto_exit_after);

    crossterm::execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    result
}

fn run_loop<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    seed: u64,
    auto_exit_after: Option<Duration>,
) -> io::Result<()> {
    let start = Instant::now();
    draw_frame(out, ctx, seed, auto_exit_after, start)?;

    loop {
        if let Some(limit) = auto_exit_after {
            if start.elapsed() >= limit {
                break;
            }
        }

        if event::poll(std::time::Duration::from_millis(150))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    _ => {}
                },
                Event::Resize(_, _) => draw_frame(out, ctx, seed, auto_exit_after, start)?,
                _ => {}
            }
        }

        if auto_exit_after.is_some() {
            draw_frame(out, ctx, seed, auto_exit_after, start)?;
        }
    }
    Ok(())
}

fn draw_frame<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    seed: u64,
    auto_exit_after: Option<Duration>,
    start: Instant,
) -> io::Result<()> {
    let (width, rows) = terminal_size();

    crossterm::execute!(out, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;

    if width < 52 {
        crossterm::execute!(out, Print("tenki: terminal too narrow (min 52 columns)"))?;
        out.flush()?;
        return Ok(());
    }

    let scene_rows = rows.saturating_sub(FIXED_ROWS).max(MIN_SCENE_ROWS);
    let d = ctx.data;
    let condition = WeatherCondition::from_code(d.current.weather_code);
    let art = art::get_art(condition, d.current.is_day);
    let bg = scene::generate(
        condition,
        d.current.is_day,
        width,
        scene_rows,
        seed,
        ctx.theme,
    );

    print_sep(out, width, ctx)?;
    print_header(out, ctx, width, condition)?;
    print_sep(out, width, ctx)?;
    print_stats(out, ctx, width)?;
    print_sep(out, width, ctx)?;
    print_scene(out, ctx, &art, &bg, width, scene_rows)?;
    print_sep(out, width, ctx)?;

    if ctx.show_chart && !ctx.data.hourly.is_empty() {
        print_chart(out, ctx, width)?;
        print_sep(out, width, ctx)?;
    }

    if ctx.show_forecast && d.daily.len() > 1 {
        print_forecast(out, ctx, width)?;
        print_sep(out, width, ctx)?;
    }

    let hint = match auto_exit_after {
        Some(limit) => {
            let remaining = limit.saturating_sub(start.elapsed()).as_secs().max(1);
            format!(" Auto exit in {remaining}s ")
        }
        None => " Q to quit ".to_string(),
    };
    let hint_col = width.saturating_sub(hint.len()) as u16;
    crossterm::execute!(
        out,
        cursor::MoveTo(hint_col, rows.saturating_sub(1) as u16),
        SetForegroundColor(ctx.theme.dim_color()),
        Print(&hint),
        ResetColor,
    )?;

    out.flush()?;
    Ok(())
}

fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

fn nl<W: Write>(out: &mut W) -> io::Result<()> {
    crossterm::execute!(out, Print("\r\n"))
}

fn print_sep<W: Write>(out: &mut W, width: usize, ctx: &RenderContext) -> io::Result<()> {
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.border_color()),
        Print(SEP.repeat(width)),
        ResetColor,
    )?;
    nl(out)
}

fn print_header<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    width: usize,
    condition: WeatherCondition,
) -> io::Result<()> {
    let d = ctx.data;
    let content_w = width.saturating_sub(4);
    let right = format!("{} {}", condition.icon(), condition.description());
    let right_w = visible_len(&right);
    let left = truncate_visible(&location_label(d), content_w.saturating_sub(right_w + 2));
    let gap = content_w.saturating_sub(visible_len(&left) + right_w);

    crossterm::execute!(
        out,
        Print("  "),
        SetForegroundColor(ctx.theme.title_color()),
        Print(left),
        ResetColor,
        Print(" ".repeat(gap)),
        SetForegroundColor(ctx.theme.info_color()),
        Print(right),
        ResetColor,
        Print("  "),
    )?;
    nl(out)
}

fn print_stats<W: Write>(out: &mut W, ctx: &RenderContext, width: usize) -> io::Result<()> {
    let d = ctx.data;
    let temp_color = if d.current.temperature < 0.0 {
        ctx.theme.cold_color()
    } else if d.current.temperature > 30.0 {
        ctx.theme.highlight_color()
    } else {
        ctx.theme.temp_color()
    };
    let filled = ((d.current.humidity / 100.0) * 8.0).round() as usize;
    let bar = format!(
        "{}{}",
        "█".repeat(filled.min(8)),
        "░".repeat(8usize.saturating_sub(filled.min(8)))
    );
    let (hi, lo) = d
        .daily
        .first()
        .map(|day| {
            (
                fmt_temp(day.temp_max, ctx.units),
                fmt_temp(day.temp_min, ctx.units),
            )
        })
        .unwrap_or_default();

    let mut rows = vec![
        vec![
            stat(
                "Temperature",
                &fmt_temp(d.current.temperature, ctx.units),
                temp_color,
            ),
            stat(
                "Feels",
                &format!(
                    "{} ({})",
                    fmt_temp(d.current.feels_like, ctx.units),
                    feels_desc(d.current.feels_like)
                ),
                ctx.theme.info_color(),
            ),
            stat("Range", &format!("↑{}  ↓{}", hi, lo), ctx.theme.dim_color()),
        ],
        vec![
            stat(
                "Wind",
                &format!(
                    "{}  {} {}",
                    fmt_wind(d.current.wind_speed, ctx.units),
                    wind_direction_label(d.current.wind_direction),
                    wind_arrow(d.current.wind_direction)
                ),
                ctx.theme.info_color(),
            ),
            stat(
                "Humidity",
                &format!("[{}] {:.0}%", bar, d.current.humidity),
                ctx.theme.info_color(),
            ),
            stat(
                "Precip",
                &fmt_precip(d.current.precipitation, ctx.units),
                ctx.theme.info_color(),
            ),
        ],
    ];

    let cols = if width >= 84 { 3 } else { 2 };
    if cols == 2 {
        rows = vec![
            vec![
                stat(
                    "Temperature",
                    &fmt_temp(d.current.temperature, ctx.units),
                    temp_color,
                ),
                stat(
                    "Feels",
                    &format!(
                        "{} ({})",
                        fmt_temp(d.current.feels_like, ctx.units),
                        feels_desc(d.current.feels_like)
                    ),
                    ctx.theme.info_color(),
                ),
            ],
            vec![
                stat(
                    "Wind",
                    &format!(
                        "{}  {} {}",
                        fmt_wind(d.current.wind_speed, ctx.units),
                        wind_direction_label(d.current.wind_direction),
                        wind_arrow(d.current.wind_direction)
                    ),
                    ctx.theme.info_color(),
                ),
                stat(
                    "Humidity",
                    &format!("[{}] {:.0}%", bar, d.current.humidity),
                    ctx.theme.info_color(),
                ),
            ],
            vec![
                stat("Range", &format!("↑{}  ↓{}", hi, lo), ctx.theme.dim_color()),
                stat(
                    "Precip",
                    &fmt_precip(d.current.precipitation, ctx.units),
                    ctx.theme.info_color(),
                ),
            ],
        ];
    }

    for row in rows {
        print_stat_row(out, ctx, width, &row)?;
    }
    Ok(())
}

fn print_stat_row<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    width: usize,
    row: &[(&str, String, Color)],
) -> io::Result<()> {
    let content_w = width.saturating_sub(4);
    let base = content_w / row.len();
    let extra = content_w % row.len();

    crossterm::execute!(out, Print("  "))?;
    for (idx, (label, value, color)) in row.iter().enumerate() {
        let cell_w = base + usize::from(idx < extra);
        let label_w = visible_len(label);
        let value = truncate_visible(value, cell_w.saturating_sub(label_w + 2));

        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.dim_color()),
            Print(label),
            ResetColor,
            Print(": "),
            SetForegroundColor(*color),
            Print(&value),
            ResetColor,
        )?;

        let used = label_w + 2 + visible_len(&value);
        if cell_w > used {
            crossterm::execute!(out, Print(" ".repeat(cell_w - used)))?;
        }
    }
    nl(out)
}

fn print_scene<W: Write>(
    out: &mut W,
    ctx: &RenderContext,
    art: &[Vec<crate::art::pieces::ArtSegment>],
    bg: &scene::Scene,
    width: usize,
    scene_height: usize,
) -> io::Result<()> {
    let art_w = crate::art::pieces::ART_WIDTH;
    let art_col = width.saturating_sub(art_w) / 2;
    let art_row_start = scene_height.saturating_sub(art.len()) / 2;
    let art_row_end = art_row_start + art.len();

    for row in 0..scene_height {
        let bg_text = bg.lines.get(row).map(|s| s.as_str()).unwrap_or("");
        let bg_color = bg.color;

        if row < art_row_start || row >= art_row_end {
            crossterm::execute!(
                out,
                SetForegroundColor(bg_color),
                Print(bg_text),
                ResetColor
            )?;
        } else {
            let art_idx = row - art_row_start;
            let left: String = bg_text.chars().take(art_col).collect();
            let right: String = bg_text.chars().skip(art_col + art_w).collect();

            crossterm::execute!(out, SetForegroundColor(bg_color), Print(left), ResetColor)?;
            for seg in &art[art_idx] {
                crossterm::execute!(
                    out,
                    SetForegroundColor(ctx.theme.art_color(seg.color)),
                    Print(&seg.text),
                    ResetColor,
                )?;
            }
            crossterm::execute!(out, SetForegroundColor(bg_color), Print(right), ResetColor)?;
        }
        nl(out)?;
    }
    Ok(())
}

fn print_chart<W: Write>(out: &mut W, ctx: &RenderContext, width: usize) -> io::Result<()> {
    let title = pad_right("  24h temperature trend", width);
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.dim_color()),
        Print(title),
        ResetColor,
    )?;
    nl(out)?;

    let mut buf = Vec::new();
    chart::render_chart_wide(&mut buf, &ctx.data.hourly, ctx.theme, width)
        .map_err(io::Error::other)?;
    let chart_str = String::from_utf8_lossy(&buf);
    for line in chart_str.lines() {
        let padded = pad_right(line, width);
        crossterm::execute!(out, Print(&padded))?;
        nl(out)?;
    }
    Ok(())
}

fn print_forecast<W: Write>(out: &mut W, ctx: &RenderContext, width: usize) -> io::Result<()> {
    let title = pad_right("  next days", width);
    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.dim_color()),
        Print(title),
        ResetColor,
    )?;
    nl(out)?;

    let days: Vec<_> = ctx.data.daily.iter().skip(1).take(2).collect();
    let cell_w = (width.saturating_sub(2)) / days.len().max(1);

    crossterm::execute!(out, Print(" "))?;
    for day in days {
        let cond = WeatherCondition::from_code(day.weather_code);
        let line = format!(
            "{}  {}  ↑{} ↓{}  ☂ {}",
            day.date,
            cond.icon(),
            fmt_temp(day.temp_max, ctx.units),
            fmt_temp(day.temp_min, ctx.units),
            fmt_precip(day.precip_sum, ctx.units),
        );
        let cell = pad_center(&truncate_visible(&line, cell_w), cell_w);
        crossterm::execute!(
            out,
            SetForegroundColor(ctx.theme.info_color()),
            Print(&cell),
            ResetColor,
        )?;
    }
    crossterm::execute!(out, Print(" "))?;
    nl(out)
}

fn stat<'a>(label: &'a str, value: &str, color: Color) -> (&'a str, String, Color) {
    (label, value.to_string(), color)
}
