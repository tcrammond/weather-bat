use chrono::prelude::*;
use colored::Colorize;

mod codes;

fn main() {
    let api_url = if cfg!(debug_assertions) {
        "http://localhost:8888/.netlify/functions/get-weather"
    } else {
        "https://the-weather-bat.netlify.app/.netlify/functions/get-weather"
    };
    let lat = std::env::var("LAT").expect("A latitude is required.");
    let lon = std::env::var("LON").expect("A longitude is required.");

    let response = reqwest::blocking::Client::new()
        .get(api_url)
        .query(&[("lat", lat), ("lon", lon)])
        .send();

    if let Err(_error) = response {
        println!(
            "{}",
            "🦇  Couldn't reach the weather service. Try again later?".red()
        );
        return;
    }

    let response = response.unwrap().json::<serde_json::Value>();
    if let Err(_error) = response {
        println!(
            "{}",
            "🦇  Couldn't figure out what the weather service said. Try again later?".red()
        );
        return;
    }

    let response = response.unwrap();

    let sunset = response["current"]["sunset"].as_i64();
    let temp = response["current"]["temp"].as_f64();
    let _humidity = response["current"]["humidity"].as_f64();
    let weather = response["current"]["weather"][0]["description"].as_str();
    let weather_code = response["current"]["weather"][0]["id"].as_i64();
    let rain_expected = if let Some(hourly) = response["hourly"].as_array() {
        // TODO: Haven't seen multiple weather entries but should iterate over it anyway.
        let next = hourly[1]["weather"][0]["description"].as_str().unwrap();
        next.contains("rain")
    } else {
        false
    };

    let now: DateTime<Local> = Local::now();

    print_heading();
    print_left_and_right(
        format!(
            "🌡️  The temperature outside is {:.2}°C ",
            to_c(temp.unwrap())
        ),
        format!("🕐 It is {}", now.format("%a %b %e, %T").to_string()),
    );

    if let Some(weather) = weather {
        if let Some(weather_code) = weather_code {
            let emoji = match codes::WEATHER_CONDITIONS
                .iter()
                .find(|t| t.0 == weather_code)
            {
                Some(code) => code.1,
                None => '🦇',
            };
            print_left_and_right(
                format!("{} We have {} ", emoji, weather),
                format!(
                    "🌇 Sunset is at {}",
                    Utc.timestamp(sunset.unwrap(), 0).with_timezone(&Local).format("%T")
                ),
            )
        } else {
            print_left_and_right(
                format!("{} We have {}", '🦇', weather),
                format!(
                    "🌇 Sunset is at {}",
                    Utc.timestamp(sunset.unwrap(), 0).with_timezone(&Local).format("%T")
                ),
            )
        }
    }

    if rain_expected {
        print_divider();
        println!(
            "{:<81}",
            " ☔ Grab a splendid umbrella, you can expect rain in the next hour."
                .black()
                .on_white()
        );
    }

    print_footer();

    // if let Some(humidity) = humidity {
    //     println!("💦 Humidity is at {:.2}%", humidity);
    // }
}

fn to_c(k: f64) -> f64 {
    k - 273.15
}

fn print_divider() {
    println!(
        "{:^82}",
        "--------------------------------------------------------------------------------"
            .black()
            .on_white()
    );
}

fn print_heading() {
    println!("");
    println!(
        "{:^81}",
        "🦇 Good day, delicious friend!".black().on_white()
    );
    print_divider()
}

fn print_footer() {
    print_divider();
    println!("");
}

fn print_left_and_right(left: String, right: String) {
    println!(
        "{}",
        format!(" {:<37} | {:>38} ", left, right).black().on_white()
    );
}
