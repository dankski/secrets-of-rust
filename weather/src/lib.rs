use anyhow::{Context, Result, bail};
use clap::{arg, Parser};

use std::fmt::Display;

use reqwest::blocking::RequestBuilder;
use serde_json::Value;


#[derive(Debug, PartialEq)]
pub struct Weather {
  pub temperature: f64,
  summary: String,
}

impl Weather {
  #[must_use]
  pub fn into_fahrenheit(mut self) -> Self {
    self.temperature = self.temperature * 1.8 + 32.0;
    self
  }
}

pub struct Weatherstack {
  pub base_url: String,
  api_key: String,
}

impl Weatherstack {

  #[must_use]
  pub fn new(api_key: &str) -> Self {
    Self {
      base_url: "https://api.weatherstack.com/current".into(),
      api_key: api_key.to_owned(),
    }
  }

  pub fn get_weather(&self, location: &str) -> Result<Weather> {
    let resp = request(&self.base_url, location, &self.api_key).send()?;
    let weather = deserialize(&resp.text()?)?;
    Ok(weather)
  }

}

impl Display for Weather {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {:.1}C", self.summary, self.temperature)
  }
}


fn request(base_url: &str, location: &str, api_key: &str) -> RequestBuilder {
  reqwest::blocking::Client::new()
    .get(base_url)
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

  use super::*;

  use url::Host::Domain;

  use httpmock::{Method, MockServer};
  use reqwest::StatusCode;


  #[test]
  fn mock_server_responds_with_hello() {
    let server = MockServer::start();
    server.mock(|when, then| {
      when.method(Method::GET);
      then.status(StatusCode::OK.into()).body("Hello");
    });

    let resp = reqwest::blocking::Client::new()
        .get(server.base_url())
        .send()
        .unwrap();

     assert_eq!(resp.status(), StatusCode::OK, "wrong status"); 
     assert_eq!(resp.text().unwrap(), "Hello", "wrong message");
  }

  #[test]
  fn get_weather_fn_makes_correct_api_call() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {

      when.method(Method::GET)
          .path("/current")
          .query_param("query", "Zurich,ZH")
          .query_param("access_key", "dummy api key");

      then.status(StatusCode::OK.into())
          .header("content-type", "application/json")
          .body_from_file("tests/data/ws.json");
    });

    let mut ws = Weatherstack::new("dummy api key");
    ws.base_url = server.base_url() + "/current";
    
    let weather = ws.get_weather("Zurich,ZH");
    mock.assert();

    assert_eq!(
      weather.unwrap(),
      Weather {
        temperature: 11.2,
        summary: "Sunny".into(),
      },
      "wrong weather"
    );
  }

  #[test]
  fn request_builds_correct_request() {
    let req = request(
      "https://example.com/current", 
      "Zurich,ZH", 
      "dummy API key"
    );
    let req = req.build().unwrap();

    assert_eq!(req.method(), "GET", "Wrong Method");

    let url = req.url();
    assert_eq!(url.host(),
              Some(Domain("example.com")),
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

  #[test]
  fn into_fahrenheit_fn_correctly_converts_temparture() {
    let weather = Weather {
      temperature: 10.0,
      summary: "Partly cloudy".into(),
    };

    assert_eq!(weather.into_fahrenheit(), Weather {
      temperature: 50.0,
      summary: "Partly cloudy".into(),
      "wrong weather"
    })
  }
}
