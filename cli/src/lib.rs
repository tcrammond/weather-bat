use chrono::prelude::*;
use std::{error::Error, fmt::Display};

mod codes;

pub struct WeatherResult {
    pub greeting: String,
    pub temp: String,
    pub time: String,
    pub rain: Option<String>,
    pub sunset: String,
    pub weather: String,
}

#[derive(Debug)]
pub enum WeatherBatError {
    Network,
    Unknown,
}

impl Error for WeatherBatError {}
impl Display for WeatherBatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            WeatherBatError::Network => "Couldn't find the weather service. Try again later?",
            _ => "Couldn't figure out what the weather service said. Try again later?",
        };
        write!(f, "{}", text)
    }
}

pub fn summon_the_weather_bat(lat: String, lon: String) -> Result<WeatherResult, WeatherBatError> {
    let api_url = if cfg!(debug_assertions) {
        "http://localhost:8888/.netlify/functions/get-weather"
    } else {
        "https://the-weather-bat.netlify.app/.netlify/functions/get-weather"
    };

    let response = reqwest::blocking::Client::new()
        .get(api_url)
        .query(&[("lat", lat), ("lon", lon)])
        .send();

    if let Err(_error) = response {
        return Err(WeatherBatError::Network);
    }

    let response = response.unwrap().json::<serde_json::Value>();
    if let Err(_error) = response {
        return Err(WeatherBatError::Unknown);
    }

    let response = response.unwrap();
    let sunset = response["current"]["sunset"].as_i64();
    let temp = response["current"]["temp"].as_f64();
    let weather = response["current"]["weather"][0]["description"].as_str();
    let weather_code = response["current"]["weather"][0]["id"].as_i64();
    let rain_expected = if let Some(hourly) = response["hourly"].as_array() {
        // TODO: Haven't seen multiple weather entries but should iterate over it anyway.
        let next = hourly[1]["weather"][0]["description"].as_str().unwrap();
        next.contains("rain")
    } else {
        false
    };

    let rain = if rain_expected {
        Some(String::from(
            " ☔ Grab a splendid umbrella, you can expect rain in the next hour.",
        ))
    } else {
        None
    };

    let weather = if let Some(weather) = weather {
        if let Some(weather_code) = weather_code {
            let emoji = match codes::WEATHER_CONDITIONS
                .iter()
                .find(|t| t.0 == weather_code)
            {
                Some(code) => code.1,
                None => '🦇',
            };

            format!("{} We have {} ", emoji, weather)
        } else {
            format!("{} We have {}", '🦇', weather)
        }
    } else {
        String::from("I don't know what the weather is")
    };

    return Ok(WeatherResult {
        greeting: String::from("🦇 Good day, delicious friend!"),

        rain,
        sunset: format!(
            "🌇 Sunset is at {}",
            Utc.timestamp(sunset.unwrap(), 0)
                .with_timezone(&Local)
                .format("%T")
        ),
        temp: format!(
            "🌡️  The temperature outside is {:.2}°C ",
            to_c(temp.unwrap())
        ),
        time: format!(
            "🕐 It is {}",
            Local::now().format("%a %b %e, %T").to_string()
        ),
        weather,
    });
}

fn to_c(k: f64) -> f64 {
    k - 273.15
}
