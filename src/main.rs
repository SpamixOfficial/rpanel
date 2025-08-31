use backend::{RTRef, RenderTree, SubRoutine, xmlparser};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use std::{cell::RefCell, rc::Rc};

use crate::renderer::Renderer;

mod backend;
mod renderer;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let (render_tree, subroutines) = xmlparser::Parser::new("demo.xml")?.parse()?.ret()?;
    dbg!(&render_tree);

    // UI can be synchronous, making it async makes no sense whatsoever
    run(render_tree)?;
    ratatui::restore();
    Ok(())
}

fn run(rt: Vec<RTRef>) -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let renderer = Renderer::new(rt);

    loop {
        terminal.draw(|frame| renderer.render(frame))?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }

    Ok(())
}
