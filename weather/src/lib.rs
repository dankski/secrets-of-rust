use anyhow::Result;

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Weather {
  temperature: f64,
  summary: String,
}

impl Display for Weather {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
  let resp = request(location, api_key).send()?;
  let weather = deserialize(&resp.text()?)?;
  Ok(weather)
}

fn request(location: &str, api_key: &str) -> RequestBuilder {
  reqwest::blocking::Client::new()
    .get("https://api.weatherstack.com/current")
    .query(&[("query", location), ("access_key", api_key)])
}

#[cfg(test)]
mod tests {
  use std::env;

  use super::*;

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
    let req = req.build().unwra();

    assert_eq!(req.method(), "GET", "Wrong Method"):

    let url = req.url();
    assert_eq!(url.host(),
              Some(Domain("api.weatherstack.com"),
              "Wrong host"
    );
    
    assert_eq!(url.path(), "/current", "Wrong path");

    let params: Vec<(_,_)> = url.query_pairs().collect();
    assert_eq!(
      params,
      vec![
          ("query".into(), "ZUrich,ZH".into()),
          ("access_key".into(), "dummy API key".into())
      ],
      "Wrong, params"
    );
  },
}
