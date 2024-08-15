use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Company {
    name: String,
    urls: Vec<String>
}