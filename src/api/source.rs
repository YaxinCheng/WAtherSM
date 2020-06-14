pub enum Source {
    MetaWeather,
    OpenWeather,
}

impl Source {
    pub fn weather_url(&self, woeid: u64) -> String {
        use Source::*;
        match self {
            MetaWeather => format!("https://www.metaweather.com/api/location/{}", woeid),
            OpenWeather => unimplemented!("Not able to use woeid"),
        }
    }

    pub fn weather_url_by_lat_lon(&self, lat: f32, lon: f32) -> String {
        use Source::*;
        match self {
            MetaWeather => unimplemented!("Unable to use lat lon"),
            OpenWeather => format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric", 
                                   lat, lon, include_str!("../../.apikey")),
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
