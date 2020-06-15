use super::models;
use another_radix_trie::RadixTrie;

pub struct Storage {
    trie: RadixTrie<(String, usize)>,
    populated: bool,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            trie: RadixTrie::new(),
            populated: false,
        }
    }

    pub fn populates(&mut self) {
        if self.populated {
            return;
        }
        self.populated = true;
        let cities = models::bunch_load().expect("Failed to load cities");
        cities.into_iter().for_each(|city| {
            let full_name = city.full_name();
            let id = city.id;
            self.trie.insert(&full_name.to_lowercase(), (full_name, id));
        });
    }

    pub fn find(&self, name: &str) -> Vec<(String, usize)> {
        self.trie
            .start_with(&name.to_lowercase())
            .into_iter()
            .map(|(_, val)| val.to_owned())
            .collect::<Vec<_>>()
    }
}
