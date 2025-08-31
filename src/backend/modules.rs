use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, WidgetRef},
};

use crate::{
    backend::{Attributes, ComponentType, Module, RenderCallback, Store, SubRoutine},
    utils::{create_borders, get_border_type, parse_from_attributes, read_opt_attributes},
};

pub fn create_renderer(
    ct: &ComponentType,
    store: Option<Store>,
    attributes: Attributes,
) -> RenderCallback {
    match ct {
        ComponentType::Column | ComponentType::Window | ComponentType::Row => {
            Box::new(Layout::new(attributes))
        }
        ComponentType::Text => Box::new(Text::new(attributes)),
        ComponentType::Block => Box::new(BlockComp::new(attributes)),
        ComponentType::Plugin => Box::new(Plugin {}),
    }
}

pub fn get_subroutine(ct: &ComponentType) -> fn(&mut SubRoutine) {
    match ct {
        ComponentType::Column | ComponentType::Window | ComponentType::Row => Layout::subroutine,
        ComponentType::Text => Text::subroutine,
        ComponentType::Block => BlockComp::subroutine,
        ComponentType::Plugin => Plugin::subroutine,
    }
}

struct BlockComp {
    fill: Option<Color>,
}

impl BlockComp {
    fn new(attributes: Attributes) -> Self {
        let lock = attributes.read();
        let fill = parse_from_attributes(lock.get("fill"));
        Self { fill }
    }
}

impl Module for BlockComp {}

impl WidgetRef for BlockComp {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new();

        if let Some(f) = self.fill {
            block = block.bg(f)
        };

        block.render(area, buf);
    }
}

/* Layout */
struct Layout {
    borders: Borders,
    btype: BorderType,
}

impl Layout {
    fn new(attributes: Attributes) -> Self {
        let lock = attributes.read();
        let borders = create_borders(lock.get("border"));
        let btype = get_border_type(lock.get("borderType"));

        Self { borders, btype }
    }
}

impl Module for Layout {}

impl WidgetRef for Layout {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().borders(self.borders).border_type(self.btype);
        block.render(area, buf);
    }
}

#[derive(Default)]
struct Text {
    attributes: Attributes,
}

impl Text {
    fn new(attributes: Attributes) -> Self {
        Self { attributes }
    }
}

impl Module for Text {}

impl WidgetRef for Text {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lock = self.attributes.read();
        // store attr
        let text = read_opt_attributes(lock.get("text"));

        // convert text into Lines
        let mut text_widgets: Vec<Line> = vec![];
        if let Some(t) = text {
            text_widgets = t.split('\n').map(|s| Line::from(s.to_string())).collect();
        }

        let pg = Paragraph::new(text_widgets);

        // useful for debugging
        //pg = pg.block(Block::bordered().border_style(Style::new().fg(Color::Green)));

        pg.render(area, buf);
    }
}

/// Mainly serves as a container, but also handles some of the attribute templating
struct Plugin {}

impl Module for Plugin {}

impl WidgetRef for Plugin {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}
