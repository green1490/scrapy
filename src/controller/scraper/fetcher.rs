use std::collections::HashMap;
use crate::model::company::Company;
use super::fetch_item::FetchItem;


pub struct Fetcher {
    fetch_items: HashMap<String, FetchItem>
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            fetch_items: HashMap::new()
        }
    }

    pub fn add(mut self,company: Company) -> Self {
        self.fetch_items.insert(
            company.name.clone(),
            FetchItem::new(company)
        );
        self
    }

    pub fn remove(&mut self, company_name:String) {
        self.fetch_items.remove(&company_name);
    }
}