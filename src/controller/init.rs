use std::io;
use crate::Company;

pub fn init() -> Result<Vec<Company>,io::Error> {
    const companies: Vec<Company> = Vec::new();
    let mut rdr = csv::Reader::from_path("src/data/pest.csv")?;
        for record in rdr.records() {
            let result = record?;
            println!("{}", result.get(1).unwrap());
        }
    return Ok(companies)
}
