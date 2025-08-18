use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget, WidgetRef},
};

use crate::{
    backend::{ComponentType, Module, RenderCallback, Store, SubRoutine},
    utils::{bool_from_optstr, create_borders},
};

pub fn create_renderer(ct: &ComponentType, store: Store, attributes: Store) -> RenderCallback {
    match ct {
        ComponentType::Column | ComponentType::Window | ComponentType::Row => Box::new(Layout {}),
        ComponentType::Text => Box::new(Text::new(store, attributes)),
        ComponentType::Plugin => Box::new(Plugin {}),
    }
}

pub fn get_subroutine(ct: &ComponentType) -> fn(&mut SubRoutine) {
    match ct {
        ComponentType::Column | ComponentType::Window | ComponentType::Row => Layout::subroutine,
        ComponentType::Text => Text::subroutine,
        ComponentType::Plugin => Plugin::subroutine,
    }
}

/* Layout */
struct Layout {}

impl Module for Layout {}

impl WidgetRef for Layout {
    fn render_ref(&self, _: Rect, _: &mut Buffer) {}
}

#[derive(Default)]
struct Text {
    store: Store,
    attributes: Store,
    borders: Borders,
}

impl Text {
    fn new(store: Store, attributes: Store) -> Self {
        let locked_attributes = attributes.lock().unwrap().clone();
        let borders = create_borders(locked_attributes.get("border"));

        Self {
            store,
            attributes,
            borders,
        }
    }
}

impl Module for Text {}

impl WidgetRef for Text {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lock = self.store.lock().unwrap();
        // store attr
        let text = lock.get("text");

        let text_widgets: Vec<Line> = text
            .map(|t| t.split('\n').map(Line::from).collect())
            .unwrap_or_default();

        let mut pg = Paragraph::new(text_widgets);
        pg = pg.block(Block::new().borders(self.borders));

        pg.render(area, buf);
    }
}

struct Plugin {}

impl Module for Plugin {}

impl WidgetRef for Plugin {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}
