# tenki 技术文档

## 项目概况

| 项目 | 值 |
| ------ | ----- |
| 语言 | Rust 2021 edition |
| MSRV | 1.75 |
| 二进制名 | `tenki` |
| 许可证 | MIT |
| 依赖数 | 7 个直接依赖 |
| 网络模型 | 同步 IO（无 async 运行时） |

---

## 目录结构

```text
src/
├── main.rs           # CLI 入口，参数解析，模块协调
├── config.rs         # TOML 配置加载
├── location.rs       # 位置解析（CLI / 配置 / IP）
├── weather.rs        # Open-Meteo HTTP 客户端
├── model.rs          # 核心数据结构与枚举
├── units.rs          # 公制/英制换算
├── art/
│   ├── mod.rs        # get_art() 分发函数
│   ├── pieces.rs     # 12 种天气 ASCII art 定义（ART_WIDTH = 22）
│   └── colors.rs     # ArtColor 语义色枚举
├── theme/
│   ├── mod.rs        # Theme trait + resolve()
│   ├── default.rs    # 深色主题
│   ├── light.rs      # 浅色主题
│   ├── mono.rs       # 单色主题
│   ├── ocean.rs      # 海洋主题
│   ├── forest.rs     # 森林主题
│   └── sunset.rs     # 日落主题
└── render/
    ├── mod.rs        # RenderContext + 共享工具函数
    ├── card.rs       # card 渲染器
    ├── compact.rs    # compact 渲染器
    ├── oneline.rs    # oneline 渲染器
    ├── fullscreen.rs # fullscreen 渲染器（交互式事件循环）
    ├── chart.rs      # 24h 温度趋势图（含 wide 变体）
    └── scene.rs      # 全屏背景图案生成器

docs/
├── product.md        # 产品说明
├── tech.md           # 技术说明
└── design.md         # ASCII 图稿基线（与 pieces.rs 对齐）
```

---

## 依赖说明

| 依赖 | 版本 | 用途 |
| ------ | ------ | ------ |
| `clap` | 4（derive） | CLI 参数解析，`#[derive(Parser)]` 方式 |
| `ureq` | 2（json feature） | 同步 HTTP 客户端，JSON 反序列化 |
| `serde` | 1（derive） | 序列化/反序列化框架 |
| `serde_json` | 1 | JSON 反序列化（Open-Meteo 响应） |
| `toml` | 0.8 | 配置文件解析 |
| `crossterm` | 0.27 | 跨平台终端控制（颜色、光标、原始模式） |
| `dirs` | 5 | XDG 标准目录路径（`~/.config`） |

**注意：** crossterm 0.27 中不存在 `crossterm::Result`，错误类型使用 `std::io::Result`。

---

## 数据模型

```rust
// 地理位置
struct Location {
    name: String,
    country: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
}

// 当前天气
struct CurrentWeather {
    temperature: f64,      // 气温（°C）
    feels_like: f64,       // 体感温度（°C）
    humidity: f64,         // 湿度（0-100）
    wind_speed: f64,       // 风速（km/h）
    wind_direction: f64,   // 风向（度，0-360）
    precipitation: f64,    // 降水量（mm）
    weather_code: u16,     // WMO 天气代码
    is_day: bool,          // 日间/夜间标志
}

// 小时数据点
struct HourlyPoint {
    hour: u8,
    temperature: f64,
    precip_probability: f64,
    weather_code: u16,
}

// 每日预报
struct DailyForecast {
    date: String,          // "YYYY-MM-DD"
    weather_code: u16,
    temp_max: f64,
    temp_min: f64,
    precip_sum: f64,
}

// 完整天气响应
struct WeatherData {
    location: Location,
    current: CurrentWeather,
    hourly: Vec<HourlyPoint>,   // 24 个时间点
    daily: Vec<DailyForecast>,  // 3 天
}
```

### WeatherCondition 枚举与 WMO 代码映射

```rust
enum WeatherCondition {
    ClearSky,      // WMO 0
    PartlyCloudy,  // WMO 1, 2
    Overcast,      // WMO 3
    Fog,           // WMO 45, 48
    LightDrizzle,  // WMO 51, 53, 56
    LightRain,     // WMO 61, 63, 66, 80, 81
    HeavyRain,     // WMO 55, 57, 65, 67, 82
    LightSnow,     // WMO 71, 73, 77, 85
    HeavySnow,     // WMO 75, 86
    Thunderstorm,  // WMO 95, 96, 99
}
```

