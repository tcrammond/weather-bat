use chrono::prelude::*;
use colored::Colorize;

mod codes;

fn main() {
    let lat = std::env::var("LAT").expect("A latitude is required.");
    let lon = std::env::var("LON").expect("A longitude is required.");

    let client = reqwest::blocking::Client::new();
    let url = "http://localhost:8888/.netlify/functions/get-weather";

    let response = client
        .get(url)
        .query(&[("lat", lat), ("lon", lon)])
        .send();

    if let Err(_error) = response {
        println!("{}", "🦇  Couldn't reach the weather service. Try again later?".red());
        return;
    }

    let response = response.unwrap().json::<serde_json::Value>();
    if let Err(_error) = response {
        println!("{}", "🦇  Couldn't figure out what the weather service said. Try again later?".red());
        return;
    }

    let response = response.unwrap();

    let sunset = response["current"]["sunset"].as_i64();
    let temp = response["current"]["temp"].as_f64();
    let _humidity = response["current"]["humidity"].as_f64();
    let weather = response["current"]["weather"][0]["description"].as_str();
    let weather_code = response["current"]["weather"][0]["id"].as_i64();
    let rain1h = response["current"]["rain"]["1h"].as_f64();

    let now: DateTime<Local> = Local::now();

    println!("");
    println!("{:^81}", "🦇 Good day, delicious friend!".black().on_white());
    println!("{:^82}", "--------------------------------------------------------------------------------".black().on_white());
    println!("{}", print_left_and_right(&format!("🌡️  The temperature outside is {:.2}°C ", to_c(temp.unwrap())), format!("🕐 It is {}", now.format("%a %b %e, %T").to_string()).as_str()).black().on_white());


    if let Some(weather) = weather {
        if let Some(weather_code) = weather_code {
            let emoji = match codes::WEATHER_CONDITIONS
                .iter()
                .find(|t| t.0 == weather_code)
            {
                Some(code) => code.1,
                None => '🦇',
            };
            println!("{}", print_left_and_right(&format!("{} We have {}", emoji, weather), &format!("🌇 Sunset is at {}",
            Utc.timestamp(sunset.unwrap(), 0).format("%T"))).black().on_white());
        } else {
            println!("{}", print_left_and_right(&format!("{} We have {}", '🦇', weather), &format!("🌇 Sunset is at {}",
            Utc.timestamp(sunset.unwrap(), 0).format("%T"))).black().on_white());
        }
    }

    if let Some(rain1h) = rain1h {
        println!("{:^81}", format!("☔ Grab a splendid umbrella, there's a {} chance of rain in the next hour", rain1h).black().on_white());
    }

    println!("{:^82}", "--------------------------------------------------------------------------------".black().on_white());
    println!("");

    // if let Some(humidity) = humidity {
    //     println!("💦 Humidity is at {:.2}%", humidity);
    // }
}

fn to_c(k: f64) -> f64 {
    k - 273.15
}

fn print_left_and_right (left: &str, right: &str) -> String {
    format!(" {:<37} | {:>38} ", left, right)
}
