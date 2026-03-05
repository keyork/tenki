use crate::model::Location;
use serde::Deserialize;

// ── IP auto-detect ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct IpApiResponse {
    status: String,
    city: Option<String>,
    country: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
}

/// Detect current location via IP geolocation (ip-api.com, free, no key).
pub fn from_ip() -> Result<Location, String> {
    let resp: IpApiResponse =
        ureq::get("http://ip-api.com/json/?fields=status,city,country,lat,lon")
            .timeout(std::time::Duration::from_secs(5))
            .call()
            .map_err(|e| format!("IP geolocation request failed: {e}"))?
            .into_json()
            .map_err(|e| format!("IP geolocation parse failed: {e}"))?;

    if resp.status != "success" {
        return Err("IP geolocation returned non-success status".into());
    }

    Ok(Location {
        name: resp.city.unwrap_or_else(|| "Unknown".into()),
        country: resp.country.unwrap_or_default(),
        latitude: resp.lat.ok_or("Missing lat")?,
        longitude: resp.lon.ok_or("Missing lon")?,
        timezone: "auto".into(),
    })
}

// ── Geocoding (city name → coordinates) ──────────────────────────────────────

#[derive(Deserialize)]
struct GeoResponse {
    results: Option<Vec<GeoResult>>,
}

#[derive(Deserialize)]
struct GeoResult {
    name: String,
    latitude: f64,
    longitude: f64,
    country: Option<String>,
    timezone: Option<String>,
}

/// Resolve a city name to a `Location` via Open-Meteo Geocoding API.
pub fn from_city(city: &str) -> Result<Location, String> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=zh&format=json",
        urlencoded(city),
    );

    let resp: GeoResponse = ureq::get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .call()
        .map_err(|e| format!("Geocoding request failed: {e}"))?
        .into_json()
        .map_err(|e| format!("Geocoding parse failed: {e}"))?;

    let result = resp
        .results
        .and_then(|mut v| {
            if v.is_empty() {
                None
            } else {
                Some(v.remove(0))
            }
        })
        .ok_or_else(|| format!("City not found: '{city}'"))?;

    Ok(Location {
        name: result.name,
        country: normalize_country(result.country.as_deref()),
        latitude: result.latitude,
        longitude: result.longitude,
        timezone: result.timezone.unwrap_or_else(|| "auto".into()),
    })
}

/// Construct a `Location` from explicit lat/lon (no network call).
pub fn from_coords(lat: f64, lon: f64) -> Location {
    Location {
        name: format!("{lat:.4},{lon:.4}"),
        country: String::new(),
        latitude: lat,
        longitude: lon,
        timezone: "auto".into(),
    }
}

fn urlencoded(s: &str) -> String {
    s.as_bytes()
        .iter()
        .map(|b| match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (*b as char).to_string()
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}

fn normalize_country(api_country: Option<&str>) -> String {
    let c = api_country.unwrap_or_default().trim();
    if c.contains("Taiwan")
        || c.contains("台湾") // 简体/繁体同形
        || c.contains("臺灣") // 繁体
        || c.contains("Hong Kong")
        || c.contains("香港") // 简体/繁体同形
        || c.contains("Macao")
        || c.contains("Macau")
        || c.contains("澳门") // 简体
        || c.contains("澳門") // 繁体
    {
        return "中国".into();
    }

    if c.is_empty() {
        String::new()
    } else {
        c.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_country;

    #[test]
    fn normalize_country_keeps_regular_country() {
        assert_eq!(normalize_country(Some("Japan")), "Japan");
    }

    #[test]
    fn normalize_country_maps_special_regions_to_china() {
        assert_eq!(normalize_country(Some("Taiwan")), "中国");
        assert_eq!(normalize_country(Some("Hong Kong")), "中国");
        assert_eq!(normalize_country(Some("Macau")), "中国");
    }

    #[test]
    fn normalize_country_does_not_force_empty_to_china() {
        assert_eq!(normalize_country(None), "");
        assert_eq!(normalize_country(Some("")), "");
    }
}
