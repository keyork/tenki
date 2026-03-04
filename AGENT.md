# tenki — 有调性的终端天气 CLI

tenki (天気) 是一个用 Rust 编写的终端天气工具。它通过高质量 ASCII art、精心设计的 256 色配色、和克制的信息布局，让命令行里也能感受天气的温度。

## Commands

```bash
cargo build                    # 构建
cargo run                      # 运行（自动定位）
cargo run -- tokyo             # 查东京天气
cargo run -- --forecast        # 带预报
cargo run -- --mode compact    # 紧凑模式
cargo test                     # 运行测试
cargo clippy                   # lint
cargo fmt -- --check           # 格式检查
```

## Architecture

```text
src/
├── main.rs           # CLI 入口（clap），协调各模块
├── config.rs         # TOML 配置加载（~/.config/tenki/config.toml）
├── location.rs       # 位置解析：CLI 参数 > 配置 > IP 自动定位
├── weather.rs        # Open-Meteo HTTP 客户端
├── model.rs          # 所有数据结构定义
├── units.rs          # 公制/英制单位换算
├── art/
│   ├── mod.rs        # ArtRegistry：weather code → ASCII art 分发
│   ├── pieces.rs     # 12 种天气状态的 ASCII art 定义
│   └── colors.rs     # 语义色彩枚举（SunCore, CloudBody, RainDrop...）
├── theme/
│   ├── mod.rs        # Theme trait：语义色 → ANSI 256 色映射
│   ├── default.rs    # 深色终端主题
│   ├── light.rs      # 浅色终端主题
│   └── mono.rs       # 单色灰度主题
└── render/
    ├── mod.rs        # Renderer trait
    ├── card.rs       # Card 模式：带圆角边框的信息卡片
    ├── compact.rs    # Compact 模式：无边框紧凑输出
    ├── oneline.rs    # Oneline 模式：单行，tmux/prompt 用
    └── chart.rs      # 24h 温度 mini-chart（Unicode block chars）
```

## Tech Stack

- **Rust** (edition 2021, MSRV 1.75)
- **clap** (derive) — CLI 参数解析
- **ureq** — 同步 HTTP 客户端（轻量，CLI 场景不需要 async）
- **serde** + **serde_json** — JSON 反序列化
- **toml** + **serde** — 配置文件解析
- **crossterm** — 跨平台终端 ANSI 色彩输出
- **dirs** — XDG 目录规范路径

不使用 tokio/async，这是一个快进快出的 CLI 工具，同步 IO 更简单。

## API Endpoints

所有外部 API 都是免费的，不需要 API key。

### 天气数据 — Open-Meteo Forecast API

```text
GET https://api.open-meteo.com/v1/forecast
  ?latitude={lat}&longitude={lon}
  &current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,weather_code,wind_speed_10m,wind_direction_10m,is_day
  &hourly=temperature_2m,precipitation_probability,weather_code
  &daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum
  &timezone=auto
  &forecast_days=3
```

单次请求获取 current + hourly + daily。返回 JSON。

### 城市名 → 坐标 — Open-Meteo Geocoding API

```text
GET https://geocoding-api.open-meteo.com/v1/search
  ?name={city_name}
  &count=1
  &language=en
```

返回 `{ "results": [{ "name", "latitude", "longitude", "country", "timezone", ... }] }`

### IP 自动定位

```text
GET http://ip-api.com/json/?fields=status,city,country,lat,lon
```

免费，无需 key，限速 45 req/min（CLI 场景绰绰有余）。

## Core Data Model

