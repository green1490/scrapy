use ratatui::{style::Stylize, widgets::{Paragraph, Widget}};

pub fn company_list() -> impl Widget {
    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
        .white()
        .on_blue()
}