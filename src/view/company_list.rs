use ratatui::{style::{Style, Stylize}, widgets::{Block, List, ListState, Padding, StatefulWidget}};
use crate::model::company::Company;

pub fn company_list(companies: &Vec<Company>) -> impl StatefulWidget<State = ListState> {
    let company_names:Vec<String> = companies.iter().map(|company| company.name.clone() ).collect();
    let list = List::new(company_names)
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
        )
        .highlight_style(
            Style::new()
                .italic()
                .green()
                .underlined()
        )
        .repeat_highlight_symbol(true);
    list
}