---

## 配置系统

### 配置文件结构（TOML）

```toml
[location]
city = "Tokyo"          # Option<String>
latitude = 35.6762      # Option<f64>，与 longitude 配合使用
longitude = 139.6503    # Option<f64>

[display]
units = "metric"        # "metric" | "imperial"，默认 "metric"
mode = "card"           # "card" | "compact" | "oneline" | "fullscreen"，默认 "card"
                         # 也支持 "showcase"（5 秒后自动退出）
theme = "default"       # "default" | "light" | "mono" | "ocean" | "forest" | "sunset"
                        # 默认 "default"
show_chart = true       # bool，默认 true
```

### 配置加载流程

```text
config::load()
  → dirs::config_dir()           // 获取 ~/.config 路径
  → read_to_string("tenki/config.toml")
  → toml::from_str()             // 反序列化
  → 失败时返回 Config::default() // 零配置可用
```

### 参数优先级

```text
CLI 参数 > 配置文件 > 程序默认值
```

main.rs 中通过 `std::env::args()` 检测 CLI 参数是否存在来决定是否覆盖配置文件。

---

## 位置解析模块

`location.rs` 提供三种位置来源：

### 1. 坐标直接使用

```rust
pub fn from_coords(lat: f64, lon: f64) -> Location
```

创建一个不发网络请求的占位 Location（name 设为坐标字符串）。

### 2. 城市名 → 坐标（Geocoding API）

```rust
pub fn from_city(name: &str) -> Result<Location, String>
```

请求 Open-Meteo Geocoding API（无需 API Key）：

```text
GET https://geocoding-api.open-meteo.com/v1/search
    ?name={city_name}&count=1&language=zh&format=json
```

返回 `results[0]` 中的城市、国家/地区、坐标等字段。  
代码中对 `台湾/香港/澳门` 统一归属显示为 `中国`。

### 3. IP 自动定位

```rust
pub fn from_ip() -> Result<Location, String>
```

请求 ip-api.com：

```text
GET http://ip-api.com/json/?fields=status,city,country,lat,lon
```

免费，限速 45 次/分。

---

## 天气数据模块

`weather.rs` 封装 Open-Meteo Forecast API 调用。

### API 请求结构

```text
GET https://api.open-meteo.com/v1/forecast
    ?latitude={lat}
    &longitude={lon}
    &current=temperature_2m,relative_humidity_2m,apparent_temperature,
             precipitation,weather_code,wind_speed_10m,wind_direction_10m,is_day
    &hourly=temperature_2m,precipitation_probability,weather_code
    &daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum
    &timezone=auto
    &forecast_days=3
```

单次请求获取 current + hourly（24h）+ daily（3 天）全部数据。

### JSON 反序列化

使用 serde 派生宏，定义与 API 响应结构对应的中间类型，再转换为内部 `WeatherData`。

---

## 渲染系统

### RenderContext

所有渲染器共享的输入上下文：

```rust
pub struct RenderContext<'a> {
    pub data: &'a WeatherData,
    pub theme: &'a dyn Theme,
    pub units: Units,
    pub show_chart: bool,
    pub show_forecast: bool,
}
```

### 共享工具函数（render/mod.rs）

```rust
pub(crate) fn visible_len(s: &str) -> usize
pub(crate) fn is_wide(c: char) -> bool
```

`visible_len` 剥离 ANSI 转义序列后计算实际显示宽度，将 emoji 和 CJK 字符计为 2 列。`is_wide` 覆盖 Unicode 宽字符范围（Hangul、CJK、Emoji block 等）。

### card 渲染器

固定宽度 78 列，带圆角 Unicode 边框（`╭ ╮ ╰ ╯ │ ─`）。左侧 26 列场景（含 22 列 ASCII art），右侧 44 列气象数据。

可选：24h 趋势图区块 + 3 天预报区块，均内嵌于同一边框内。

### compact 渲染器

无边框，art 与数据左右并排，无固定宽度约束。

### oneline 渲染器

单行：`城市名: 图标 温度 风速 湿度`。

### fullscreen 渲染器

**交互式事件循环模式**，使用 crossterm 原始模式：

