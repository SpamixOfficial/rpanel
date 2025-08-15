use backend::{RenderTree, SubRoutine, xmlparser};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::{cell::RefCell, process::exit, rc::Rc};

mod backend;

#[derive(Default)]
struct App {
    components: Vec<Rc<RefCell<RenderTree>>>,
    routines: Vec<SubRoutine>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let (render_tree, subroutines) = xmlparser::Parser::new("demo.xml")?.parse()?.ret()?;
    dbg!(render_tree);
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
