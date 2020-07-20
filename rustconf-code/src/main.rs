use std::fs::File;
use std::path::PathBuf;

use eyre::WrapErr;
use reqwest::blocking::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use structopt::StructOpt;
use thiserror::Error;

mod openweather;
use openweather::OneCall;

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
        match serde_json::from_reader(&*bytes) {
            Ok(val) => Ok(val),
            Err(err) => {
                let client_err: Result<ClientError, _> =
                    serde_json::from_reader(&*bytes);
                match client_err {
                    Ok(client_err) => {
                        Err(WeatherError::Client(client_err))
                    }
                    Err(_) => {
                        Err(WeatherError::Deserialize(err))
                    }
                }
            }
        }
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
