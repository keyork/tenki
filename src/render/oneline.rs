use super::RenderContext;
use crate::{
    model::{wind_direction_label, WeatherCondition},
    units::{fmt_temp, fmt_wind},
};
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use std::io::{self};

pub fn render(ctx: &RenderContext) -> io::Result<()> {
    let out = &mut io::stdout();
    let d = ctx.data;
    let condition = WeatherCondition::from_code(d.current.weather_code);

    crossterm::execute!(
        out,
        SetForegroundColor(ctx.theme.title_color()),
        Print(d.location.name.as_str()),
        ResetColor,
        Print(": "),
        SetForegroundColor(ctx.theme.info_color()),
        Print(format!(
            "{} {} 💨{} 💧{}%",
            condition.icon(),
            fmt_temp(d.current.temperature, ctx.units),
            fmt_wind(d.current.wind_speed, ctx.units),
            d.current.humidity as u64,
        )),
        ResetColor,
    )?;
    println!();
    let _ = wind_direction_label(d.current.wind_direction); // available if needed

    Ok(())
}
