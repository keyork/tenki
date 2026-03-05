# tenki

终端天气工具（Rust）。支持城市/坐标/IP 查询，内置 ASCII 图、动态全屏与多主题。

- [ ] 已知问题：openmeteo的城市解析不太对，后续会调整成其他更好的城市解析工具。（主要是高德地图和百度地图都要API_KEY，搞个工具还要去配API_KEY未免有点太麻烦了）

## 安装

推荐一行安装：

```bash
curl -fsSL https://raw.githubusercontent.com/keyork/tenki/main/install.sh | sh
```

安装脚本行为：

- 优先下载 GitHub Release 预编译二进制
- 若对应资源不存在（如 404），自动回退源码编译安装（需要 `git` + `cargo`）

指定版本：

```bash
curl -fsSL https://raw.githubusercontent.com/keyork/tenki/main/install.sh | sh -s -- --version v0.1.0
```

源码安装：

```bash
git clone https://github.com/keyork/tenki
cd tenki
cargo install --path .
```

## 快速使用

```bash
# 自动定位（IP）
tenki

# 指定城市
tenki 北京
tenki "new york"

# 指定坐标
tenki --lat 39.9042 --lon 116.4074

# 全屏动态（Q/Esc/Ctrl+C 退出）
tenki --mode fullscreen tokyo

# 全屏展示 5 秒后退出
tenki --mode showcase tokyo

# 关闭动画（静态）
tenki --mode fullscreen --static tokyo
```

## 显示模式

- `card`：默认卡片布局
- `compact`：紧凑布局
- `oneline`：单行输出（适合脚本/状态栏）
- `fullscreen`：全屏模式（支持动态背景）
- `showcase`：全屏展示，5 秒自动退出

## 主题

可用主题：

- `default`
- `light`
- `mono`
- `ocean`
- `forest`
- `sunset`

示例：

```bash
tenki --theme ocean tokyo
```

## 常用参数

```text
tenki [CITY] [OPTIONS]

  -m, --mode <MODE>       card | compact | oneline | fullscreen | showcase
  -u, --units <UNITS>     metric | imperial
  -t, --theme <THEME>     default | light | mono | ocean | forest | sunset
  -f, --forecast          显示 3 天预报
      --no-chart          隐藏 24h 温度趋势图
      --static            关闭 fullscreen/showcase 动画
      --lat <LAT>         纬度
      --lon <LON>         经度
  -h, --help
  -V, --version
```

## 配置文件

路径：`~/.config/tenki/config.toml`

```toml
[location]
city = "Tokyo"
# latitude = 35.6762
# longitude = 139.6503

[display]
units = "metric"      # metric | imperial
mode = "card"         # card | compact | oneline | fullscreen | showcase
theme = "default"     # default | light | mono | ocean | forest | sunset
show_chart = true
```

优先级：`CLI 参数 > 配置文件 > 默认值`

## 地名解析与数据源

- 天气数据：Open-Meteo
- 城市解析：Open-Meteo Geocoding API（无需 API Key）
- IP 定位：ip-api.com

地区显示规则：台湾/香港/澳门统一显示为 `中国`。

## License

Apache License 2.0
