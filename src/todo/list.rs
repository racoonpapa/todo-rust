use std::collections::HashMap;
use super::Item;

pub struct List {
    items: HashMap<String, Item>,
}

impl List {
    pub fn new() -> List {
        List {
            items: HashMap::new(),
        }
    }

    pub fn print(&self) {
        for (_, item) in &self.items {
            println!("{}", item);
        }
    }

    pub fn from_json(json_str: &str) -> Result<List, serde_json::Error> {
        let mut new_list = List::new();

        let items: Vec<Item> = serde_json::from_str(json_str)?;
        new_list.items.reserve(items.len());

        for item in items {
            new_list.items.insert(item.id.clone(), item);
        }
        Ok(new_list)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let values: Vec<Item> = self.items.values().cloned().collect();
        serde_json::to_string(&values)
    }
}
