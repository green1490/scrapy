use crate::model::company::Company;

use super::fetch_item::{self, FetchItem};

//storing as hashmap
pub struct Fetcher {
    fetch_items: Vec<FetchItem>
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher { 
            fetch_items: vec![]
        }
    }

    pub fn add(mut self,company: Company) -> Self {
        todo!();
    }

    pub fn remove(&mut self, url:String) {
        todo!("implement remove");
    }
}