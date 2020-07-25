use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::path::PathBuf;

use chrono::prelude::*;
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
    let config: OpenWeather =
        serde_json::from_reader(&config_json)
            .wrap_err("Failed to deserialize JSON")?;
    let onecall: OneCall = config
        .get(
            "onecall",
            &[
                ("exclude", "currently,minutely"),
                ("units", "imperial"),
            ],
        )
        .wrap_err("Failed to deserialize hourly weather data")?;
    println!("Data: {:#?}", onecall);
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
                .unwrap_or(WeatherError::Deserialize(err))
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

    fn historical(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<HistoricalHourly>, WeatherError> {
        let historical: Historical = self.get(
            "onecall",
            &[
                ("exclude", "currently,minutely"),
                ("units", "imperial"),
                ("start", &start.timestamp().to_string()),
                ("end", &end.timestamp().to_string()),
            ],
        )?;
        Ok(historical.hourly)
    }

    fn historical_count(
        &self,
        start: DateTime<Utc>,
        count: usize,
    ) -> Result<Vec<HistoricalHourly>, WeatherError> {
        let historical: Historical = self.get(
            "onecall",
            &[
                ("exclude", "currently,minutely"),
                ("units", "imperial"),
                ("start", &start.timestamp().to_string()),
                ("cnt", &count.to_string()),
            ],
        )?;
        Ok(historical.hourly)
    }
}

#[derive(Error, Debug)]
enum WeatherError {
    #[error("Request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Deserializing JSON: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("Client error ({}): {}", .0.code, .0.message)]
    Client(ClientError),
    #[error("Couldn't parse timestamp: {0}")]
    DateTime(#[from] chrono::format::ParseError),
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
