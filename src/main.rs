use anyhow::Result;
use backend::{RenderTree, SubRoutine, xmlparser};
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::path::PathBuf;

mod backend;

#[derive(Default)]
struct App {
    components: Vec<RenderTree>,
    routines: Vec<SubRoutine>,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");
    xmlparser::Parser::new("demo.xml").unwrap().parse().unwrap();

    /*let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(frame.area());
            frame.render_widget(
                Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
                layout[0],
            );
            frame.render_widget(
                Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL)),
                layout[1],
            );
        })?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();*/
    return Ok(());
}

async fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    /*loop {
        terminal.draw(render_callback)
    }*/
    return Ok(());
}
