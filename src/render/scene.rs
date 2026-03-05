use crate::{art::colors::ArtColor, model::WeatherCondition, theme::Theme};
use crossterm::style::Color;

pub struct Scene {
    /// One string per row, each exactly `width` display-columns wide (all narrow chars).
    pub lines: Vec<String>,
    pub color: Color,
}

// ── xorshift64 RNG (no external deps) ────────────────────────────────────────

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        let mut r = Rng(seed ^ 0xcafe_babe_dead_c0de);
        for _ in 0..8 {
            r.step();
        }
        r
    }
    fn step(&mut self) {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
    }
    fn next(&mut self) -> u64 {
        self.step();
        self.0
    }
    fn usize(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        (self.next() % max as u64) as usize
    }
    fn chance(&mut self, num: usize, den: usize) -> bool {
        den > 0 && self.usize(den) < num
    }
}

fn ridge(col: usize, row: usize, period: usize, drift: usize) -> bool {
    ((col + drift + row * 2) % period) < period / 3
}

fn cloud_band(row: usize, height: usize, bias: usize) -> usize {
    let top_weight = height.saturating_sub(row);
    bias + top_weight.saturating_mul(10) / height.max(1)
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn generate(
    condition: WeatherCondition,
    is_day: bool,
    width: usize,
    height: usize,
    seed: u64,
    theme: &dyn Theme,
) -> Scene {
    use WeatherCondition::*;
    let mut rng = Rng::new(seed);
    let (lines, color) = match (condition, is_day) {
        (ClearSky, true) => sky_day(&mut rng, width, height, theme),
        (ClearSky, false) | (PartlyCloudy, false) => sky_night(&mut rng, width, height, theme),
        (PartlyCloudy, true) => partly_cloudy(&mut rng, width, height, theme),
        (Overcast, _) => overcast(&mut rng, width, height, theme),
        (Fog, _) => fog(&mut rng, width, height, theme),
        (LightDrizzle | LightRain, _) => light_rain(&mut rng, width, height, theme),
        (HeavyRain, _) => heavy_rain(&mut rng, width, height, theme),
        (Thunderstorm, _) => thunderstorm(&mut rng, width, height, theme),
        (LightSnow, _) => light_snow(&mut rng, width, height, theme),
        (HeavySnow, _) => heavy_snow(&mut rng, width, height, theme),
    };
    Scene { lines, color }
}

pub fn animate(base: &Scene, condition: WeatherCondition, is_day: bool, frame: u64) -> Scene {
    if base.lines.is_empty() {
        return Scene {
            lines: Vec::new(),
            color: base.color,
        };
    }

    let mut lines = base.lines.clone();
    use WeatherCondition::*;
    match condition {
        ClearSky | PartlyCloudy => {
            apply_sway(&mut lines, frame, 1, 3);
            if !is_day {
                twinkle(&mut lines, frame, 37);
            }
        }
        Overcast => {
            apply_sway(&mut lines, frame, 1, 6);
        }
        Fog => {
            apply_fog_flow(&mut lines, frame);
        }
        LightDrizzle | LightRain | HeavyRain | Thunderstorm => {
            let speed = if matches!(condition, LightDrizzle | LightRain) {
                3
            } else {
                2
            };
            lines = scroll_down(&lines, ((frame / speed) as usize) % lines.len());
            apply_rain_shear(&mut lines, frame);
            if matches!(condition, Thunderstorm) {
                animate_lightning(&mut lines, frame);
            }
        }
        LightSnow | HeavySnow => {
            let speed = if matches!(condition, LightSnow) { 4 } else { 3 };
            lines = scroll_down(&lines, ((frame / speed) as usize) % lines.len());
            apply_sway(&mut lines, frame, 1, 4);
            twinkle(&mut lines, frame, 53);
        }
    }

    Scene {
        lines,
        color: base.color,
    }
}

// ── Scene generators ──────────────────────────────────────────────────────────

/// Clear daytime sky — sparse drifting dots, occasional high birds.
fn sky_day(rng: &mut Rng, width: usize, height: usize, theme: &dyn Theme) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let drift = rng.usize(11);
        for col in 0..width {
            let horizon = row > (height * 2) / 3 && ridge(col, row, 13, drift);
            let c = if row < height / 4 && rng.chance(1, 90) {
                'v'
            } else if horizon {
                if rng.chance(1, 4) {
                    '_'
                } else {
                    '.'
                }
            } else if (ridge(col, row, 19, drift) && rng.chance(1, 8)) || rng.chance(1, 45) {
                '.'
            } else {
                ' '
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.dim_color())
}

/// Clear/partly-cloudy night — scattered stars of varying brightness.
fn sky_night(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    let nebula_row = rng.usize(height.max(1));
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let drift = rng.usize(17);
        for col in 0..width {
            let band = row.abs_diff(nebula_row) <= 1 && ridge(col, row, 9, drift);
            let c = if band {
                if rng.chance(1, 3) {
                    '+'
                } else {
                    '.'
                }
            } else {
                match rng.usize(90) {
                    0 => '+',
                    1..=3 => '*',
                    4..=10 => '.',
                    _ => ' ',
                }
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::Star))
}

/// Partly cloudy — sky with light cloud texture denser near the top.
fn partly_cloudy(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let cloud_num = cloud_band(row, height, 6);
        let drift = rng.usize(9);
        for col in 0..width {
            let n = rng.usize(100);
            let c = if ridge(col, row, 11, drift) && n < cloud_num + 8 {
                if rng.chance(1, 2) {
                    ':'
                } else {
                    '~'
                }
            } else if n < cloud_num {
                if rng.chance(2, 3) {
                    ':'
                } else {
                    '.'
                }
            } else if row > height / 2 && n < cloud_num + 3 {
                '.'
            } else {
                ' '
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::CloudLight))
}

/// Overcast — heavy cloud texture, denser at the top.
fn overcast(rng: &mut Rng, width: usize, height: usize, theme: &dyn Theme) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let density = cloud_band(row, height, 10);
        let drift = rng.usize(7);
        for col in 0..width {
            let n = rng.usize(100);
            let c = if ridge(col, row, 8, drift) && n < density + 10 {
                '='
            } else if n < density {
                if row < height / 2 {
                    '#'
                } else {
                    ':'
                }
            } else if n < density + 8 {
                '.'
            } else {
                ' '
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::CloudDark))
}

