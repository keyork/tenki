use crate::model::{CurrentWeather, DailyForecast, HourlyPoint, Location, WeatherData};
use serde::Deserialize;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

// ── Raw API response shapes ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct ApiResponse {
    current: ApiCurrent,
    hourly: ApiHourly,
    daily: ApiDaily,
}

#[derive(Deserialize)]
struct ApiCurrent {
    temperature_2m: f64,
    apparent_temperature: f64,
    relative_humidity_2m: f64,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    precipitation: f64,
    weather_code: u16,
    is_day: u8,
}

#[derive(Deserialize)]
struct ApiHourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    precipitation_probability: Vec<Option<f64>>,
    weather_code: Vec<u16>,
}

#[derive(Deserialize)]
struct ApiDaily {
    time: Vec<String>,
    weather_code: Vec<u16>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    precipitation_sum: Vec<Option<f64>>,
}

// ── Public fetch function ─────────────────────────────────────────────────────

/// Fetch weather data for a location from Open-Meteo.
pub fn fetch(location: Location) -> Result<WeatherData, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={lat}&longitude={lon}\
         &current=temperature_2m,relative_humidity_2m,apparent_temperature,\
         precipitation,weather_code,wind_speed_10m,wind_direction_10m,is_day\
         &hourly=temperature_2m,precipitation_probability,weather_code\
         &daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum\
         &timezone=auto\
         &forecast_days=3",
        lat = location.latitude,
        lon = location.longitude,
    );

    let resp: ApiResponse = ureq::get(&url)
        .timeout(TIMEOUT)
        .call()
        .map_err(|e| format!("Weather request failed: {e}"))?
        .into_json()
        .map_err(|e| format!("Weather parse failed: {e}"))?;

    let current = CurrentWeather {
        temperature: resp.current.temperature_2m,
        feels_like: resp.current.apparent_temperature,
        humidity: resp.current.relative_humidity_2m,
        wind_speed: resp.current.wind_speed_10m,
        wind_direction: resp.current.wind_direction_10m,
        precipitation: resp.current.precipitation,
        weather_code: resp.current.weather_code,
        is_day: resp.current.is_day != 0,
    };

    // Take at most 24 hourly points
    let hourly: Vec<HourlyPoint> = resp
        .hourly
        .time
        .iter()
        .zip(resp.hourly.temperature_2m.iter())
        .zip(resp.hourly.precipitation_probability.iter())
        .zip(resp.hourly.weather_code.iter())
        .take(24)
        .map(|(((time, &temp), prob), &code)| {
            let hour = parse_hour(time);
            HourlyPoint {
                hour,
                temperature: temp,
                precip_probability: prob.unwrap_or(0.0),
                weather_code: code,
            }
        })
        .collect();

    let daily: Vec<DailyForecast> = resp
        .daily
        .time
        .iter()
        .zip(resp.daily.weather_code.iter())
        .zip(resp.daily.temperature_2m_max.iter())
        .zip(resp.daily.temperature_2m_min.iter())
        .zip(resp.daily.precipitation_sum.iter())
        .map(|((((date, &code), &max), &min), precip)| DailyForecast {
            date: date.clone(),
            weather_code: code,
            temp_max: max,
            temp_min: min,
            precip_sum: precip.unwrap_or(0.0),
        })
        .collect();

    Ok(WeatherData {
        location,
        current,
        hourly,
        daily,
    })
}

fn parse_hour(time: &str) -> u8 {
    // time is like "2024-01-15T14:00"
    time.split('T')
        .nth(1)
        .and_then(|t| t.split(':').next())
        .and_then(|h| h.parse().ok())
        .unwrap_or(0)
}
