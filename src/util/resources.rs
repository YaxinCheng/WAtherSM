// The numeric code is provided by OpenWeather to indicate the weather condition
// The full table and explaination can be found here: https://openweathermap.org/weather-conditions

pub fn animation(id: u16, is_night: bool, feels_like: f32) -> Option<String> {
    let mut video_name = String::from("weather_l_");
    let (weather, append_time_suffix) = match id {
        200..=232 | 762..=781 => ("thunderstorm", true),
        300..=321 | 500..=531 => ("rain", true),
        600..=622 => ("snow", true),
        701..=721 | 741 => ("fog", true),
        731 | 751 | 761 => ("windy", true),
        800 => {
            if is_night {
                ("clear", false)
            } else if feels_like > 35.0 {
                ("hot", false)
            } else {
                ("sunny", false)
            }
        }
        801 => {
            if is_night {
                ("partly_cloud", true)
            } else {
                ("partly_sunny", false)
            }
        }
        802 => ("partly_cloud", true),
        803 | 804 => ("cloudy", true),
        _ => return None,
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

pub fn icon(id: u16, is_night: bool, feels_like: f32) -> Option<String> {
    let mut icon_name = String::new();
    let (weather, append_time_suffix) = match id {
        210..=221 => ("thunderstorm", false),
        200..=202 => ("thunderstorm_with_rain", true),
        230..=232 => ("thunderstorm_with_drizzle", true),
        300..=311 => ("light_drizzle", true),
        312..=321 => ("drizzle", true),
        500 | 501 | 520 | 521 => {
            if is_night {
                ("light_rain", true)
            } else {
                ("rain", true)
            }
        }
        502..=504 | 522 | 531 => ("rain", true),
        511 => ("light_shower_snow", false),
        615..=622 => ("rain_snow", false),
        600 => ("light_snow", true),
        601 => ("snow", true),
        602 => ("heavy_snow", true),
        612 => ("light_snow_sleet", false),
        611 | 613 => ("sleet", false),
        701 => ("mist", false),
        702 | 731 | 751 | 761 | 762 | 771 => ("smoke", false),
        741 => ("fog", false),
        721 => ("haze", true),
        781 => ("tornado", false),
        800 => {
            if is_night || feels_like < 35.0 {
                ("clear", true)
            } else {
                ("hot", false)
            }
        }
        801 => ("partly_clear", true),
        802 => ("partly_cloudy", true),
        803 => ("mostly_cloudy", true),
        804 => ("cloudy", true),
        _ => return None,
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
