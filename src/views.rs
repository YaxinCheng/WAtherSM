use crate::api::{Location, LocationWeather};
use yew::{html, Html};

pub trait View {
    fn display(&self) -> Html;
}

pub struct WeatherBoard {
    title: String,
    background: WeatherBackground,
    today: WeatherToday,
}

impl WeatherBoard {
    pub fn new(location: Location, weather: LocationWeather) -> Self {
        let title = location.title();
        let background = WeatherBackground {
            weather_id: weather.id(),
            is_night: weather.is_night(),
        };
        let sun_rise_time = weather.sun_rise_time();
        let sun_set_time = weather.sun_set_time();
        let today = WeatherToday {
            weather,
            sun_rise_time,
            sun_set_time,
        };
        WeatherBoard {
            title,
            background,
            today,
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
        </>
        }
    }
}

struct WeatherBackground {
    weather_id: u16,
    is_night: bool,
}

impl WeatherBackground {
    fn source_video(&self) -> &'static str {
        if self.is_night {
            "weather_l_sunny.mp4"
        } else {
            ""
        }
    }
}

impl View for WeatherBackground {
    fn display(&self) -> Html {
        html! {
        <>
            <video autoplay=true loop=true muted=true webkit-playsinline=true
            playsinline=true id="background">
                <source src={ self.source_video() } type="video/mp4"/>
            </video>
            <script>
               {"
                const media = document.getElementById('background');
                media.muted = true;
                media.play();
               "}
            </script>
        </>
        }
    }
}

struct WeatherToday {
    weather: LocationWeather,
    sun_rise_time: String,
    sun_set_time: String,
}

impl View for WeatherToday {
    fn display(&self) -> Html {
        let temperature = &self.weather.temperature;
        let wind = &self.weather.wind;
        html! {
            <div id="today">
                <h2>{ &self.weather.description() }</h2>
                <h3>{ &format!("{} °C", temperature.temp) }</h3>
                <table>
                    <tr>
                        <th style="text-align: right">{ "Feels Like" }</th>
                        <th style="text-align: left">{ &format!("{} °C", temperature.feels_like) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Min"}</th>
                        <th style="text-align: left">{ &format!("{} °C", temperature.temp_min) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Max"}</th>
                        <th style="text-align: left">{ &format!("{} °C", temperature.temp_max) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Pressure"}</th>
                        <th style="text-align: left">{ &format!("{} hPa", temperature.pressure) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Humidity"}</th>
                        <th style="text-align: left">{ &format!("{} %", temperature.humidity) }</th>
                    </tr>
                    <tr></tr>
                    <tr>
                        <th style="text-align: right">{ "Visibility"}</th>
                        <th style="text-align: left">{ &format!("{} m", self.weather.visibility) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Wind Speed"}</th>
                        <th style="text-align: left">{ &format!("{} m/s", wind.speed) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Degree" }</th>
                        <th style="text-align: left">{ &format!("{} °", wind.degree) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Sunrise"}</th>
                        <th style="text-align: left">{ &format!("{}", self.sun_rise_time) }</th>
                    </tr>
                    <tr>
                        <th style="text-align: right">{ "Sunset"}</th>
                        <th style="text-align: left">{ &format!("{}", self.sun_set_time) }</th>
                    </tr>
                </table>
            </div>
        }
    }
}

// pub struct ForecastPanel {
//     coming_weather: Vec<Weather>,
// }
//
// impl View for ForecastPanel {
//     fn display(&self) -> Html {
//         html! {
//             <div>
//                 <h2>{ self.coming_weather.len() }</h2>
//             </div>
//         }
//     }
// }
