use super::fetch_item::FetchItem;

pub struct Fetcher {
    fetch_items: Vec<FetchItem>
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher { 
            fetch_items: vec![]
        }
    }

    pub fn add(mut self,url: String) -> Self {
        self.fetch_items.push(FetchItem::new(url));
        self
    }

    pub fn remove(&mut self, url:String) {
        for item in 0..self.fetch_items.len() {
            if self.fetch_items[item].url == url {
                let index = item - 1;
                self.fetch_items.swap_remove(index);
            }
        }
    }
}