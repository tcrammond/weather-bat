use colored::Colorize;
use weather_bat::WeatherBatError;

mod codes;

#[tokio::main]
async fn main() -> Result<(), WeatherBatError> {
    let lat = std::env::var("LAT").expect("A latitude is required.");
    let lon = std::env::var("LON").expect("A longitude is required.");

    let weather_result = weather_bat::summon_the_weather_bat(lat, lon).await;

    if let Err(err) = weather_result {
        println!("{}", err);
        return Err(err);
    }

    let weather = weather_result.unwrap();

    print_heading(weather.greeting);
    print_left_and_right(weather.temp, weather.time);
    print_left_and_right(weather.weather, weather.sunset);

    if let Some(rain) = weather.rain {
        print_divider();
        println!("{:<81}", rain.black().on_white());
    }

    print_footer();
    Ok(())
}

fn print_divider() {
    println!(
        "{:^82}",
        "--------------------------------------------------------------------------------"
            .black()
            .on_white()
    );
}

fn print_heading(greeting: String) {
    println!("");
    println!("{:^81}", greeting.black().on_white());
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