```text
render()
  → enable_raw_mode()
  → EnterAlternateScreen + cursor::Hide
  → run_loop()
      → draw_frame()（首次绘制）
      → event::poll(120ms) 循环
          → Key(Q / Esc / Ctrl+C) → break
      → 每 tick 递增 frame 并 draw_frame()（持续动画）
  → cursor::Show + LeaveAlternateScreen
  → disable_raw_mode()
```

当 CLI 传入 `--static` 时：
- 背景与主图动画关闭（frame 固定为 0）
- `fullscreen` 仅在 resize 时重绘
- `showcase` 为更新倒计时会低频重绘（静态画面）

`draw_frame` 在每次绘制前调用 `cursor::MoveTo(0,0)` + `Clear(All)` 清屏，然后按顺序输出各区块（分隔线 → 标题 → 数据 → 场景 → 图表 → 预报）。

**原始模式注意事项：** 所有换行必须使用 `\r\n`，不能使用 `println!`。本项目在 `nl()` 辅助函数中封装：

```rust
fn nl<W: Write>(out: &mut W) -> io::Result<()> {
    crossterm::execute!(out, Print("\r\n"))
}
```

### 全屏场景布局计算

```text
固定行数 FIXED_ROWS = 16
场景行数 = max(terminal_rows - FIXED_ROWS, MIN_SCENE_ROWS=6)

art 水平居中：art_col = (width - ART_WIDTH) / 2
art 垂直居中：art_row_start = (scene_rows - art_height) / 2
```

art 与背景的合成：

- 非 art 行：整行输出背景字符串（bg_color）
- art 行：`bg[0..art_col]` + art 各色段 + `bg[art_col+ART_WIDTH..]`
- 字符串切片使用 `.chars().take/skip()` 而非字节索引，确保多字节字符安全
- art 会依据天气类型做轻微位移（sway/bob），形成“呼吸感”

---

## 背景场景生成器（scene.rs）

为 fullscreen 模式生成天气主题的 ASCII 背景纹理。

### 随机数生成器

内置 xorshift64 算法，无外部依赖：

```rust
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self { /* 初始化 + 8 次预热 */ }
    fn step(&mut self) {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
    }
}
```

种子来源：`SystemTime::now().as_secs()`，在 `render()` 入口处固定，resize 重绘时种子不变（同一次运行背景图案稳定）。

### 动画层（scene::animate）

`scene::generate()` 先生成静态纹理；`scene::animate()` 再按 frame 与天气类型做动态变换：

- 晴天/多云：水平漂移；夜间额外星点闪烁
- 阴天/雾：慢速层流
- 雨/雷暴：纵向滚动 + 斜向偏移，模拟降水
- 雪：缓慢下落 + 轻微横向漂移

### 各天气背景策略

| 天气 | 字符集 | 颜色令牌 | 密度策略 |
| ------ | -------- | ---------- | --------- |
| 晴天 | `.` `v`（飞鸟） | `dim_color` | 稀疏散点，鸟只在上1/4区域 |
| 夜晚 | `+` `*` `.` | `ArtColor::Star` | 亮度分级（+>*>.） |
| 多云 | `#` `:` `.` | `ArtColor::CloudLight` | 上半部更密 |
| 阴天 | `#` `:` | `ArtColor::CloudDark` | 顶部最密，向下渐稀 |
| 雾 | `~` `-` | `ArtColor::FogMist` | 奇偶行交替波纹 |
| 小雨 | `/` | `ArtColor::RainDrop` | 每9列1条斜线，行偏移 |
| 大雨 | `/` `\|` | `ArtColor::RainDrop` | 每4列2字符 |
| 雷暴 | `/` `\|` `!` | `ArtColor::RainDrop` | 大雨 + 随机1-3个`!`落雷点 |
| 小雪 | `+` `.` | `ArtColor::SnowFlake` | 稀疏雪花 |
| 大雪 | `*` `+` `.` | `ArtColor::SnowFlake` | 密集雪花，带大晶体`*` |

背景字符串保证全为 ASCII（单字节），确保按字符索引切片与按列宽索引一致。

---

## 主题系统

### Theme Trait

```rust
pub trait Theme {
    fn art_color(&self, c: ArtColor) -> Color;
    fn title_color(&self) -> Color;
    fn temp_color(&self) -> Color;
    fn info_color(&self) -> Color;
    fn dim_color(&self) -> Color;
    fn border_color(&self) -> Color;
    fn highlight_color(&self) -> Color;   // 高温警示色
    fn cold_color(&self) -> Color;        // 低温警示色
    fn chart_color(&self, normalized: f64) -> Color;
}
```

