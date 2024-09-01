use crate::model::company::Company;

pub struct FetchItem {
    pub company: Company,
    pub status: u16
}

impl FetchItem {
    pub fn new(comapny: Company) -> Self {
        FetchItem { company:comapny, status:404 }
    }

    pub fn change_status(&mut self, new_status: u16) {
        self.status = new_status;
    }
}