```rust
struct WeatherData {
    location: Location,        // name, country, lat, lon
    current: CurrentWeather,   // temp, feels_like, humidity, wind, precip, code, is_day
    hourly: Vec<HourlyPoint>,  // 24 个时间点：temp, precip_prob, code
    daily: Vec<DailyForecast>, // 3 天：code, temp_max, temp_min, precip_sum
}

// WMO weather code → 内部天气状态
enum WeatherCondition {
    ClearSky,           // WMO 0
    PartlyCloudy,       // WMO 1, 2
    Overcast,           // WMO 3
    Fog,                // WMO 45, 48
    LightDrizzle,       // WMO 51, 53, 56
    LightRain,          // WMO 61, 63, 66
    HeavyRain,          // WMO 55, 65, 67
    Thunderstorm,       // WMO 95, 96, 99
    LightSnow,          // WMO 71, 73, 77, 85
    HeavySnow,          // WMO 75, 86
}
```

每个 `WeatherCondition` 有 day/night 两种 ASCII art 变体（通过 `is_day` 切换）。

## WMO Weather Code 完整映射表

必须实现这个映射，它是 Open-Meteo 的核心返回字段：

| Code | Meaning | → WeatherCondition |
| ------ | --------- | ------------------- |
| 0 | Clear sky | ClearSky |
| 1 | Mainly clear | PartlyCloudy |
| 2 | Partly cloudy | PartlyCloudy |
| 3 | Overcast | Overcast |
| 45 | Fog | Fog |
| 48 | Depositing rime fog | Fog |
| 51 | Light drizzle | LightDrizzle |
| 53 | Moderate drizzle | LightDrizzle |
| 55 | Dense drizzle | HeavyRain |
| 56 | Light freezing drizzle | LightDrizzle |
| 57 | Dense freezing drizzle | HeavyRain |
| 61 | Slight rain | LightRain |
| 63 | Moderate rain | LightRain |
| 65 | Heavy rain | HeavyRain |
| 66 | Light freezing rain | LightRain |
| 67 | Heavy freezing rain | HeavyRain |
| 71 | Slight snow | LightSnow |
| 73 | Moderate snow | LightSnow |
| 75 | Heavy snow | HeavySnow |
| 77 | Snow grains | LightSnow |
| 80 | Slight rain showers | LightRain |
| 81 | Moderate rain showers | LightRain |
| 82 | Violent rain showers | HeavyRain |
| 85 | Slight snow showers | LightSnow |
| 86 | Heavy snow showers | HeavySnow |
| 95 | Thunderstorm | Thunderstorm |
| 96 | Thunderstorm with slight hail | Thunderstorm |
| 99 | Thunderstorm with heavy hail | Thunderstorm |

## ASCII Art 设计规范

这是本项目的灵魂，必须用心做。

**尺寸**：每幅 art 宽 20-30 列、高 7-10 行。保持所有 art 同一尺寸以便对齐。

**风格要求**：手绘感、有细节、有场景氛围——不是粗暴的几何线条。

**着色方式**：art 定义中使用语义色标记（ArtColor 枚举），由 Theme 映射到具体 ANSI 256 色值。这样同一幅 art 在不同主题下自动适配。

```rust
enum ArtColor {
    SunCore,       // 太阳中心
    SunRay,        // 太阳光线
    CloudLight,    // 亮色云体
    CloudDark,     // 暗色云体（雨云）
    RainDrop,      // 雨滴
    SnowFlake,     // 雪花
    Lightning,     // 闪电
    FogMist,       // 雾
    MoonBody,      // 月亮
    Star,          // 星星
    Ground,        // 可选的地面元素
}

// 每一行由多个带色段组成
struct ArtSegment {
    text: String,
    color: ArtColor,
}
type ArtLine = Vec<ArtSegment>;
type ArtPiece = Vec<ArtLine>;
```

**12 种天气状态的 ASCII art 参考**（日间版本，需另做夜间版本）：

晴天——有层次的太阳，光线辐射感：

```text
       \    |    /
        '-.___.-'
       ‒ (     ) ‒
        .-'```'-.
       /    |    \
```

SunCore 着芯体 `(     )`，SunRay 着光线 `\ | /`。

多云——太阳半遮挡：

```text
    \  /
  _ /"".-.
    \_(   ).
    /(___(__) 
```

阴天——厚云层叠：

```text
              
     .--.     
  .-(    ).   
 (___.__)__)  
              
