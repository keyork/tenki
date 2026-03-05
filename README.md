# tenki

> 天気（てんき）: 有调性的终端天气

- [ ] 城市解析仍有一些小问题，下一步尝试更换为其他解析服务。

tenki 是一个 Rust 编写的终端天气工具。它在命令行里提供较完整的天气信息、ASCII art 和多种显示模式（card / compact / oneline / fullscreen / showcase）。

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Beijing, China                                                   ☀  Clear Sky
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🌡 5°C  feels 1°C (cold)          💨 12 km/h  NW ↘          💧 [███░░░] 51%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  . .  v  .    .  .    .    .  .      .   .    .    .   .    .    .   .    .    .
    .      .  .      .   .    .  .  .    .  .    .   .  .    .  .   .      .   .
               ·  ·          ·  ·     ·  ·     ·        ·       ·       ·
                        [ art centered ]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  24h    00    03    06    09    12    15    18    21
          5°    4°    4°    7°   12°   13°    9°    6°
         ▁▁    ▁▁    ▁▁    ▃▃    ▆▆    ██    ▄▄    ▂▂
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 安装

### 一行安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/keyork/tenki/main/install.sh | sh
```

安装脚本会优先下载预编译二进制；若对应版本/平台资源暂不可用（如 404），会自动回退为源码编译安装（需本机有 `git` 和 `cargo`）。

可选: 安装指定版本

```bash
curl -fsSL https://raw.githubusercontent.com/keyork/tenki/main/install.sh | sh -s -- --version v0.1.0
```

### 从源码安装（开发用）

```bash
git clone https://github.com/keyork/tenki
cd tenki
cargo install --path .
```

---

## 快速开始

```bash
# 自动定位（基于 IP，不需要城市 API Key）
tenki

# 指定坐标
tenki --lat 39.9042 --lon 116.4074

# 指定城市
tenki 北京
tenki "new york"

# 全屏模式（按 Q 退出）
tenki --mode fullscreen tokyo

# 展示模式（全屏显示 5 秒后自动退出）
tenki --mode showcase tokyo

# 其他示例
tenki tokyo --forecast
tenki london --units imperial
```

---

## 城市解析配置（Open-Meteo）

城市名解析使用 Open-Meteo Geocoding API，不需要 API Key。

地区显示规则: 台湾/香港/澳门的国家归属统一显示为 `中国`。

---

## 显示模式

| 模式 | 命令 | 描述 |
| --- | --- | --- |
| `card` | `tenki` | 默认。带边框的信息卡片 + ASCII art |
| `compact` | `tenki --mode compact` | 无边框，紧凑排版 |
| `oneline` | `tenki --mode oneline` | 单行，适合状态栏或脚本 |
| `fullscreen` | `tenki --mode fullscreen` | 全屏动态模式（背景与 ASCII 图轻动画），按 `Q` 退出 |
| `showcase` | `tenki --mode showcase` | 全屏动态展示，5 秒后自动退出 |

---

## CLI 选项

```text
tenki [城市名] [选项]

参数：
  [城市名]              要查询的城市（省略则自动定位）

选项：
  -m, --mode <MODE>     显示模式 [默认: card] [可选: card, compact, oneline, fullscreen, showcase]
  -u, --units <UNITS>   单位制 [默认: metric] [可选: metric, imperial]
  -t, --theme <THEME>   配色主题 [默认: default] [可选: default, light, mono]
  -f, --forecast        显示 3 天预报
      --no-chart        隐藏 24h 温度趋势图
      --static          关闭 fullscreen/showcase 动画（静态场景）
      --lat <LAT>       纬度（覆盖城市/自动定位）
      --lon <LON>       经度（覆盖城市/自动定位）
  -h, --help            显示帮助
  -V, --version         显示版本
```

---

## 配置文件

路径: `~/.config/tenki/config.toml`

```toml
[location]
city = "Tokyo"
# 或直接指定坐标（优先级更高）
# latitude = 35.6762
# longitude = 139.6503

[display]
units = "metric"      # metric | imperial
mode = "card"         # card | compact | oneline | fullscreen | showcase
theme = "default"     # default | light | mono
show_chart = true
```

优先级: **CLI 参数 > 配置文件 > 默认值**

---

## 配色主题

| 主题 | 适用场景 |
| --- | --- |
| `default` | 深色终端（推荐） |
| `light` | 浅色终端 |
| `mono` | 单色/低色彩终端 |

---

## 数据来源

- 天气数据: [Open-Meteo](https://open-meteo.com)
- 城市解析: [Open-Meteo Geocoding API](https://open-meteo.com/en/docs/geocoding-api)
- IP 定位: [ip-api.com](https://ip-api.com)

---

## 技术栈

- Rust 2021 (MSRV 1.75)
- clap
- ureq
- serde / serde_json / toml
- crossterm

---

## License

MIT
