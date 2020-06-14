use crate::api::{Location, LocationWeather};
use crate::util;
use yew::{html, Html};

pub trait View {
    fn display(&self) -> Html;
}

pub struct WeatherBoard {
    background: Option<WeatherBackground>,
    today: WeatherToday,
}

impl WeatherBoard {
    pub fn new(location: Location, weather: LocationWeather) -> Self {
        let title = location.title();
        let background = WeatherBackground::new(
            weather.id(),
            weather.is_night(),
            weather.temperature.feels_like,
        );
        let sun_rise_time = weather.sun_rise_time();
        let sun_set_time = weather.sun_set_time();
        let icon = WeatherIcon::new(
            weather.id(),
            weather.is_night(),
            weather.temperature.feels_like,
        );
        let today = WeatherToday {
            title,
            icon,
            weather,
            sun_rise_time,
            sun_set_time,
        };
        WeatherBoard { background, today }
    }
}

impl View for WeatherBoard {
    fn display(&self) -> Html {
        html! {
        <>
            {
            if let Some(background) = self.background.as_ref() {
                background.display()
            } else {
                html!{}
            }
            }
            { self.today.display() }
        </>
        }
    }
}

struct WeatherBackground {
    source_video: String,
}

impl WeatherBackground {
    pub fn new(weather_id: u16, is_night: bool, feels_like: f32) -> Option<Self> {
        let source_video = util::resources::animation(weather_id, is_night, feels_like)?;
        Some(WeatherBackground { source_video })
    }
}

impl View for WeatherBackground {
    fn display(&self) -> Html {
        html! {
        <>
            <video autoplay=true loop=true muted=true webkit-playsinline=true
            playsinline=true id="background">
                <source src={ &format!("/animations/{}", self.source_video) } type="video/mp4"/>
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

struct WeatherIcon {
    image_source: String,
}

impl WeatherIcon {
    pub fn new(weather_id: u16, is_night: bool, feels_like: f32) -> Option<Self> {
        let image_source = util::resources::icon(weather_id, is_night, feels_like)?;
        Some(WeatherIcon { image_source })
    }
}

impl View for WeatherIcon {
    fn display(&self) -> Html {
        html! {
            <img id="icon" src={ &format!("/icons/{}", self.image_source) } />
        }
    }
}

pub struct WeatherToday {
    title: String,
    icon: Option<WeatherIcon>,
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
                <h1>{ &self.title }</h1>
                <h2 margin-top="0">{ &self.weather.description() }</h2>
                <div>
                    <div style="float: left">
                        <p style="font-size: 40px" class="no_margin_top">{ &format!("{}", temperature.temp.round() as isize) }</p>
                    </div>
                    <div style="float: left">
                        <p style="font-size: 30px" class="no_margin_top">{ "°C" }</p>
                    </div>
                </div>
                <div style="display: inline-block">

                <table id="table">
                    <tr>
                        <th>{ "Feels Like" }</th>
                        <th>{ &format!("{} °C", temperature.feels_like.round() as isize) }</th>
                    </tr>
                    <tr>
                        <th>{ "Min"}</th>
                        <th>{ &format!("{} °C", temperature.temp_min.round() as isize) }</th>
                    </tr>
                    <tr>
                        <th>{ "Max"}</th>
                        <th>{ &format!("{} °C", temperature.temp_max.round() as isize) }</th>
                    </tr>
                    <tr>
                        <th>{ "Pressure"}</th>
                        <th>{ &format!("{} hPa", temperature.pressure) }</th>
                    </tr>
                    <tr>
                        <th>{ "Humidity"}</th>
                        <th>{ &format!("{} %", temperature.humidity) }</th>
                    </tr>
                    <tr></tr>
                    <tr>
                        <th>{ "Visibility"}</th>
                        <th>{ &format!("{} m", self.weather.visibility) }</th>
                    </tr>
                    <tr>
                        <th>{ "Wind Speed"}</th>
                        <th>{ &format!("{} m/s", wind.speed) }</th>
                    </tr>
                    <tr>
                        <th>{ "Wind Degree" }</th>
                        <th>{ &format!("{} °", wind.degree) }</th>
                    </tr>
                    <tr>
                        <th>{ "Sunrise"}</th>
                        <th>{ &format!("{}", self.sun_rise_time) }</th>
                    </tr>
                    <tr>
                        <th>{ "Sunset"}</th>
                        <th>{ &format!("{}", self.sun_set_time) }</th>
                    </tr>
                </table>
                {
                    if let Some(icon) = self.icon.as_ref() {
                        icon.display()
                    } else {
                        html! {}
                    }
                }
                </div>
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
