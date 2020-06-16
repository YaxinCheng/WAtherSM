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
    Shade,
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
            Msg::LoadLocation => self.load_location(),
            Msg::LoadWeather(title, id) => return self.load_weather(title, id),
            Msg::Fetched(location, response) => return self.display_weather(location, response),
            Msg::Search(city) => return self.search_city(&city),
            Msg::LoadSearchBar => self.location_api.populates(),
            Msg::Failed(info) => self.console.error(&info),
            Msg::PlayVideo => self.play_video(),
            Msg::Shade => return self.shade_views(),
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
                self.view.as_ref()
                    .and_then(|board| board.background.as_ref())
                    .map(|background| background.display())
                    .unwrap_or(html!{})
            }
            <div id="weatherPanel">
            {
                self.view.as_ref()
                    .map(|board| &board.today)
                    .map(|today| today.display())
                    .unwrap_or(html!{})
            }
            <div id="buttonLine">
            <button id="shade" onclick=self.link.callback(|_| Msg::Shade)>{ "🔼️" }</button>
            <button id="sync" onclick=self.link.callback(|_| Msg::LoadLocation)>{ "🔄" }</button>
            </div>
            </div>
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

impl Model {
    fn shade_views(&mut self) -> bool {
        let class = "shaded";
        let targeted_ids = ["shade", "today", "sync"];
        for id in &targeted_ids {
            if let Some(element) = utils::document().get_element_by_id(id) {
                let class_list = element.class_list();
                if class_list.contains(class) {
                    let _ = class_list.remove_1(class);
                } else {
                    element.set_class_name(class);
                }
            }
        }
        true
    }

    fn play_video(&mut self) {
        if let Some(element) = utils::document()
            .get_element_by_id("background")
            .and_then(|element| element.dyn_into::<HtmlMediaElement>().ok())
        {
            element.set_muted(true);
            element.load();
            let _ = element.play();
        }
    }

    fn search_city(&mut self, city: &str) -> bool {
        if city.len() < 3 {
            self.suggestions.clear();
        } else {
            self.suggestions = self.location_api.find(&city).into_iter().collect();
        }
        true
    }

    fn display_weather(&mut self, title: String, weather: LocationWeather) -> bool {
        let window = utils::window();
        let portrait = match (window.inner_height(), window.inner_width()) {
            (Ok(height), Ok(width)) => {
                height.as_f64().unwrap_or_default() > width.as_f64().unwrap_or_default()
            }
            _ => false,
        };
        self.view
            .replace(WeatherBoard::new(title, weather, portrait));
        self.link.send_message(Msg::PlayVideo);
        return true;
    }

    fn load_weather(&mut self, city_name: String, id: usize) -> bool {
        self.suggestions.clear();
        if let Some(storage) = self.storage.as_mut() {
            storage.store(LOCATION_KEY, Ok(format!("{}|{}", city_name, id)));
        }
        let res = self.weather_api.fetch(
            id,
            self.link.callback_once(
                move |response: Response<Json<Result<LocationWeather, Error>>>| {
                    let (meta, Json(res)) = response.into_parts();
                    if meta.status.is_success() {
                        match res {
                            Ok(body) => Msg::Fetched(city_name, body),
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

    fn load_location(&mut self) {
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
}
