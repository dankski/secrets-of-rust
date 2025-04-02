use anyhow::{Context, Result, bail};
use clap::{arg, Parser};

use std::fmt::Display;

use reqwest::blocking::RequestBuilder;
use serde_json::Value;

#[derive(Parser)]
struct Args {
  #[arg(required = true)]
  locaction: Vec<String>,

  #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
  api_key: String,
}

#[derive(Debug, PartialEq)]
pub struct Weather {
  temperature: f64,
  summary: String,
}

impl Display for Weather {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {:.1}C", self.summary, self.temperature)
  }
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
  
  if location.is_empty() {
    bail!("Usage: weather <LOCATION>");
  }

  let resp = request(location, api_key).send()?;
  let weather = deserialize(&resp.text()?)?;
  Ok(weather)
}

fn request(location: &str, api_key: &str) -> RequestBuilder {
  reqwest::blocking::Client::new()
    .get("https://api.weatherstack.com/current")
    .query(&[("query", location), ("access_key", api_key)])
}

fn deserialize(json: &str) -> Result<Weather> {
  let val: Value = serde_json::from_str(json)?;
  
  let temperature = val
      .pointer("/current/temperature")
      .and_then(Value::as_f64)
      .with_context(|| format!("bad response: {val}"))?;
  
  let summary = val
      .pointer("/current/weather_descriptions/0")
      .and_then(Value::as_str)
      .with_context(|| format!("bad response: {val}"))?
      .to_string();
  Ok(Weather {
    temperature,
    summary
  })
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::env;

  use super::*;

  use url::Host::Domain;

  #[test]
  fn get_weather_fn_returns_correct_weather_for_location() {
    let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
    let weather = get_weather("Zurich,ZH", &api_key).unwrap();
    
    assert_eq!(
      weather,
      Weather {
        temperature: 11.2,
        summary: "Sunny".into(),
      },
      "wrong weather"
    );
  }

  #[test]
  fn request_builds_correct_request() {
    let req = request("Zurich,ZH", "dummy API key");
    let req = req.build().unwrap();

    assert_eq!(req.method(), "GET", "Wrong Method");

    let url = req.url();
    assert_eq!(url.host(),
              Some(Domain("api.weatherstack.com")),
              "Wrong host"
    );
    
    assert_eq!(url.path(), "/current", "Wrong path");

    let params: Vec<(_,_)> = url.query_pairs().collect();
    assert_eq!(
      params,
      vec![
          ("query".into(), "Zurich,ZH".into()),
          ("access_key".into(), "dummy API key".into())
      ],
      "Wrong, params"
    );
  }

  #[test]
  fn deserialize_extracts_correct_weather_from_json() {
    let json = fs::read_to_string("tests/data/ws.json").unwrap();
    let weather = deserialize(&json).unwrap();

    assert_eq!(
      weather,
      Weather {
        temperature: 11.2,
        summary: "Sunny".into(),
      },
      "Wrong weather"
    );
  }
}
