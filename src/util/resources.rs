// The numeric code is provided by OpenWeather to indicate the weather condition
// The full table and explaination can be found here: https://openweathermap.org/weather-conditions

use crate::api::Condition::{self, *};
use crate::api::{Atmosphere, Cloud};

pub fn animation(id: Condition, is_night: bool, feels_like: f32, portrait: bool) -> Option<String> {
    use Atmosphere::*;
    let mut video_name = String::from("weather_");
    if !portrait {
        video_name.push_str("l_");
    }
    let (weather, append_time_suffix) = match id {
        Thunderstorm(_) => ("thunderstorm", true),
        Drizzle(_) | Rain(_) => ("rain", true),
        Snow(_) => ("snow", true),
        Atmosphere(atmosphere) => match atmosphere {
            VolcanicAsh | Squalls | Tornado => ("thunderstorm", true),
            Mist | Smoke | Haze | Fog => ("fog", true),
            SandWhirls | Sand | Dust => ("windy", true),
        },
        Cloud(cloud) => match cloud {
            Cloud::Clear => {
                if is_night {
                    ("clear", false)
                } else if feels_like > 35.0 {
                    ("hot", false)
                } else {
                    ("sunny", false)
                }
            }
            Cloud::FewClouds => {
                if is_night {
                    ("partly_cloud", true)
                } else {
                    ("partly_sunny", false)
                }
            }
            Cloud::ScatteredClouds => ("partly_cloud", true),
            Cloud::BrokenClouds | Cloud::OvercastClouds => ("cloudy", true),
        },
    };
    video_name.push_str(weather);
    if append_time_suffix {
        video_name.push('_');
        video_name.push_str(match is_night {
            true => "night",
            false => "day",
        });
    }
    video_name.push_str(".mp4");
    Some(video_name)
}

pub fn icon(id: Condition, is_night: bool, feels_like: f32) -> Option<String> {
    let mut icon_name = String::new();
    let (weather, append_time_suffix) = match id {
        Thunderstorm(thunder) => {
            use crate::api::Thunderstorm::*;
            match thunder {
                LightThunderstorm | Thunderstorm | HeavyThunderstorm | RaggedThunderstorm => {
                    ("thunderstorm", false)
                }
                ThunderstormWithLightRain | ThunderstormWithRain | ThunderstormWithHeavyRain => {
                    ("thunderstorm_with_rain", true)
                }
                ThunderstormWithLightDrizzle
                | ThunderstormWithDrizzle
                | ThunderstormWithHeavyDrizzle => ("thunderstorm_with_drizzle", true),
            }
        }
        Drizzle(drizzle) => {
            use crate::api::Drizzle::*;
            match drizzle {
                LightIntensityDrizzle
                | Drizzle
                | HeavyIntensityDrizzle
                | LightIntensityDrizzleRain
                | DrizzleRain => ("light_drizzle", true),
                HeavyIntensityDrizzleRain
                | ShowerRainAndDrizzle
                | HeavyShowerRainAndDrizzle
                | ShowerDrizzle => ("drizzle", true),
            }
        }
        Rain(rain) => {
            use crate::api::Rain::*;
            match rain {
                LightRain | ModerateRain | LightIntensityShowerRain | ShowerRain => {
                    if is_night {
                        ("light_rain", true)
                    } else {
                        ("rain", true)
                    }
                }
                HeavyIntensityRain
                | VeryHeavyRain
                | ExtremeRain
                | HeavyIntensityShowerRain
                | RaggedShowerRain => ("rain", true),
                FreezingRain => ("light_shower_snow", false),
            }
        }
        Snow(snow) => {
            use crate::api::Snow::*;
            match snow {
                LightRainAndSnow | RainAndSnow | LightShowerSnow | ShowerSnow | HeavyShowerSnow => {
                    ("rain_snow", false)
                }
                LightSnow => ("light_snow", true),
                Snow => ("snow", true),
                HeavySnow => ("heavy_snow", true),
                LightShowerSleet => ("light_snow_sleet", false),
                Sleet | ShowerSleet => ("sleet", false),
            }
        }
        Atmosphere(atmosphere) => {
            use Atmosphere::*;
            match atmosphere {
                Mist => ("mist", false),
                Smoke | SandWhirls | Sand | Dust | VolcanicAsh | Squalls => ("smoke", false),
                Fog => ("fog", false),
                Haze => ("haze", true),
                Tornado => ("tornado", false),
            }
        }
        Cloud(cloud) => {
            use Cloud::*;
            match cloud {
                Clear => {
                    if is_night || feels_like < 35.0 {
                        ("clear", true)
                    } else {
                        ("hot", false)
                    }
                }
                FewClouds => ("partly_clear", true),
                ScatteredClouds => ("partly_cloudy", true),
                BrokenClouds => ("mostly_cloudy", true),
                OvercastClouds => ("cloudy", true),
            }
        }
    };
    icon_name.push_str(weather);
    if append_time_suffix {
        icon_name.push('_');
        icon_name.push_str(match is_night {
            true => "night",
            false => "day",
        });
    }
    icon_name.push_str(".png");
    Some(icon_name)
}
