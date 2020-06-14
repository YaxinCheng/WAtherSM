use super::models::LocationWeather;
use super::proxy::proxy;
use super::source::Source;
use super::task_manage::TaskManage;
use crate::api::Location;
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::Callback;

pub struct WeatherAPI {
    source: Source,
    service: FetchService,
    task_manage: TaskManage<FetchTask>,
}

impl Default for WeatherAPI {
    fn default() -> Self {
        WeatherAPI {
            source: Source::OpenWeather,
            service: FetchService::new(),
            task_manage: TaskManage::default(),
        }
    }
}

impl WeatherAPI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch(
        &mut self,
        location: &Location,
        callback: Callback<Response<Json<Result<LocationWeather, Error>>>>,
    ) -> Result<(), Error> {
        let (lat, lon) = location.lat_lon();
        let fetch_url = match self.source.support_cors() {
            true => self.source.weather_url_by_lat_lon(lat, lon),
            false => proxy(self.source.weather_url_by_lat_lon(lat, lon)),
        };
        let request = Request::get(fetch_url).body(Nothing)?;
        let task = self.service.fetch(request, callback)?;
        self.task_manage.store_weather_fetch(task);
        Ok(())
    }
}
