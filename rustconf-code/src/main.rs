use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use reqwest::{Client, Response};
use serde::Deserialize;
use structopt::StructOpt;
use thiserror::Error;

mod raw;
mod weather;
pub use weather::CurrentWeather;

static OPEN_WEATHER_BASE: &str = "https://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
async fn main() {
	if let Err(err) = main_inner().await {
		println!("{}", err);
	}
}

async fn main_inner() -> Result<(), MainError> {
	let opt = Opt::from_args();
	let api_key = File::open(opt.key_file)?;
	let client: OpenWeather = serde_json::de::from_reader(api_key)?;
	let weather = client.current_weather(&opt.location).await?;
	println!("{:#?}", weather);
	Ok(())
}

#[derive(StructOpt)]
struct Opt {
	/// OpenWeather API key file; a JSON file containing an `api_key` field.
	#[structopt(
		long = "key-file",
		parse(from_os_str),
		default_value = "openweather_api.json"
	)]
	key_file: PathBuf,

	/// OpenWeather location name.
	#[structopt(default_value = "Waltham,MA,US")]
	location: String,
}

#[derive(Error, Debug)]
enum MainError {
	#[error("{0}")]
	Weather(#[from] WeatherError),
	#[error("{0}")]
	Io(#[from] io::Error),
	#[error("Deserializing JSON: {0}")]
	Json(#[from] serde_json::error::Error),
}

#[derive(Deserialize, Debug, Clone)]
struct OpenWeather {
	api_key: String,
	#[serde(skip)]
	client: Client,
}

impl OpenWeather {
	async fn current_weather_raw(&self, location: &str) -> Result<Response, reqwest::Error> {
		self.client
			.get(OPEN_WEATHER_BASE)
			.query(&[
				("q", location),
				("units", "metric"),
				("mode", "xml"),
				("appid", &self.api_key),
			])
			.send()
			.await
	}

	pub async fn current_weather(&self, location: &str) -> Result<CurrentWeather, WeatherError> {
		let bytes = self.current_weather_raw(location).await?.bytes().await?;
		let res: Result<raw::CurrentWeather, WeatherError> =
			quick_xml::de::from_reader(bytes.as_ref()).map_err(Into::into);
		match res {
			Ok(weather) => Ok(weather.try_into()?),
			Err(_) => Err(WeatherError::Client(quick_xml::de::from_reader(
				bytes.as_ref(),
			)?)),
		}
	}
}

#[derive(Error, Debug)]
enum WeatherError {
	#[error("Request: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Deserializing XML: {0}")]
	Deserialize(#[from] quick_xml::de::DeError),
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
