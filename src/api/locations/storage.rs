use another_radix_trie::RadixTrie;
use anyhow::Error;
use wasm_bindgen::__rt::std::collections::BTreeMap;
use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::Callback;

pub struct Storage {
    trie: RadixTrie<(String, usize)>,
    pub populated: bool,
    loading_task: Option<FetchTask>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            trie: RadixTrie::new(),
            populated: false,
            loading_task: None,
        }
    }

    pub fn load(&mut self, callback: Callback<Response<Result<Vec<u8>, Error>>>) {
        if self.populated {
            return;
        }
        let request = Request::get("city.list.json")
            .body(Nothing)
            .expect("Failed to create request");
        self.loading_task = FetchService::new().fetch_binary(request, callback).ok();
    }

    pub fn populates(&mut self, data: Vec<u8>) {
        if self.populated {
            return;
        }
        self.populated = true;
        let cities = serde_json::from_slice::<BTreeMap<String, usize>>(&data)
            .expect("Failed to deserialize");
        cities
            .into_iter()
            .for_each(|(name, id)| self.trie.insert(&name.to_lowercase(), (name, id)));
    }

    pub fn find(&self, name: &str) -> Vec<(String, usize)> {
        self.trie
            .start_with(&name.to_lowercase())
            .into_iter()
            .map(|(_, val)| val.to_owned())
            .collect::<Vec<_>>()
    }
}
