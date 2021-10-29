# weather-bat

A delightful little command to display your local weather, as retrieved from [OpenWeather](https://openweathermap.org/).

![Example terminal output from weather-bat](https://github.com/tcrammond/weather-bat/raw/main/example.png)

## Installation

Requirements: [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

You'll need to build the binary first:

```shell
git clone git@github.com:tcrammond/weather-bat.git
cd weather-bat
cargo build --release -p weather-bat
```

Then install to your `$PATH`:

```shell
cargo install --path cli
```

_This places the binary at `~/.cargo/bin/weather-bat`. Ensure `~/.cargo/bin` is in your `$PATH` so that you can run the binary from any directory._

## Usage

You'll need the latitude and longitude of your location.

```shell
LAT=51.178882 LON=-1.8284037 weather-bat
```

Output:

```shell
                          🦇 Good day, delicious friend!
 --------------------------------------------------------------------------------
 🌡️  The temperature outside is 12.90°C  |           🕐 It is Fri Oct 29, 13:31:22
 🌧  We have moderate rain               |                🌇 Sunset is at 16:47:05
 --------------------------------------------------------------------------------
```
