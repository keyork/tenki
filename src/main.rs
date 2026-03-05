mod art;
mod config;
mod location;
mod model;
mod render;
mod theme;
mod units;
mod weather;

use clap::{parser::ValueSource, CommandFactory, FromArgMatches, Parser, ValueEnum};
use render::RenderContext;
use std::{thread, time::Duration};
use units::Units;

/// 有调性的终端天气 CLI — tenki (天気)
#[derive(Parser, Debug)]
#[command(name = "tenki", version, about = "Beautiful terminal weather")]
struct Cli {
    /// City name (omit for auto-detect via IP)
    city: Option<String>,

    /// Output mode
    #[arg(short, long, value_enum, default_value = "card")]
    mode: Mode,

    /// Unit system
    #[arg(short, long, value_enum, default_value = "metric")]
    units: UnitArg,

    /// Color theme
    #[arg(short, long, default_value = "default")]
    theme: String,

    /// Show 3-day forecast (card/fullscreen modes)
    #[arg(short, long)]
    forecast: bool,

    /// Hide the 24h temperature mini-chart
    #[arg(long)]
    no_chart: bool,

    /// Disable fullscreen/showcase animation (static scene)
    #[arg(long = "static")]
    static_view: bool,

    /// Latitude (overrides city / auto-detect)
    #[arg(long)]
    lat: Option<f64>,

    /// Longitude (overrides city / auto-detect)
    #[arg(long)]
    lon: Option<f64>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Card,
    Compact,
    Fullscreen,
    Showcase,
    Oneline,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum UnitArg {
    Metric,
    Imperial,
}

const WEATHER_MAX_ATTEMPTS: usize = 5;
const WEATHER_RETRY_DELAY_MS: u64 = 500;

fn main() {
    let matches = Cli::command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    let cfg = config::load();

    // Resolve units (CLI > config > default)
    let units = match cli.units {
        UnitArg::Imperial => Units::Imperial,
        UnitArg::Metric => Units::Metric,
    };
    let units = if arg_from_cli(&matches, "units") {
        units
    } else {
        Units::from_str(&cfg.display.units)
    };

    // Resolve mode (CLI > config)
    let mode = if arg_from_cli(&matches, "mode") {
        cli.mode
    } else {
        match cfg.display.mode.as_str() {
            "compact" => Mode::Compact,
            "fullscreen" => Mode::Fullscreen,
            "showcase" => Mode::Showcase,
            "oneline" => Mode::Oneline,
            _ => cli.mode,
        }
    };

    // Resolve show_chart
    let show_chart = if arg_from_cli(&matches, "no_chart") {
        !cli.no_chart
    } else {
        cfg.display.show_chart
    };

    // Resolve theme
    let theme_name = if arg_from_cli(&matches, "theme") {
        cli.theme.clone()
    } else {
        cfg.display.theme.clone()
    };
    let theme = theme::resolve(&theme_name);

    // Resolve location
    let loc_result = if let (Some(lat), Some(lon)) = (cli.lat, cli.lon) {
        Ok(location::from_coords(lat, lon))
    } else if let Some(city) = cli.city.as_deref() {
        location::from_city(city)
    } else if let (Some(lat), Some(lon)) = (cfg.location.latitude, cfg.location.longitude) {
        Ok(location::from_coords(lat, lon))
    } else if let Some(city) = cfg.location.city.as_deref() {
        location::from_city(city)
    } else {
        location::from_ip()
    };

    let loc = match loc_result {
        Ok(l) => l,
        Err(e) => {
            eprintln!("tenki: location error: {e}");
            std::process::exit(1);
        }
    };

    // Fetch weather
    let data = match fetch_weather_with_retry(&loc) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("tenki: weather error: {e}");
            std::process::exit(1);
        }
    };

    let ctx = RenderContext {
        data: &data,
        theme: theme.as_ref(),
        units,
        show_chart,
        show_forecast: cli.forecast,
        animate: !cli.static_view,
    };

    let result = match mode {
        Mode::Card => render::card::render(&ctx),
        Mode::Compact => render::compact::render(&ctx),
        Mode::Fullscreen => render::fullscreen::render(&ctx),
        Mode::Showcase => render::fullscreen::render_showcase(&ctx),
        Mode::Oneline => render::oneline::render(&ctx),
    };

    if let Err(e) = result {
        eprintln!("tenki: render error: {e}");
        std::process::exit(1);
    }
}

fn arg_from_cli(matches: &clap::ArgMatches, id: &str) -> bool {
    matches.value_source(id) == Some(ValueSource::CommandLine)
}

fn fetch_weather_with_retry(location: &model::Location) -> Result<model::WeatherData, String> {
    let mut last_err = String::new();

    for attempt in 1..=WEATHER_MAX_ATTEMPTS {
        match weather::fetch(location.clone()) {
            Ok(data) => return Ok(data),
            Err(err) => {
                last_err = err;
                if attempt < WEATHER_MAX_ATTEMPTS {
                    thread::sleep(Duration::from_millis(WEATHER_RETRY_DELAY_MS));
                }
            }
        }
    }

    Err(format!(
        "{} (attempted {} times)",
        last_err, WEATHER_MAX_ATTEMPTS
    ))
}
