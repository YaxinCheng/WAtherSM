use crate::api::{LocationWeather, WeatherAPI};
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
    Add,
    Fetched(LocationWeather),
    Failed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            console: ConsoleService::new(),
            api: WeatherAPI::new(),
            view: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.console.log("Add");
                let res = self.api.fetch(
                    44418,
                    self.link.callback(
                        |response: Response<Json<Result<LocationWeather, Error>>>| {
                            if let (meta, Json(Ok(body))) = response.into_parts() {
                                if meta.status.is_success() {
                                    Msg::Fetched(body)
                                } else {
                                    Msg::Failed
                                }
                            } else {
                                Msg::Failed
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
            Msg::Fetched(response) => {
                self.view.replace(WeatherBoard::new(response));
            }
            Msg::Failed => self.console.log("Failed"),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
        <>
            <div>
                <button onclick=self.link.callback(|_| Msg::Add)>{ "+1" }</button>
            </div>
            <div>
            {
                if let Some(view) = &self.view {
                    view.display()
                } else {
                    html!{}
                }
            }
            </div>
        </>
        }
    }
}
