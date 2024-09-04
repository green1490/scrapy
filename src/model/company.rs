use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Company {
    pub name: String,
    pub url: String
}

impl Company {
    pub fn new(name:String, url: String) -> Self {
        Company{
            name,
            url
        }
    }
}