```

小雨——云+细雨丝：

```text
     .--.     
  .-(    ).   
 (___.__)__)  
  ╱ ╱ ╱ ╱    
 ╱ ╱ ╱ ╱     
```

大雨——更密的雨 + 更暗的云：

```text
     .--.     
  .-(    ).   
 (___.__)__)  
 ‖╱‖╱‖╱‖╱   
 ╱‖╱‖╱‖╱‖   
 ‖╱‖╱‖╱‖╱   
```

雷暴——加闪电：

```text
     .--.     
  .-(    ).   
 (___.__)__)  
  ⚡╱ ╱⚡╱    
 ╱ ╱⚡╱ ╱     
```

小雪——柔和的雪花：

```text
     .--.     
  .-(    ).   
 (___.__)__)  
  *  *  *     
    *  *  *   
  *  *  *     
```

大雪——密集：

```text
     .--.     
  .-(    ).   
 (___.__)__)  
 * * * * * *  
  * * * * *   
 * * * * * *  
  * * * * *   
```

雾——水平层叠线条：

```text
               
 _ - _ - _ -  
  _ - _ - _   
 _ - _ - _ -  
  _ - _ - _   
 _ - _ - _ -  
               
```

晴夜——月亮+星星：

```text
        *        
    .  *    .    
      _....._    
    .' ° ° '.   
   / °    °  \  
   | °  °    |  
    '.  ° °.'   
  *   '-----' * 
```

MoonBody 着月亮，Star 着星星 `*`。

这些只是参考，实际实现时请精心调整每一个字符的位置，确保视觉平衡。

## 三种输出模式

### Card 模式（默认）

带圆角 Unicode 边框的信息卡片，左侧 ASCII art，右侧数据：

```text
╭──────────────────────────────────────────╮
│                                          │
│      .--.        Amsterdam, NL           │
│   .-(    ).      ☁  Overcast             │
│  (___.__)__)                             │
│                  🌡  14°C  feels 11°C    │
│                  💨  18 km/h SW          │
│                  💧  72%    ☂ 0.2 mm     │
│                                          │
│  ── 24h ─────────────────────────────    │
│  06  09  12  15  18  21  00              │
│  12° 13° 15° 14° 13° 11° 10°            │
│  ▂▂  ▃▃  ▅▅  ▄▄  ▃▃  ▂▂  ▁▁            │
│                                          │
╰──────────────────────────────────────────╯
```

圆角字符：`╭ ╮ ╰ ╯ │ ─`

### Compact 模式

```text
    .--.       Amsterdam, NL — Overcast
 .-(    ).     14°C (feels 11°C) | 💨 18 km/h | 💧 72%
(___.__)__)
```

### Oneline 模式

```text
Amsterdam: ☁ 14°C 💨18km/h 💧72%
```

用 `--mode card|compact|oneline` 切换，配置文件也可设置默认值。

## Theme 系统

语义色到具体 ANSI 256 色的映射。使用 crossterm 的 `Color::AnsiValue(n)` 输出。

### default 主题（深色终端）

```text
SunCore     → 220 (亮金)     SunRay      → 228 (淡金)
CloudLight  → 250 (亮灰)     CloudDark   → 245 (深灰)
RainDrop    → 111 (淡蓝)     SnowFlake   → 255 (亮白)
Lightning   → 226 (亮黄)     FogMist     → 249 (雾灰)
MoonBody    → 230 (暖白)     Star        → 228 (暖黄)