/// Fog — alternating waves of `~` and `-` drifting across the scene.
fn fog(rng: &mut Rng, width: usize, height: usize, theme: &dyn Theme) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let phase = rng.usize(5);
        for col in 0..width {
            let c = if row % 3 == 0 {
                if (col + phase) % 6 < 4 {
                    '~'
                } else {
                    ' '
                }
            } else if row % 3 == 1 {
                if (col + phase + 2) % 7 < 4 {
                    '-'
                } else {
                    ' '
                }
            } else if (col + phase + 1) % 8 < 5 {
                '='
            } else {
                ' '
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::FogMist))
}

/// Light rain — sparse diagonal streaks.
fn light_rain(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let offset = rng.usize(9);
        for col in 0..width {
            let c = if (col + row * 2 + offset) % 9 == 0 {
                '/'
            } else if row > height / 2 && (col + row + offset) % 13 == 0 {
                '.'
            } else {
                ' '
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::RainDrop))
}

/// Heavy rain — dense rain with vertical streaks mixed in.
fn heavy_rain(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let offset = rng.usize(4);
        for col in 0..width {
            let c = match (col + row * 2 + offset) % 4 {
                0 => '/',
                1 => '|',
                2 if row > height / 2 && rng.chance(1, 5) => '.',
                _ => ' ',
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::RainDrop))
}

/// Thunderstorm — heavy rain with a few random lightning strike columns.
fn thunderstorm(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let num_bolts = 1 + rng.usize(3);
    let bolts: Vec<(usize, usize)> = (0..num_bolts)
        .map(|_| (rng.usize(height.max(1)), rng.usize(width.max(1))))
        .collect();

    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let offset = rng.usize(4);
        for col in 0..width {
            let c = if bolts.iter().any(|&(r, c)| r == row && c == col) {
                '!'
            } else {
                match (col + row * 2 + offset) % 4 {
                    0 => '/',
                    1 => '|',
                    2 if row > height / 2 && rng.chance(1, 6) => '.',
                    _ => ' ',
                }
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::RainDrop))
}

