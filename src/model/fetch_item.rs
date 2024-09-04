use crate::{interface::fetch::Fetch, model::company::Company};

pub struct FetchItem {
    pub company: Company,
    pub status: u16
}

impl FetchItem {
    // web_site scraper where impl fetch
    pub fn new<T:Fetch>(comapny: Company, site_scraper:T) -> Self {
        FetchItem { company:comapny, status:404 }
    }

    pub fn change_status(&mut self, new_status: u16) {
        self.status = new_status;
    }
}