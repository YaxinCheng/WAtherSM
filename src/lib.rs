#![recursion_limit = "1024"]

mod api;
mod model;
mod util;
mod views;
use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen(start)]
pub fn start() {
    App::<model::Model>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
