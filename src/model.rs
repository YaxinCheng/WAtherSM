use crate::api::{Location, LocationWeather, WeatherAPI};
use crate::views::{View, WeatherBoard};
use anyhow::Error;
use yew::format::Json;
use yew::services::console::ConsoleService;
use yew::services::fetch::Response;
use yew::{html, Component, ComponentLink, Html};

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    api: WeatherAPI,
    view: Option<WeatherBoard>,
}

pub enum Msg {
    LoadLocation,
    LoadWeather(Location),
    Fetched(Location, LocationWeather),
    Failed(&'static str),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,
            console: ConsoleService::new(),
            api: WeatherAPI::new(),
            view: None,
        };
        model.update(Msg::LoadLocation);
        model
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadLocation => {
                // load location
                self.update(Msg::LoadWeather(Location::test_default()));
            }
            Msg::LoadWeather(location) => {
                let res = self.api.fetch(
                    &location.clone(),
                    self.link.callback_once(
                        move |response: Response<Json<Result<LocationWeather, Error>>>| {
                            if let (meta, Json(Ok(body))) = response.into_parts() {
                                if meta.status.is_success() {
                                    Msg::Fetched(location, body)
                                } else {
                                    Msg::Failed("Not success")
                                }
                            } else {
                                Msg::Failed("Json parse failed")
                            }
                        },
                    ),
                );
                if let Err(error) = res {
                    self.console
                        .error(format!("Error for requesting weather: {}", error).as_str());
                }
                self.console.log("Completed");
            }
            Msg::Fetched(location, response) => {
                self.view.replace(WeatherBoard::new(location, response));
            }
            Msg::Failed(info) => self.console.error(info),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {
                if let Some(view) = &self.view {
                    view.display()
                } else {
                    html!{}
                }
            }
            </div>
        }
    }
}
