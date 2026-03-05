#![allow(dead_code)]
//! All core data structures for tenki.
/// A geographic location.
#[derive(Debug, Clone)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

/// Current weather conditions.
#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub precipitation: f64,
    pub weather_code: u16,
    pub is_day: bool,
}

/// A single hourly data point.
#[derive(Debug, Clone)]
pub struct HourlyPoint {
    pub hour: u8,
    pub temperature: f64,
    pub precip_probability: f64,
    pub weather_code: u16,
}

/// A daily forecast summary.
#[derive(Debug, Clone)]
pub struct DailyForecast {
    pub date: String,
    pub weather_code: u16,
    pub temp_max: f64,
    pub temp_min: f64,
    pub precip_sum: f64,
}

/// Full weather response for a location.
#[derive(Debug, Clone)]
pub struct WeatherData {
    pub location: Location,
    pub current: CurrentWeather,
    pub hourly: Vec<HourlyPoint>,
    pub daily: Vec<DailyForecast>,
}

/// WMO weather code → internal condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherCondition {
    ClearSky,
    PartlyCloudy,
    Overcast,
    Fog,
    LightDrizzle,
    LightRain,
    HeavyRain,
    Thunderstorm,
    LightSnow,
    HeavySnow,
}

impl WeatherCondition {
    /// Map a WMO weather code to a `WeatherCondition`.
    pub fn from_code(code: u16) -> Self {
        match code {
            0 => Self::ClearSky,
            1 | 2 => Self::PartlyCloudy,
            3 => Self::Overcast,
            45 | 48 => Self::Fog,
            51 | 53 | 56 => Self::LightDrizzle,
            61 | 63 | 66 | 80 | 81 => Self::LightRain,
            55 | 57 | 65 | 67 | 82 => Self::HeavyRain,
            71 | 73 | 77 | 85 => Self::LightSnow,
            75 | 86 => Self::HeavySnow,
            95 | 96 | 99 => Self::Thunderstorm,
            _ => Self::Overcast,
        }
    }

    /// Human-readable description.
    pub fn description(self) -> &'static str {
        match self {
            Self::ClearSky => "Clear Sky",
            Self::PartlyCloudy => "Partly Cloudy",
            Self::Overcast => "Overcast",
            Self::Fog => "Fog",
            Self::LightDrizzle => "Light Drizzle",
            Self::LightRain => "Light Rain",
            Self::HeavyRain => "Heavy Rain",
            Self::Thunderstorm => "Thunderstorm",
            Self::LightSnow => "Light Snow",
            Self::HeavySnow => "Heavy Snow",
        }
    }

    /// Emoji icon for the condition (used in info panels, not ASCII art).
    pub fn icon(self) -> &'static str {
        match self {
            Self::ClearSky => "☀",
            Self::PartlyCloudy => "⛅",
            Self::Overcast => "☁",
            Self::Fog => "🌫",
            Self::LightDrizzle => "🌦",
            Self::LightRain => "🌧",
            Self::HeavyRain => "🌧",
            Self::Thunderstorm => "⛈",
            Self::LightSnow => "🌨",
            Self::HeavySnow => "❄",
        }
    }
}

/// Cardinal wind direction from degrees.
pub fn wind_direction_label(degrees: f64) -> &'static str {
    let idx = ((degrees + 22.5) / 45.0) as usize % 8;
    ["N", "NE", "E", "SE", "S", "SW", "W", "NW"][idx]
}

/// Arrow showing where the wind is blowing (opposite of "from" direction).
pub fn wind_arrow(degrees: f64) -> &'static str {
    let idx = ((degrees + 22.5) / 45.0) as usize % 8;
    ["↓", "↙", "←", "↖", "↑", "↗", "→", "↘"][idx]
}

/// Short descriptor for feels-like temperature.
pub fn feels_desc(celsius: f64) -> &'static str {
    if celsius < -15.0 {
        "brutal"
    } else if celsius < 0.0 {
        "freezing"
    } else if celsius < 8.0 {
        "cold"
    } else if celsius < 15.0 {
        "cool"
    } else if celsius < 22.0 {
        "comfortable"
    } else if celsius < 28.0 {
        "warm"
    } else {
        "hot"
    }
}
