use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Company {
    name: String,
    urls: Vec<String>
}