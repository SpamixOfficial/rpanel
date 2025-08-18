use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Clear, Widget, WidgetRef},
};

use crate::backend::RTRef;

pub struct Renderer {
    tree: Vec<RTRef>,
}

impl Renderer {
    pub fn new(tree: Vec<RTRef>) -> Self {
        Self { tree }
    }

    pub fn render(&self, frame: &mut Frame) {
        let builder = AreaBuilder::new(frame.area());
        for t in self.tree.clone() {
            Self::recurse_render(t, frame, builder);
        }
    }

    fn recurse_render(tree: RTRef, frame: &mut Frame, area_builder: AreaBuilder) {
        let lock = tree.borrow();
        let children = lock.children.clone();
        
        // render if not layout
        if lock.ctype.is_layout() {
            if children.len() == 0 {
                return;
            }
        } else {
            area_builder.render_into_area(
                frame.buffer_mut(),
                &lock.renderer,
            );
            return; // a module never has any children
        }

        for child in children {
            Self::recurse_render(child, frame, area_builder);
        }
    }
}

#[derive(Clone, Copy)]
struct AreaBuilder {
    area: Rect,
}

impl AreaBuilder {
    fn new(area: Rect) -> Self {
        Self { area }
    }

    fn layout(self, direction: Direction, constraints: Vec<Constraint>) -> Vec<Self> {
        let res = Layout::default()
            .direction(direction)
            .constraints(constraints)
            .split(self.area);
        res.iter().map(|a| Self { area: *a }).collect::<Vec<Self>>()
    }

    fn render_into_area(&self, buf: &mut Buffer, widget: &Box<dyn WidgetRef>) {
        widget.render_ref(self.area, buf);
    }
}
