use serde::Deserialize;
use serde_json::Result;

pub fn bunch_load() -> Result<Vec<City>> {
    let json_bytes = include_bytes!("../../../static/city.list.json");
    serde_json::from_slice(json_bytes)
}

#[derive(Deserialize)]
pub struct City {
    pub id: usize,
    name: String,
    state: String,
    country: String,
}

impl City {
    pub fn full_name(&self) -> String {
        let mut builder = vec![self.name.as_str()];
        if !self.state.is_empty() {
            builder.push(&self.state);
        }
        builder.push(&self.country);
        builder.join(",")
    }
}

#[cfg(test)]
mod bunch_load_tests {
    use crate::api::locations::models::bunch_load;

    #[test]
    fn test_bunch_load() {
        let result = bunch_load().expect("Failed to load");
        assert_eq!(result.len(), 209579);
    }
}
