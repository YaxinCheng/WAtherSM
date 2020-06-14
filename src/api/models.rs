use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LocationWeather {
    #[serde(rename = "weather")]
    pub descriptions: Vec<Desc>,
    #[serde(rename = "main")]
    pub temperature: Temperatures,
    pub visibility: usize,
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
pub struct Location {
    title: String,
    parent: Parent,
    latt_long: String,
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

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Parent {
    title: String,
}

impl Location {
    pub fn title(&self) -> String {
        format!("{}, {}", self.title, self.parent.title)
    }

    pub fn lat_lon(&self) -> (f32, f32) {
        let parsed = self
            .latt_long
            .splitn(2, ",")
            .map(str::parse::<f32>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        if parsed.len() != 2 {
            panic!(
                "The lat and lon is not successfully parsed: {}",
                self.latt_long
            );
        }
        (parsed[0], parsed[1])
    }

    pub fn test_default() -> Self {
        Location {
            title: "Toronto".to_string(),
            parent: Parent {
                title: "Canada".to_string(),
            },
            latt_long: "43.648560,-79.385368".to_string(),
        }
    }
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
        self.current_time >= self.times.sunrise || self.current_time < self.times.sunset
    }
}
