pub enum Source {
    MetaWeather,
}

impl Source {
    pub fn weather_url(&self, woeid: u64) -> String {
        use Source::*;
        match self {
            MetaWeather => format!("https://www.metaweather.com/api/location/{}", woeid),
        }
    }

    pub fn support_cors(&self) -> bool {
        use Source::*;
        match self {
            MetaWeather => false,
        }
    }
}
