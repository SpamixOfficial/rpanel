use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, WidgetRef},
};

use crate::{
    backend::{ComponentType, Module, RenderCallback, Store, SubRoutine},
    utils::{bool_from_optstr, create_borders, get_border_type},
};

pub fn create_renderer(ct: &ComponentType, store: Store, attributes: Store) -> RenderCallback {
    match ct {
        ComponentType::Column | ComponentType::Window | ComponentType::Row => Box::new(Layout::new(attributes)),
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
struct Layout {
    borders: Borders,
    btype: BorderType
}

impl Layout {
    fn new(attributes: Store) -> Self {
        let lock = attributes.lock().unwrap().clone();
        let borders = create_borders(lock.get("border"));
        let btype = get_border_type(lock.get("borderType"));

        Self {
            borders,
            btype
        }
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
    store: Store,
    attributes: Store,
}

impl Text {
    fn new(store: Store, attributes: Store) -> Self {
        let locked_attributes = attributes.lock().unwrap().clone();

        Self {
            store,
            attributes,
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

        let pg = Paragraph::new(text_widgets);

        pg.render(area, buf);
    }
}

struct Plugin {}

impl Module for Plugin {}

impl WidgetRef for Plugin {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}
