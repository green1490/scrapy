use ratatui::{style::Stylize, widgets::{List, Paragraph, Widget}};
use crate::model::company::Company;

pub fn company_list(companies: &Vec<Company>) -> impl Widget {
    // let company_names:Vec<String> = companies.iter().map(|company| company.name ).collect();
    let item = ["test1"];
    let list = List::new(item);
    list
}