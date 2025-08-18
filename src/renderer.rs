use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::WidgetRef,
};

use crate::backend::{ComponentType, RTRef};

pub struct Renderer {
    tree: Vec<RTRef>,
}

impl Renderer {
    pub fn new(tree: Vec<RTRef>) -> Self {
        Self { tree }
    }

    pub fn render(&self, frame: &mut Frame) {
        let builder = AreaBuilder::new(frame.area());

        // initial constraints
        let areas = Self::build_children_layout(ComponentType::Window, &self.tree, builder);

        for (i, t) in self.tree.clone().into_iter().enumerate() {
            Self::recurse_render(t, frame, areas[i]);
        }
    }

    fn recurse_render(tree: RTRef, frame: &mut Frame, area_builder: AreaBuilder) {
        let lock = tree.borrow();
        let children = lock.children.clone();
        let ctype = lock.ctype;

        if !ctype.is_layout() {
            area_builder.render_into_area(frame.buffer_mut(), &lock.renderer);
            return; // a module never has any children
        }

        if children.len() == 0 {
            return;
        }

        let areas: Vec<AreaBuilder> = Self::build_children_layout(ctype, &children, area_builder);

        for (i, child) in children.into_iter().enumerate() {
            Self::recurse_render(child, frame, areas[i]);
        }
    }

    fn build_children_layout(
        ctype: ComponentType,
        children: &Vec<RTRef>,
        area_builder: AreaBuilder,
    ) -> Vec<AreaBuilder> {
        let constraints: Vec<Constraint> = children
            .iter()
            .map(|f| f.borrow().size_constraint.clone())
            .collect();

        let areas = area_builder.layout(ctype.layout_direction(), constraints);

        areas
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
