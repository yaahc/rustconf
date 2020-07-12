use std::convert::{TryFrom, TryInto};

use chrono::prelude::*;

use crate::raw;

pub use crate::raw::{
    Celsius, Clouds, Direction, HectoPascal, LatLon, Meters, MetersSec, Millimeters, Percent,
    WeatherDescription,
};

#[derive(Debug, Clone)]
pub struct CurrentWeather {
    city: City,
    sun: Sun,
    temperature: Temperature,
    humidity: Percent,
    pressure: HectoPascal,
    wind: Wind,
    clouds: Clouds,
    visibility: Meters,
    precipitation: Option<Precipitation>,
    description: WeatherDescription,
    last_update: DateTime<Utc>,
}

impl TryFrom<raw::CurrentWeather> for CurrentWeather {
    type Error = chrono::format::ParseError;
    fn try_from(weather: raw::CurrentWeather) -> Result<Self, Self::Error> {
        Ok(CurrentWeather {
            sun: (&weather.city.sun).try_into()?,
            city: weather.city.into(),
            temperature: Temperature {
                current: weather.temperature.current,
                min: weather.temperature.min,
                max: weather.temperature.max,
                feels_like: weather.feels_like.value,
            },
            humidity: weather.humidity.percent,
            pressure: weather.pressure.value,
            wind: weather.wind.into(),
            clouds: weather.clouds,
            visibility: weather.visibility.distance,
            precipitation: weather.precipitation.into(),
            description: weather.weather,
            last_update: parse_datetime(&weather.last_update.time)?,
        })
    }
}

#[derive(Debug, Clone)]
struct City {
    id: String,
    name: String,
    coord: LatLon,
    country: String,
    timezone: FixedOffset,
}

impl From<raw::City> for City {
    fn from(c: raw::City) -> City {
        City {
            id: c.id,
            name: c.name,
            coord: c.coord,
            country: c.country,
            timezone: FixedOffset::east(c.timezone.into()),
        }
    }
}

#[derive(Debug, Clone)]
struct Sun {
    rise: DateTime<Utc>,
    set: DateTime<Utc>,
}

impl<'a> TryFrom<&'a raw::Sun> for Sun {
    type Error = chrono::format::ParseError;
    fn try_from(s: &'a raw::Sun) -> Result<Self, Self::Error> {
        Ok(Sun {
            rise: parse_datetime(&s.rise)?,
            set: parse_datetime(&s.set)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Temperature {
    current: Celsius,
    feels_like: Celsius,
    /// Minimum temperature within the city; relevant for large urban areas.
    min: Celsius,
    max: Celsius,
}

#[derive(Debug, Clone)]
struct Wind {
    speed: MetersSec,
    name: String,
    gusts: Option<MetersSec>,
    direction: Direction,
}

impl From<raw::Wind> for Wind {
    fn from(wind: raw::Wind) -> Wind {
        Wind {
            speed: wind.speed.speed,
            name: wind.speed.name,
            gusts: wind.gusts.speed,
            direction: wind.direction,
        }
    }
}

#[derive(Debug, Clone)]
struct Precipitation {
    /// e.g. `"rain"` or `"snow"`
    kind: String,
    amount: Millimeters,
}

impl From<raw::Precipitation> for Option<Precipitation> {
    fn from(precipitation: raw::Precipitation) -> Option<Precipitation> {
        match precipitation.mode.as_str() {
            "no" => None,
            _ => Some(Precipitation {
                kind: precipitation.mode,
                amount: precipitation
                    .amount
                    .expect("Non-none precipitation should have an amount"),
            }),
        }
    }
}

fn parse_datetime(ts: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    Ok(Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(ts, "%Y-%m-%dT%H:%M:%S")?))
}
