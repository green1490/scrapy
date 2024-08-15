mod model;
use model::company::Company;
use csv::Error;
use csv::Reader;

fn main() -> Result<(),Error> {
    let mut rdr = csv::Reader::from_path("src/data/pest.csv")?;
    for record in rdr.records() {
        let result = record?;
        println!("{}", result.get(1).unwrap());
    }
    Ok(())
}
