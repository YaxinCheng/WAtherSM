use super::models::LocationWeather;
use super::proxy::proxy;
use super::source::Source;
use super::task_manage::TaskManage;
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
            source: Source::MetaWeather,
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
        woeid: u64,
        callback: Callback<Response<Json<Result<LocationWeather, Error>>>>,
    ) -> Result<(), Error> {
        let fetch_url = match self.source.support_cors() {
            true => self.source.weather_url(woeid),
            false => proxy(self.source.weather_url(woeid)),
        };
        let request = Request::get(fetch_url).body(Nothing)?;
        let task = self.service.fetch(request, callback)?;
        self.task_manage.store_weather_fetch(task);
        Ok(())
    }
}
