use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    city: String,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherInfo>,
    main: MainInfo,
    name: String,
}

#[derive(Debug, Deserialize)]
struct WeatherInfo {
    description: String,
}

#[derive(Debug, Deserialize)]
struct MainInfo {
    temp: f32,
    humidity: u32,
}

fn main() {
    let args = Cli::parse();
    let api_key = match env::var("OPENWEATHER_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: OPENWEATHER_API_KEY environment variable not set.");
            std::process::exit(1);
        }
    };

    match get_weather(&args.city, &api_key) {
        Ok(weather) => {
            println!(
                "City: {}\nWeather: {}\nTemperature: {:.2}°C\nHumidity: {}%",
                weather.name,
                weather.weather.get(0).map_or("N/A", |w| &w.description),
                kelvin_to_celsius(weather.main.temp),
                weather.main.humidity
            );
        }
        Err(err) => {
            eprintln!("Failed to get weather data: {}", err);
            std::process::exit(1);
        }
    }
}

fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key // Use the `api_key` parameter here
    );

    let client = Client::new();
    let response = client.get(&url).send()?.error_for_status()?;

    let weather_data = response.json::<WeatherResponse>()?;
    Ok(weather_data)
}

fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}
