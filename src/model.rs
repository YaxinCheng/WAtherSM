use crate::api::locations::Storage;
use crate::api::{LocationWeather, WeatherAPI};
use crate::views::{View, WeatherBoard};
use anyhow::Error;
use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;
use yew::format::Json;
use yew::services::console::ConsoleService;
use yew::services::fetch::Response;
use yew::services::storage::Area;
use yew::services::StorageService;
use yew::utils;
use yew::{html, Component, ComponentLink, Html, InputData, KeyboardEvent};

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    storage: Option<StorageService>,
    weather_api: WeatherAPI,
    location_api: Storage,
    view: Option<WeatherBoard>,
    suggestions: Vec<(String, usize)>,
}

pub enum Msg {
    LoadLocation,
    LoadSearchBar,
    LoadWeather(String, usize),
    Fetched(String, LocationWeather),
    Search(String),
    Failed(String),
    PlayVideo,
    Ignored,
}

static LOCATION_KEY: &str = "location";

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,
            console: ConsoleService::new(),
            storage: StorageService::new(Area::Local).ok(),
            weather_api: WeatherAPI::new(),
            location_api: Storage::new(),
            view: None,
            suggestions: vec![],
        };
        model.update(Msg::LoadLocation);
        model
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadLocation => {
                // load location
                let mut msg_title = String::from("Toronto");
                let mut msg_id = 6167865_usize;
                if let Some(location) = self
                    .storage
                    .as_ref()
                    .and_then(|storage| storage.restore::<Result<String, Error>>(LOCATION_KEY).ok())
                {
                    let mut split = location.splitn(2, "|");
                    let title = split.next();
                    let id = split.next().and_then(|id| id.parse::<usize>().ok());
                    match (title, id) {
                        (Some(title), Some(id)) => {
                            msg_title = title.to_owned();
                            msg_id = id;
                        }
                        _ => (),
                    }
                }
                self.link.send_message(Msg::LoadWeather(msg_title, msg_id));
            }
            Msg::LoadWeather(title, id) => {
                self.suggestions.clear();
                if let Some(storage) = self.storage.as_mut() {
                    storage.store(LOCATION_KEY, Ok(format!("{}|{}", title, id)));
                }
                let res = self.weather_api.fetch(
                    id,
                    self.link.callback_once(
                        move |response: Response<Json<Result<LocationWeather, Error>>>| {
                            let (meta, Json(res)) = response.into_parts();
                            if meta.status.is_success() {
                                match res {
                                    Ok(body) => Msg::Fetched(title, body),
                                    Err(error) => Msg::Failed(format!("{}", error)),
                                }
                            } else {
                                Msg::Failed("Response failed".to_owned())
                            }
                        },
                    ),
                );
                if let Err(error) = res {
                    self.console
                        .error(format!("Error for requesting weather: {}", error).as_str());
                }
                return true;
            }
            Msg::Fetched(location, response) => {
                self.view.replace(WeatherBoard::new(location, response));
                self.link.send_message(Msg::PlayVideo);
                return true;
            }
            Msg::Search(city) => {
                self.console.log(&format!("Search: {}", city));
                if city.len() < 3 {
                    self.suggestions.clear();
                } else {
                    self.suggestions = self.location_api.find(&city).into_iter().collect();
                }
                return true;
            }
            Msg::LoadSearchBar => self.location_api.populates(),
            Msg::Failed(info) => self.console.error(&info),
            Msg::PlayVideo => {
                if let Some(element) = utils::document()
                    .get_element_by_id("background")
                    .and_then(|element| element.dyn_into::<HtmlMediaElement>().ok())
                {
                    element.set_muted(true);
                    element.load();
                    let _ = element.play();
                }
            }
            Msg::Ignored => (),
        };
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let list = self.suggestions.clone().into_iter().map(move |(title, id)| {
            let cloned_title = title.clone();
            html! {
                <li onclick=self.link.callback_once(move |_| Msg::LoadWeather(cloned_title, id))>{ &title }</li>
            }
        });
        html! {
        <>
            {
                if let Some(view) = &self.view {
                    view.display()
                } else {
                    html!{}
                }
            }
            <div id="searchBarArea">
                <input id="searchBar"
                    placeholder="Find your city here"
                    onclick=self.link.callback(|_| Msg::LoadSearchBar)
                    oninput=self.link.callback(|e: InputData| Msg::Search(e.value))
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        Msg::Ignored
                    })
                />
                <ul id="suggestions">
                { for list }
                </ul>
            </div>
        </>
        }
    }
}
