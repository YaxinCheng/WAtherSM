use yew::services::Task;

pub(crate) struct TaskManage<T: Task> {
    weather_fetch: Option<T>,
    woeid_fetch: Option<T>,
}

impl<T: Task> Default for TaskManage<T> {
    fn default() -> Self {
        TaskManage {
            weather_fetch: None,
            woeid_fetch: None,
        }
    }
}

impl<T: Task> TaskManage<T> {
    pub fn store_weather_fetch(&mut self, task: T) {
        self.weather_fetch.replace(task);
    }

    pub fn store_woeid_fetch(&mut self, task: T) {
        self.woeid_fetch.replace(task);
    }
}
