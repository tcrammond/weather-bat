use chrono::prelude::*;
use reqwest::header::USER_AGENT;

mod codes;

fn main() {
    let api_token = std::env::var("WEATHER_KEY")
        .expect("An OpenWeather API key is required. See https://openweathermap.org/.");
    let lat = std::env::var("LAT").expect("A latitude is required.");
    let lon = std::env::var("LON").expect("A longitude is required.");

    // let mut arg_iterator = std::env::args();
    // arg_iterator.next();
    // let args: String = arg_iterator.collect();

    let client = reqwest::blocking::Client::new();
    let url = "https://api.openweathermap.org/data/2.5/onecall";

    let response = client
        .get(url)
        .header(USER_AGENT, "tc-weather-bat")
        .query(&[("appid", api_token), ("lat", lat), ("lon", lon)])
        .send();

    if let Err(_error) = response {
        println!("🦇  Couldn't reach the weather service. Try again later?");
        return;
    }

    let response = response.unwrap().json::<serde_json::Value>();
    if let Err(_error) = response {
        println!("🦇  Couldn't figure out what the weather service said. Try again later?");
        return;
    }

    let response = response.unwrap();

    let sunset = response["current"]["sunset"].as_i64();
    let temp = response["current"]["temp"].as_f64();
    let humidity = response["current"]["humidity"].as_f64();
    let weather = response["current"]["weather"][0]["description"].as_str();
    let weather_code = response["current"]["weather"][0]["id"].as_i64();
    // let rain1h = response["current"]["rain"]["1h"].as_f64();

    let now: DateTime<Local> = Local::now();

    println!("🦇 Good day, delicious friend! ");
    println!("It is {}", now.format("%a %b %e, %T").to_string());

    if let Some(temp) = temp {
        println!("The temperature outside is {:.2}°C", to_c(temp));
    }

    if let Some(weather) = weather {
        if let Some(weather_code) = weather_code {
            let emoji = match codes::WEATHER_CONDITIONS
                .iter()
                .find(|t| t.0 == weather_code)
            {
                Some(code) => code.1,
                None => '🦇',
            };
            println!("We have {} {}", weather, emoji);
        } else {
            println!("We have {}", weather);
        }
    }

    if let Some(humidity) = humidity {
        println!("Humidity is at {:.2}%", humidity);
    }

    if let Some(sunset) = sunset {
        println!(
            "You can catch the sunset at {}",
            Utc.timestamp(sunset, 0).format("%T")
        );
    }
}

fn to_c(k: f64) -> f64 {
    k - 273.15
}
