/// Unit system selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Metric,
    Imperial,
}

impl Units {
    pub fn from_str(s: &str) -> Self {
        if s == "imperial" {
            Self::Imperial
        } else {
            Self::Metric
        }
    }
}

/// Format temperature with unit suffix.
pub fn fmt_temp(celsius: f64, units: Units) -> String {
    match units {
        Units::Metric => format!("{:.0}°C", celsius),
        Units::Imperial => format!("{:.0}°F", celsius * 9.0 / 5.0 + 32.0),
    }
}

/// Format wind speed with unit suffix.
pub fn fmt_wind(kmh: f64, units: Units) -> String {
    match units {
        Units::Metric => format!("{:.0} km/h", kmh),
        Units::Imperial => format!("{:.0} mph", kmh * 0.621_371),
    }
}

/// Format precipitation with unit suffix.
pub fn fmt_precip(mm: f64, units: Units) -> String {
    match units {
        Units::Metric => format!("{:.1} mm", mm),
        Units::Imperial => format!("{:.2} in", mm * 0.039_370_1),
    }
}
