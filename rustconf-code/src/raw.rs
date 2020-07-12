use derive_more::{Add, AddAssign, Mul, MulAssign};
use serde::{Deserialize, Serialize};

macro_rules! num_types {
    ($($it:item)*) => {
        $(
            #[derive(
                Serialize,
                Deserialize,
                Debug,
                Clone,
                Copy,
                PartialEq,
                PartialOrd,
                derive_more::From,
                derive_more::Into,
                Add,
                Mul,
                AddAssign,
                MulAssign,
            )]
            #[serde(transparent)]
            $it
        )*
    }
}

num_types! {
    pub struct Latitude(f64);
    pub struct Longitude(f64);
    pub struct Celsius(f64);
    /// hPa = 100 [Pa], `kg / (m s^2)`.
    ///
    /// [Pa]: https://en.wikipedia.org/wiki/Pascal_(unit)
    pub struct HectoPascal(f64);
    /// A percent, from 0 to 100.
    pub struct Percent(u16);
    pub struct Meters(f64);
    /// Meters per second.
    pub struct MetersSec(f64);
    /// mm.
    pub struct Millimeters(f64);
    pub struct Degrees(f64);
    /// UTC [Unix timestamp] for date of calculation.
    ///
    /// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
    pub struct UnixTime(u64);
    pub struct Seconds(i32);
    pub struct CityId(u64);
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "current")]
pub struct CurrentWeather {
    pub city: City,
    pub temperature: Temperature,
    pub feels_like: FeelsLike,
    pub humidity: Humidity,
    pub pressure: Pressure,
    pub wind: Wind,
    pub clouds: Clouds,
    pub visibility: Visibility,
    pub precipitation: Precipitation,
    pub weather: WeatherDescription,
    #[serde(rename = "lastupdate")]
    pub last_update: LastUpdate,
}

#[derive(Deserialize, Debug, Clone)]
pub struct City {
    pub id: String,
    pub name: String,
    pub coord: LatLon,
    /// An [ISO 3166-1 alpha-2] country code, like `US`, `GB`, or `JP`.
    ///
    /// [ISO 3166-1 alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    pub country: String,
    pub timezone: Seconds,
    pub sun: Sun,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LatLon {
    lat: Latitude,
    lon: Longitude,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sun {
    /// Sunrise time, ISO 8601 timestamp.
    pub rise: String,
    /// Sunset time, ISO 8601 timestamp.
    pub set: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Temperature {
    #[serde(rename = "value")]
    pub current: Celsius,
    /// Minimum temperature within the city; relevant for large urban areas.
    pub min: Celsius,
    pub max: Celsius,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeelsLike {
    pub value: Celsius,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Humidity {
    #[serde(rename = "value")]
    pub percent: Percent,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pressure {
    pub value: HectoPascal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wind {
    pub speed: WindSpeed,
    pub gusts: Gusts,
    pub direction: Direction,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WindSpeed {
    #[serde(rename = "value")]
    pub speed: MetersSec,
    /// e.g. `"Light breeze"`.
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Gusts {
    #[serde(rename = "value")]
    pub speed: Option<MetersSec>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Direction {
    #[serde(rename = "value")]
    pub degrees: Degrees,
    /// e.g. `"WSW"`
    pub code: String,
    /// e.g. `"West-southwest"`
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Clouds {
    #[serde(rename = "value")]
    pub cloudiness: Percent,
    /// e.g. `"scattered clouds"`
    #[serde(rename = "name")]
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Visibility {
    #[serde(rename = "value")]
    pub distance: Meters,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Precipitation {
    /// `"no"` or name of weather such as `"rain"` or `"snow"`.
    pub mode: String,
    pub amount: Option<Millimeters>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherDescription {
    /// Condition code; see [Weather Conditions][conditions].
    ///
    /// [conditions]: https://openweathermap.org/weather-conditions#Weather-Condition-Codes-2
    number: i16,
    /// Short description, e.g. `"Clear"`
    value: String,
    /// Icon name, e.g. `"01n"`; see [Weather Icons].
    ///
    /// [Weather Icons]: https://openweathermap.org/weather-conditions#Icon-list
    icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastUpdate {
    /// ISO 8601 timestamp.
    #[serde(rename = "value")]
    pub time: String,
}
