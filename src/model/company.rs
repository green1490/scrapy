use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Company {
    name: String,
    urls: Vec<String>
}

impl Company {
    pub fn new(name:String, urls:Vec<String>) -> Self {
        Company{
            name,
            urls
        }
    }
}