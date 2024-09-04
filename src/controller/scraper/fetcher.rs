use std::collections::HashMap;
use crate::model::{company::Company, fetch_item::FetchItem};


pub struct Fetcher {
    fetch_items: HashMap<String, FetchItem>
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            fetch_items: HashMap::new()
        }
    }

    // pub fn add(mut self,company: Company) -> Self {
    //     self.fetch_items.insert(
    //         company.name.clone(),
    //         FetchItem::new(company)
    //     );
    //     self
    // }

    // removes if the status is 404
    // observer?
    fn remove(&mut self, company_name: &str) {
        self.fetch_items.remove(company_name);
    }
    
    // fn fetch(company_name: &str) -> String {
    //     fetch_items[company_name].fetch() -> match and if None remove from hashmap
    // }
}