use crate::api::{LocationWeather, Weather};
use yew::prelude::Properties;
use yew::{html, Component, ComponentLink, Html};

pub trait View {
    fn display(&self) -> Html;
}

pub struct WeatherBoard {
    title: String,
    background: WeatherBackground,
    today: WeatherToday,
    forecast: ForecastPanel,
}

impl WeatherBoard {
    pub fn new(mut weather: LocationWeather) -> Self {
        let title = weather.title();
        let weather_today = weather.consolidated_weather.remove(0); // may crash. Fix later
        let background = WeatherBackground {
            weather_id: weather_today.weather_state_abbr.clone(),
            is_night: weather.is_night(),
        };
        let today = WeatherToday {
            weather: weather_today,
            sun_rise_time: weather.sun_rise_time().unwrap_or("-".to_string()),
            sun_set_time: weather.sun_set_time().unwrap_or("-".to_owned()),
        };
        let forecast = ForecastPanel {
            coming_weather: weather.consolidated_weather,
        };
        WeatherBoard {
            title,
            background,
            today,
            forecast,
        }
    }
}

impl View for WeatherBoard {
    fn display(&self) -> Html {
        html! {
        <>
            <h1>{ &self.title }</h1>
            { self.background.display() }
            { self.today.display() }
            { self.forecast.display() }
        </>
        }
    }
}

struct WeatherBackground {
    weather_id: String,
    is_night: bool,
}

impl WeatherBackground {
    fn source_video(&self) -> &'static str {
        if self.is_night {
            "night "
        } else {
            ""
        }
    }
}

impl View for WeatherBackground {
    fn display(&self) -> Html {
        html! {
            <video id="background">
                <source src={ self.source_video() } type={ "video/mp4" }/>
            </video>
        }
    }
}

struct WeatherToday {
    weather: Weather,
    sun_rise_time: String,
    sun_set_time: String,
}

impl View for WeatherToday {
    fn display(&self) -> Html {
        html! {
            <div>
                <h2>{ &self.weather.weather_state_name }</h2>
            </div>
        }
    }
}

pub struct ForecastPanel {
    coming_weather: Vec<Weather>,
}

impl View for ForecastPanel {
    fn display(&self) -> Html {
        html! {
            <div>
                <h2>{ self.coming_weather.len() }</h2>
            </div>
        }
    }
}