title_color → 117 (天空蓝)   temp_color  → 220 (暖金)
info_color  → 250 (银灰)     dim_color   → 244 (暗灰)
border_color→ 239 (深边框)   highlight   → 203 (珊瑚红，高温警示)
cold_color  → 117 (冰蓝，低温警示)
```

### light 主题（浅色终端）

ASCII art 用深色描边，数据文字用深色。将 CloudLight/CloudDark 映射到更深的灰度。

### mono 主题

全部映射到灰度色阶（232-255），通过明暗层次而非色相区分元素。

## 温度 Mini-Chart

使用 Unicode block elements 绘制 24h 温度趋势：`▁ ▂ ▃ ▄ ▅ ▆ ▇ █`（U+2581-U+2588）。

算法：取 hourly 温度数组中的 min 和 max，线性映射到 8 级高度。每 3 小时采样一个点显示。

Chart 的色彩也通过 theme 控制（低温用 cold_color，高温用 highlight）。

## 配置文件

路径：`~/.config/tenki/config.toml`（用 dirs crate 获取 XDG config 目录）。

首次运行时如果不存在，不创建配置文件（零配置可用）。用户手动创建即可生效。

```toml
[location]
city = "Amsterdam"
# 或者直接指定坐标（优先级高于 city）：
# latitude = 52.3676
# longitude = 4.9041

[display]
units = "metric"       # metric | imperial
mode = "card"          # card | compact | oneline
theme = "default"      # default | light | mono
show_chart = true      # 是否显示 24h 温度 mini-chart

# [cache]
# ttl_seconds = 300    # 缓存时间，避免频繁请求（P2 特性）
```

CLI 参数覆盖配置文件，配置文件覆盖默认值。

## CLI 参数定义（clap derive）

```text
tenki [CITY] [OPTIONS]

Arguments:
  [CITY]              City name to query (omit for auto-detect)

Options:
  -m, --mode <MODE>   Output mode [default: card] [values: card, compact, oneline]
  -u, --units <UNITS> Unit system [default: metric] [values: metric, imperial]
  -t, --theme <THEME> Color theme [default: default] [values: default, light, mono]
  -f, --forecast      Show 3-day forecast (card mode only)
      --no-chart      Hide the 24h temperature mini-chart
      --lat <LAT>     Latitude (overrides city/auto-detect)
      --lon <LON>     Longitude (overrides city/auto-detect)
  -h, --help          Print help
  -V, --version       Print version
```

## 实现顺序

按此顺序逐步构建，每步完成后应可编译运行：

1. **脚手架**：`cargo init tenki`，添加 Cargo.toml 依赖，创建目录结构，实现 clap CLI 解析，`main.rs` 打印 "tenki v0.1.0"
2. **数据模型**：`model.rs` 定义所有 struct/enum，包含 WMO code 映射
3. **天气获取**：`weather.rs` 实现 Open-Meteo 客户端，硬编码 Amsterdam 坐标测试
4. **位置解析**：`location.rs` 实现 IP 定位 + geocoding，串联到 main
5. **ASCII art**：`art/` 模块，先实现 6 种核心天气 art（晴/多云/阴/雨/雪/雷），日间版本
6. **Theme 系统**：`theme/` 模块，实现 default 主题的语义色映射
7. **Card 渲染器**：`render/card.rs`，组合 art + 数据 + 边框，输出到终端
8. **Mini-chart**：`render/chart.rs`，24h 温度趋势
9. **Compact + Oneline**：`render/compact.rs` 和 `render/oneline.rs`
10. **配置系统**：`config.rs`，TOML 解析 + CLI 参数合并
11. **夜间 art 变体**：补充 6 种夜间版本
12. **light + mono 主题**
13. **3 天预报**（--forecast）
14. **单位换算**：`units.rs`，°C↔°F、km/h↔mph、mm↔in
15. **错误处理打磨**：网络超时、API 错误、无结果的友好提示

## 重要约定

- 使用 `crossterm::style` 输出彩色文本，不要手写 ANSI escape codes
- 所有网络请求设置 5 秒超时
- 用 `eprintln!` 输出错误信息，`println!`/`print!` 输出正常内容（方便管道）
- ASCII art 中不要使用 emoji 字符（兼容性差），信息区域可以用 emoji 作为图标
- 保持 `cargo clippy` 无警告
- 公开 API 的函数和结构体写 doc comment
- 二进制名为 `tenki`（在 Cargo.toml 的 `[[bin]]` 中设置）
