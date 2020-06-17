use crate::api::{Condition, LocationWeather};
use crate::util;
use yew::{html, Callback, Html, MouseEvent};

pub trait View {
    fn display(&self) -> Html;
}

pub struct WeatherBoard {
    background: Option<WeatherBackground>,
    today: WeatherToday,
    shade_button_callback: Callback<MouseEvent>,
}

impl WeatherBoard {
    pub fn new(
        title: String,
        weather: LocationWeather,
        portrait: bool,
        sync_button_callback: Callback<MouseEvent>,
        shade_button_callback: Callback<MouseEvent>,
    ) -> Self {
        let title = title;
        let background = WeatherBackground::new(
            weather.id(),
            weather.is_night(),
            weather.temperature.feels_like,
            portrait,
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
            sync_button_callback,
        };
        WeatherBoard {
            background,
            today,
            shade_button_callback,
        }
    }
}

impl View for WeatherBoard {
    fn display(&self) -> Html {
        html! {
        <>
            {
                self.background.as_ref()
                    .map(|background| background.display())
                    .unwrap_or(html!{})
            }
            <div id="weatherPanel">
            <div id="panelControl">
                <button class="fn" id="shade" onclick=&self.shade_button_callback>{ "⬇️" }</button>
            </div>
            {
                self.today.display()
            }
            </div>
        </>
        }
    }
}

struct WeatherBackground {
    source_video: String,
}

impl WeatherBackground {
    pub fn new(
        weather_condition: Condition,
        is_night: bool,
        feels_like: f32,
        portrait: bool,
    ) -> Option<Self> {
        let source_video =
            util::resources::animation(weather_condition, is_night, feels_like, portrait)?;
        Some(WeatherBackground { source_video })
    }
}

impl View for WeatherBackground {
    fn display(&self) -> Html {
        html! {
        <>
            <video preload="auto" autoplay=true loop=true muted=true webkit-playsinline=true
            playsinline=true id="background">
                <source src={ &format!("animations/{}", self.source_video) } type="video/mp4"/>
            </video>
        </>
        }
    }
}

struct WeatherIcon {
    image_source: String,
}

impl WeatherIcon {
    pub fn new(weather_condition: Condition, is_night: bool, feels_like: f32) -> Option<Self> {
        let image_source = util::resources::icon(weather_condition, is_night, feels_like)?;
        Some(WeatherIcon { image_source })
    }
}

impl View for WeatherIcon {
    fn display(&self) -> Html {
        html! {
            <img id="icon" src={ &format!("icons/{}", self.image_source) } />
        }
    }
}

struct WeatherToday {
    title: String,
    icon: Option<WeatherIcon>,
    weather: LocationWeather,
    sun_rise_time: String,
    sun_set_time: String,
    sync_button_callback: Callback<MouseEvent>,
}

impl View for WeatherToday {
    fn display(&self) -> Html {
        let temperature = &self.weather.temperature;
        let wind = &self.weather.wind;
        html! {
            <div id="today">
                <div>
                <h1 style="display: inline-block">{ &self.title }</h1>
                <button class="fn" id="sync" onclick=&self.sync_button_callback>{ "🔄" }</button>
                </div>
                <div>
                    {
                        self.icon.as_ref()
                            .map(|icon| icon.display())
                            .unwrap_or(html!{})
                    }
                    <h2 margin-top="0">{ &self.weather.description() }</h2>
                    <div id="temperatures">
                        <div>
                            <div style="font-size: 40px" class="no_margin_top">{ &format!("{}", temperature.temp.round() as isize) }</div>
                        </div>
                        <div>
                            <div style="font-size: 30px" class="no_margin_top">{ "°C" }</div>
                        </div>
                        <div id="max-min-temp">
                            <div>{ &format!("{} °C", temperature.temp_max.round() as isize) }</div>
                            <div>{ &format!("{} °C", temperature.temp_min.round() as isize) }</div>
                        </div>
                    </div>
                </div>

                <table id="table">
                    <tr>
                        <th>{ "Feels Like" }</th>
                        <td>{ &format!("{} °C", temperature.feels_like.round() as isize) }</td>
                    </tr>
                    <tr>
                        <th>{ "Pressure"}</th>
                        <td>{ &format!("{} hPa", temperature.pressure) }</td>
                    </tr>
                    <tr>
                        <th>{ "Humidity"}</th>
                        <td>{ &format!("{} %", temperature.humidity) }</td>
                    </tr>
                    {
                        if let Some(visibility) = self.weather.visibility {
                            html!{
                            <tr>
                                <th>{ "Visibility"}</th>
                                <td>{ &format!("{} m", visibility) }</td>
                            </tr>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <tr>
                        <th>{ "Wind Speed"}</th>
                        <td>{ &format!("{} m/s", wind.speed) }</td>
                    </tr>
                    <tr>
                        <th>{ "Wind Degree" }</th>
                        <td>{ &format!("{} °", wind.degree) }</td>
                    </tr>
                    {
                        self.weather.clouds.as_ref()
                            .map(|cloud| html! {
                            <tr>
                                <th>{ "Cloudiness" }</th>
                                <td>{ &format!("{}%", cloud.all) }</td>
                            </tr>
                            })
                            .unwrap_or(html!{})
                    }
                    {
                        self.weather.rain.as_ref()
                            .and_then(|rain| rain.one_hour)
                            .map(|rain| html! {
                            <tr>
                                <th>{ "Rain in 1 hour" }</th>
                                <td>{ &format!("{:.2} mm", rain) }</td>
                            </tr>
                            })
                            .unwrap_or(html!{})
                    }
                    {
                        self.weather.rain.as_ref()
                            .and_then(|rain| rain.three_hour)
                            .map(|rain| html! {
                            <tr>
                                <th>{ "Rain in 3 hour" }</th>
                                <td>{ &format!("{:.2} mm", rain) }</td>
                            </tr>
                            })
                            .unwrap_or(html!{})
                    }
                    {
                        self.weather.snow.as_ref()
                            .and_then(|snow| snow.one_hour)
                            .map(|snow| html! {
                            <tr>
                                <th>{ "Snow in 1 hour" }</th>
                                <td>{ &format!("{:.2} mm", snow) }</td>
                            </tr>
                            })
                            .unwrap_or(html!{})
                    }
                    {
                        self.weather.snow.as_ref()
                            .and_then(|snow| snow.three_hour)
                            .map(|snow| html! {
                            <tr>
                                <th>{ "Snow in 3 hour" }</th>
                                <td>{ &format!("{:.2} mm", snow) }</td>
                            </tr>
                            })
                            .unwrap_or(html!{})
                    }
                    <tr>
                        <th>{ "Sunrise"}</th>
                        <td>{ &format!("{}", self.sun_rise_time) }</td>
                    </tr>
                    <tr>
                        <th>{ "Sunset"}</th>
                        <td>{ &format!("{}", self.sun_set_time) }</td>
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