所有颜色返回 `crossterm::style::Color`，通常为 `Color::AnsiValue(n)`（256 色）。

### ArtColor 语义令牌

```rust
pub enum ArtColor {
    SunCore,    // 太阳芯体   → 220 (bright gold)
    SunRay,     // 太阳光线   → 228 (light gold)
    CloudLight, // 浅云       → 250 (light grey)
    CloudDark,  // 雨云       → 245 (mid grey)
    RainDrop,   // 雨         → 111 (light blue)
    SnowFlake,  // 雪         → 255 (bright white)
    Lightning,  // 闪电       → 226 (bright yellow)
    FogMist,    // 雾         → 249 (fog grey)
    MoonBody,   // 月亮       → 230 (warm white)
    Star,       // 星星       → 228 (warm yellow)
    Ground,     // 地面（保留）→ 130 (earth brown)
}
```

---

## 24h 温度趋势图

### 标准版（card / compact）

`chart::render_chart()` — 固定8列，每列4字符宽。

### 宽版（fullscreen）

`chart::render_chart_wide()` — 按传入 `width` 均匀分布8列：

```text
每列宽度 col_w = (width - 2) / 8
标签/温度/方块 在列内居中：left_pad = (col_w - label.len()) / 2
```

### 方块字符集

```rust
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
// U+2581 ~ U+2588
```

高度归一化算法：

```text
normalized = (temp - min) / (max - min).max(1.0)
block_idx  = (normalized * 7.0) as usize
```

---

## 单位换算

```rust
// 温度（摄氏 → 华氏）
°F = °C × 9/5 + 32

// 风速（km/h → mph）
mph = km/h × 0.621371

// 降水（mm → 英寸）
in = mm × 0.0393701
```

---

## CLI 入口流程

```text
main()
  1. Cli::command().get_matches()         // clap 解析参数来源
     + Cli::from_arg_matches()
  2. config::load()                        // 加载配置文件
  3. 解析 units（CLI > config）
  4. 解析 mode（CLI > config）
  5. 解析 show_chart / theme / show_forecast
  6. 位置解析（CLI lat/lon > CLI city > config coords > config city > IP）
  7. fetch_weather_with_retry(loc)         // 最多重试 5 次
  8. 构建 RenderContext
  9. match mode { ... }                    // 分发渲染器
 10. 错误 → eprintln! + exit(1)
```

---

## 错误处理策略

- 天气请求失败会自动重试（最多 5 次，间隔 500ms），最终失败时 `eprintln!` + `process::exit(1)`
- 配置文件解析失败 → 静默降级为默认配置
- Geocoding 无结果 → 返回 `Err`，main 打印错误退出
- 终端过窄（< 52 列）→ fullscreen 模式打印提示后正常退出
- 渲染错误（IO 错误）→ main 捕获并 `eprintln!` 后退出

---

## 构建与开发

```bash
cargo build                    # debug 构建
cargo build --release          # release 构建（优化）
cargo run                      # 运行（自动定位）
cargo run -- tokyo             # 查东京
cargo run -- --mode fullscreen tokyo --forecast
cargo clippy                   # lint（保持零警告）
cargo fmt                      # 格式化
cargo install --path .         # 安装到 ~/.cargo/bin/
```

### 网络代理

如需代理，需在系统或 ureq 层面配置。Git 代理示例：

```bash
git config --global http.proxy http://127.0.0.1:10808
```

---

## 扩展点

### 新增天气渲染模式

1. 在 `src/render/` 创建新文件（如 `minimal.rs`）
2. 在 `render/mod.rs` 添加 `pub mod minimal;`
3. 在 `main.rs` 的 `Mode` 枚举添加变体，在 `match` 中添加分发

### 新增主题

1. 在 `src/theme/` 创建新文件，实现 `Theme` trait
2. 在 `theme/mod.rs` 的 `resolve()` 函数添加匹配分支

### 新增天气状态

1. 在 `model.rs` 的 `WeatherCondition` 添加枚举值
2. 在 `WeatherCondition::from_code()` 添加 WMO 代码映射
3. 在 `WeatherCondition::description()` 和 `icon()` 添加对应文本
4. 在 `art/pieces.rs` 实现对应 ASCII art 函数
5. 在 `art/mod.rs` 的 `get_art()` 添加分发
6. 在 `scene.rs` 的 `generate()` 添加背景图案分发
