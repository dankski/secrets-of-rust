use anyhow::Result;
use clap::Parser;

use weather::Weatherstack;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    /// Weatherstack API key
    #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
    api_key: String,

    /// Report temperature in Fahrenheit
    #[arg(required = false)]
    #[arg(short, long)]
    fahrenheit: bool,

    /// Example: "London,UK"
    #[arg(required = true)]
    location: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let location = args.location.join(" ");
    let ws = Weatherstack::new(&args.api_key);
    let weather = ws.get_weather(&location)?;

    println!(
        "{} {}",
        weather.summary,
        if args.fahrenheit {
          format!("{:.1}ºF", weather.temperature.as_fahrenheit())
        } else {
          format!("{:.1}ºC", weather.temperature.as_celsius())
        }
    );
    Ok(())
}
