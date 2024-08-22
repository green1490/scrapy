pub struct FetchItem {
    pub url: String,
    pub status: u16
}

impl FetchItem {
    pub fn new(url: String) -> Self {
        FetchItem { url, status: 404 }
    }
}