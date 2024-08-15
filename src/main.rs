mod model;
mod controller;

use controller::init::init;
use model::company::Company;
use csv::Error;
use csv::Reader;

fn main() {
    match init() {
        Ok(companies) => println!("success"),
        Err(error) => println!("error")
    }
}
