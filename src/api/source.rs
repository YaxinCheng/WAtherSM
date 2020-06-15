pub enum Source {
    #[allow(dead_code)]
    MetaWeather,
    OpenWeather,
}

impl Source {
    #[allow(dead_code)]
    pub fn weather_url(&self, woeid: u64) -> String {
        use Source::*;
        match self {
            MetaWeather => format!("https://www.metaweather.com/api/location/{}", woeid),
            OpenWeather => unimplemented!("Not able to use woeid"),
        }
    }

    pub fn weather_url_by_id(&self, id: usize) -> String {
        use Source::*;
        match self {
            MetaWeather => unimplemented!("Unable to use lat lon"),
            OpenWeather => format!(
                "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}&units=metric",
                id,
                include_str!("../../.apikey")
            ),
        }
    }

    pub fn support_cors(&self) -> bool {
        use Source::*;
        match self {
            MetaWeather => false,
            OpenWeather => true,
        }
    }
}
