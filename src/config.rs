use serde::Deserialize;
use std::path::PathBuf;

/// Top-level configuration file structure.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default)]
    pub display: DisplayConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct LocationConfig {
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_units")]
    pub units: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub show_chart: bool,
}

fn default_units() -> String {
    "metric".into()
}
fn default_mode() -> String {
    "card".into()
}
fn default_theme() -> String {
    "default".into()
}
fn default_true() -> bool {
    true
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            units: default_units(),
            mode: default_mode(),
            theme: default_theme(),
            show_chart: default_true(),
        }
    }
}

/// Load configuration from `~/.config/tenki/config.toml` if it exists.
/// Returns default config on any error or if the file does not exist.
pub fn load() -> Config {
    config_path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("tenki").join("config.toml"))
}
