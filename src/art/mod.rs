pub mod colors;
pub mod pieces;

use crate::model::WeatherCondition;
use pieces::ArtPiece;

/// Return the ASCII art piece for a given weather condition and time-of-day.
pub fn get_art(condition: WeatherCondition, is_day: bool) -> ArtPiece {
    use WeatherCondition::*;
    match (condition, is_day) {
        (ClearSky, true) => pieces::clear_sky_day(),
        (ClearSky, false) => pieces::clear_sky_night(),
        (PartlyCloudy, true) => pieces::partly_cloudy_day(),
        (PartlyCloudy, false) => pieces::partly_cloudy_night(),
        (Overcast, _) => pieces::overcast(),
        (Fog, _) => pieces::fog(),
        (LightDrizzle, _) => pieces::light_drizzle(),
        (LightRain, _) => pieces::light_rain(),
        (HeavyRain, _) => pieces::heavy_rain(),
        (Thunderstorm, _) => pieces::thunderstorm(),
        (LightSnow, _) => pieces::light_snow(),
        (HeavySnow, _) => pieces::heavy_snow(),
    }
}
