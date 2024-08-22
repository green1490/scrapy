use clap::builder::styling::Style;
use ratatui::{crossterm::style::style, style::Color , style::Stylize, widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget}};
use crate::model::company::Company;

pub fn company_list(companies: &Vec<Company>) -> impl StatefulWidget<State = ListState> {
    let company_names:Vec<String> = companies.iter().map(|company| company.name.clone() ).collect();
    let list = List::new(company_names)
        .block(Block::bordered())
        // .highlight_style::<Style>(Style::new().into())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    list
}