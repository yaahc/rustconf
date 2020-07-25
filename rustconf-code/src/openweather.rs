use chrono::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "i64")]
pub struct UnixUTC(DateTime<Utc>);

impl From<i64> for UnixUTC {
    fn from(time: i64) -> Self {
        Self(Utc.timestamp(time, 0))
    }
}

impl Into<i64> for UnixUTC {
    fn into(self) -> i64 {
        self.0.timestamp()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OneCall {
    timezone_offset: UnixUTC,
    hourly: Vec<Hourly>,
    daily: Vec<Daily>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hourly {
    dt: UnixUTC,
    temp: f64,
    feels_like: f64,
    humidity: f64,
    clouds: f64,
    rain: Option<Rain>,
    snow: Option<Snow>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cloudiness {
    all: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rain {
    #[serde(rename = "1h")]
    one_hour: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Snow {
    #[serde(rename = "1h")]
    one_hour: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Daily {
    dt: UnixUTC,
    sunrise: UnixUTC,
    sunset: UnixUTC,
    rain: Option<f64>,
    snow: Option<f64>,
    temp: DailyTemp,
    feels_like: DailyTempCommon,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyTempCommon {
    morn: f64,
    day: f64,
    eve: f64,
    night: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyTemp {
    #[serde(flatten)]
    common: DailyTempCommon,
    min: f64,
    max: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Historical {
    pub hourly: Vec<HistoricalHourly>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HistoricalHourly {
    pub dt: UnixUTC,
    pub main: HistoricalMain,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HistoricalMain {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: f64,
}
