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
    #[arg(required = true)]
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
        "{}",
        if args.fahrenheit {
            weather.into_fahrenheit()
        } else {
            weather
        }
    );
    Ok(())
}