/// Light snow — sparse flakes drifting gently.
fn light_snow(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let drift = rng.usize(7);
        for col in 0..width {
            let c = if ridge(col, row, 15, drift) && rng.chance(1, 6) {
                '*'
            } else {
                match rng.usize(28) {
                    0 => '+',
                    1 | 2 => '.',
                    _ => ' ',
                }
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::SnowFlake))
}

/// Heavy snow — dense flakes, some larger `*` crystals.
fn heavy_snow(
    rng: &mut Rng,
    width: usize,
    height: usize,
    theme: &dyn Theme,
) -> (Vec<String>, Color) {
    let mut lines = Vec::with_capacity(height);
    for row in 0..height {
        let mut line = String::with_capacity(width);
        let drift = rng.usize(9);
        for col in 0..width {
            let c = if ridge(col, row, 11, drift) && rng.chance(1, 2) {
                '*'
            } else {
                match rng.usize(10) {
                    0 => '*',
                    1 => '+',
                    2..=4 => '.',
                    _ => ' ',
                }
            };
            line.push(c);
        }
        lines.push(line);
    }
    (lines, theme.art_color(ArtColor::SnowFlake))
}

fn wave(frame: u64, amp: isize) -> isize {
    if amp <= 0 {
        return 0;
    }
    let p = match frame % 8 {
        0 | 4 => 0,
        1 | 2 => 1,
        3 => 0,
        5 | 6 => -1,
        _ => 0,
    };
    p * amp
}

fn apply_sway(lines: &mut [String], frame: u64, amp: isize, speed_div: u64) {
    if amp <= 0 || speed_div == 0 {
        return;
    }
    let phase = frame / speed_div;
    for (row, line) in lines.iter_mut().enumerate() {
        let offset = wave(phase + row as u64 / 3, amp);
        *line = shift_line(line, offset);
    }
}

fn apply_fog_flow(lines: &mut [String], frame: u64) {
    for (row, line) in lines.iter_mut().enumerate() {
        let base = if row % 2 == 0 {
            wave(frame / 2 + row as u64, 1)
        } else {
            wave(frame / 3 + row as u64, 1)
        };
        *line = shift_line(line, base);
    }
}

fn apply_rain_shear(lines: &mut [String], frame: u64) {
    for (row, line) in lines.iter_mut().enumerate() {
        let offset = match (row + frame as usize) % 4 {
            0 => -1,
            1 => 0,
            2 => -1,
            _ => 0,
        };
        *line = shift_line(line, offset);
    }
}

fn animate_lightning(lines: &mut [String], frame: u64) {
    let flash = frame % 14 == 0 || frame % 14 == 1;
    if !flash {
        return;
    }

    for line in lines.iter_mut() {
        let mut out = String::with_capacity(line.len());
        for ch in line.chars() {
            let next = match ch {
                '!' => '+',
                '+' => '*',
                '*' => '+',
                _ => ch,
            };
            out.push(next);
        }
        *line = out;
    }
}

fn shift_line(line: &str, offset: isize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let width = chars.len();
    if width == 0 {
        return String::new();
    }

    if offset > 0 {
        let off = (offset as usize).min(width);
        let mut out = String::with_capacity(width);
        out.push_str(&" ".repeat(off));
        out.extend(chars.into_iter().take(width - off));
        out
    } else {
        let off = ((-offset) as usize).min(width);
        let mut out = String::with_capacity(width);
        out.extend(chars.into_iter().skip(off));
        out.push_str(&" ".repeat(off));
        out
    }
}

fn scroll_down(lines: &[String], by: usize) -> Vec<String> {
    let h = lines.len();
    if h == 0 {
        return Vec::new();
    }
    let shift = by % h;
    (0..h)
        .map(|row| lines[(row + h - shift) % h].clone())
        .collect()
}

fn twinkle(lines: &mut [String], frame: u64, cadence: u64) {
    for (row, line) in lines.iter_mut().enumerate() {
        let mut out = String::with_capacity(line.len());
        for (col, ch) in line.chars().enumerate() {
            let pulse = (row as u64 * 31 + col as u64 * 17 + frame * 13) % cadence == 0;
            let next = if pulse {
                match ch {
                    '.' => '*',
                    '*' => '+',
                    '+' => '.',
                    _ => ch,
                }
            } else {
                ch
            };
            out.push(next);
        }
        *line = out;
    }
}
