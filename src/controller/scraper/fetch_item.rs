use crate::model::company::Company;

pub struct FetchItem {
    pub company: Company,
    pub status: u16
}

impl FetchItem {
    pub fn new(comapny: Company) -> Self {
        FetchItem { company:comapny, status:404 }
    }
}