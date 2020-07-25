use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use eyre::WrapErr;
use reqwest::blocking::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use structopt::StructOpt;
use thiserror::Error;

mod openweather;
use openweather::*;

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let config_json =
        File::open(&opt.config).wrap_err_with(|| {
            format!(
                "Failed to open config file {:?}",
                opt.config
            )
        })?;
    let config: OpenWeather = serde_json::from_reader(
        &config_json,
    )
    .wrap_err("Failed to deserialize configuration JSON")?;
    let onecall: OneCall = config
        .onecall()
        .wrap_err("Failed to deserialize hourly weather data")?;
    // println!("OneCall: {:#?}", onecall);
    let historical = config
        .historical_day(Utc::today().and_hms(0, 0, 0) - Duration::days(1))
        .wrap_err("Failed to deserialize historical hourly weather data")?;

    let yesterday =
        average(historical.iter().map(|h| h.feels_like));
    println!("Yesterday felt like: {}", yesterday);
    let today =
        average(onecall.hourly.iter().map(|h| h.feels_like));
    println!("Today should feel like: {}", today);
    let diff = TempDifference::from(yesterday, today);
    println!(
        "Today will feel {} {} yesterday",
        diff,
        match diff {
            TempDifference::Same => "as",
            _ => "than",
        }
    );
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct OpenWeather {
    api_key: String,

    lat: f64,
    lon: f64,

    #[serde(skip)]
    client: Client,
}

impl OpenWeather {
    fn get<Response: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<Response, WeatherError> {
        let bytes = self
            .client
            .get(&format!(
                "https://api.openweathermap.org/data/2.5/{}",
                endpoint
            ))
            .query(params)
            .query(&[
                ("lat", &format!("{}", self.lat)),
                ("lon", &format!("{}", self.lon)),
                ("appid", &self.api_key),
            ])
            .send()?
            .bytes()?;
        // Attempt to deserialize as `Response`
        serde_json::from_reader(&*bytes).map_err(|err| {
            // If we fail, attempt to deserialize as `ClientError`
            (&*bytes)
                .try_into()
                // If we don't have a `ClientError`, fail with the original error.
                .unwrap_or_else(|_| {
                    WeatherError::Deserialize(
                        err,
                        String::from_utf8_lossy(&*bytes)
                            .to_string(),
                    )
                })
        })
    }

    fn onecall(&self) -> Result<OneCall, WeatherError> {
        self.get(
            "onecall",
            &[
                ("exclude", "currently,minutely"),
                ("units", "imperial"),
            ],
        )
    }

    fn historical_day(
        &self,
        date: DateTime<Utc>,
    ) -> Result<Vec<HistoricalHourly>, WeatherError> {
        let historical: Historical = self.get(
            "onecall/timemachine",
            &[
                ("units", "imperial"),
                ("dt", &date.timestamp().to_string()),
            ],
        )?;
        Ok(historical.hourly)
    }

    fn yesterday(
        &self,
    ) -> Result<Vec<HistoricalHourly>, WeatherError> {
        self.historical_day(Utc::now() - Duration::days(1))
    }
}

#[derive(Error, Debug)]
enum WeatherError {
    #[error("Request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("{0} while deserializing JSON: {1}")]
    Deserialize(serde_json::Error, String),
    #[error("Client error ({}): {}", .0.code, .0.message)]
    Client(ClientError),
}

impl TryFrom<&[u8]> for WeatherError {
    type Error = serde_json::Error;
    fn try_from(response: &[u8]) -> Result<Self, Self::Error> {
        Ok(WeatherError::Client(serde_json::from_slice(
            response,
        )?))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientError {
    /// HTTP response code.
    #[serde(rename = "cod")]
    code: u16,
    message: String,
}

/// A command-line interface to the openweathermap.org API.
#[derive(Debug, StructOpt)]
struct Opt {
    /// Config filename; a JSON file with an `api_key` field.
    #[structopt(
        short,
        long,
        parse(from_os_str),
        default_value = "openweather_api.json"
    )]
    config: PathBuf,
}

enum TempDifference {
    MuchColder,
    Colder,
    Same,
    Warmer,
    MuchWarmer,
}

impl fmt::Display for TempDifference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TempDifference::MuchColder => "much colder",
                TempDifference::Colder => "colder",
                TempDifference::Same => "about the same",
                TempDifference::Warmer => "warmer",
                TempDifference::MuchWarmer => "much warmer",
            }
        )
    }
}

impl TempDifference {
    fn from(from: f64, to: f64) -> Self {
        let delta = to - from;
        if delta > 10.0 {
            TempDifference::MuchWarmer
        } else if delta > 5.0 {
            TempDifference::Warmer
        } else if delta < -10.0 {
            TempDifference::MuchColder
        } else if delta < -5.0 {
            TempDifference::Colder
        } else {
            TempDifference::Same
        }
    }
}

fn average(itr: impl Iterator<Item = f64>) -> f64 {
    let (sum, count) = itr
        .fold((0.0, 0), |(sum, count), item| {
            (sum + item, count + 1)
        });
    sum / count as f64
}
