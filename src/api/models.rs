use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LocationWeather {
    #[serde(rename = "weather")]
    pub descriptions: Vec<Desc>,
    #[serde(rename = "main")]
    pub temperature: Temperatures,
    pub visibility: Option<usize>,
    pub wind: Wind,
    //pub clouds: Clouds,
    #[serde(rename = "dt")]
    pub current_time: i64,
    #[serde(rename = "sys")]
    pub times: Times,
    #[serde(rename = "timezone")]
    timezone_offset: i64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Temperatures {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: isize,
    pub humidity: isize,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    #[serde(rename = "deg")]
    pub degree: f32,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Clouds {
    pub all: u8,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Times {
    sunrise: i64,
    sunset: i64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Desc {
    id: u16,
    description: String,
}

impl LocationWeather {
    pub fn id(&self) -> u16 {
        self.descriptions
            .first()
            .map(|desc| desc.id)
            .unwrap_or_default()
    }

    pub fn description(&self) -> String {
        let mut description = String::new();
        for (part_index, part) in self
            .descriptions
            .first()
            .map(|desc| desc.description.as_str())
            .unwrap_or_default()
            .split_ascii_whitespace()
            .enumerate()
        {
            if part_index != 0 {
                description.push(' ');
            }
            for (index, char) in part.chars().enumerate() {
                if index == 0 {
                    description.push(char.to_ascii_uppercase());
                } else {
                    description.push(char)
                }
            }
        }
        description
    }

    fn sun_rise(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.times.sunrise + self.timezone_offset, 0)
    }

    fn sun_set(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.times.sunset + self.timezone_offset, 0)
    }

    pub fn sun_rise_time(&self) -> String {
        self.sun_rise().time().format("%H:%M").to_string()
    }

    pub fn sun_set_time(&self) -> String {
        self.sun_set().time().format("%H:%M").to_string()
    }

    pub fn is_night(&self) -> bool {
        self.current_time < self.times.sunrise || self.current_time > self.times.sunset
    }
}
