mod model;
mod controller;
mod view;
mod interface;
mod site_scrapers;

use std::io::Error;
use controller::csv_reader::csv_reader;
use model::company::Company;
use view::main_view::main_view;

fn main() -> Result<(), Error> {
    // observer for the app state
    main_view()?;
    Ok(())
}
