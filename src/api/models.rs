use chrono::ParseResult;
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LocationWeather {
    title: String,
    time: String,
    sun_rise: String,
    sun_set: String,
    pub consolidated_weather: Vec<Weather>,
    parent: Parent,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Weather {
    pub weather_state_name: String,
    pub weather_state_abbr: String,
    pub wind_direction_compass: String,
    pub min_temp: f32,
    pub max_temp: f32,
    pub the_temp: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub air_pressure: f32,
    pub humidity: isize,
    pub visibility: f32,
    pub applicable_date: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Parent {
    title: String,
}

impl LocationWeather {
    pub fn title(&self) -> String {
        format!("{}, {}", self.title, self.parent.title)
    }

    fn sun_rise(&self) -> ParseResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(&self.sun_rise)
    }

    fn sun_set(&self) -> ParseResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(&self.sun_set)
    }

    pub fn sun_rise_time(&self) -> ParseResult<String> {
        self.sun_rise().map(|datetime| datetime.time().to_string())
    }

    pub fn sun_set_time(&self) -> ParseResult<String> {
        self.sun_set().map(|datetime| datetime.time().to_string())
    }

    pub fn current_time(&self) -> ParseResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(&self.time)
    }

    pub fn is_night(&self) -> bool {
        let (time, sunrise, sunset) = match (self.current_time(), self.sun_rise(), self.sun_set()) {
            (Ok(time), Ok(sunrise), Ok(sunset)) => (time, sunrise, sunset),
            _ => return false,
        };
        time.gt(&sunset) || time.lt(&sunrise)
    }
}
