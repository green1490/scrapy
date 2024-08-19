use std::io;
use crate::Company;

pub fn csv_reader() -> Result<Vec<Company>,io::Error> {
    let mut companies: Vec<Company> = Vec::new();
    let mut rdr = csv::Reader::from_path("src/data/pest.csv")?;
    let record_iter = rdr.records();
    // error if there is missing field
    for record in record_iter {
        let company = record?.deserialize::<Company>(None)?;
        companies.push(company);
    }
    return Ok(companies)
}
