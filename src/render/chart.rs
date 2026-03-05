use super::{pad_center, visible_len};
use crate::{model::HourlyPoint, theme::Theme};
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use std::io::Write;

const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Render a wide 24h chart spanning `width` columns (for fullscreen mode).
pub fn render_chart_wide<W: Write>(
    out: &mut W,
    hourly: &[HourlyPoint],
    theme: &dyn Theme,
    width: usize,
) -> std::io::Result<()> {
    render_chart_with_width(out, hourly, theme, width)
}

fn render_chart_with_width<W: Write>(
    out: &mut W,
    hourly: &[HourlyPoint],
    theme: &dyn Theme,
    width: usize,
) -> std::io::Result<()> {
    let samples: Vec<&HourlyPoint> = hourly.iter().step_by(3).take(8).collect();
    if samples.is_empty() {
        return Ok(());
    }

    let temps: Vec<f64> = samples.iter().map(|h| h.temperature).collect();
    let min = temps.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);
    let n = samples.len();
    let width = width.max(n * 4);
    let base = width / n;
    let extra = width % n;

    for row in 0..3 {
        for (i, sample) in samples.iter().enumerate() {
            let cell_w = base + usize::from(i < extra);
            let content = match row {
                0 => format!("{:02}", sample.hour),
                1 => format!("{:>3}°", sample.temperature.round() as i64),
                _ => {
                    let normalized = (sample.temperature - min) / range;
                    let block_idx = ((normalized * 7.0) as usize).min(7);
                    format!("{}{}", BLOCKS[block_idx], BLOCKS[block_idx])
                }
            };
            let padded = pad_center(&content, cell_w);
            let color = match row {
                0 => theme.dim_color(),
                1 => theme.temp_color(),
                _ => {
                    let normalized = (sample.temperature - min) / range;
                    theme.chart_color(normalized)
                }
            };

            crossterm::execute!(out, SetForegroundColor(color), Print(&padded), ResetColor,)?;
        }
        crossterm::execute!(out, Print("\n"))?;
    }

    Ok(())
}

#[allow(dead_code)]
/// Render a 24h temperature mini-chart to stdout.
pub fn render_chart<W: Write>(
    out: &mut W,
    hourly: &[HourlyPoint],
    theme: &dyn Theme,
) -> std::io::Result<()> {
    let samples: Vec<&HourlyPoint> = hourly.iter().step_by(3).take(8).collect();
    if samples.is_empty() {
        return Ok(());
    }

    let min_width = samples
        .iter()
        .map(|sample| visible_len(&format!("{:>3}°", sample.temperature.round() as i64)).max(4))
        .sum::<usize>();
    render_chart_with_width(out, hourly, theme, min_width)
}
