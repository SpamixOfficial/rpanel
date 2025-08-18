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
        let areas = Self::build_children_layout(ComponentType::Window, &self.tree, builder, None);

        for (i, t) in self.tree.clone().into_iter().enumerate() {
            Self::recurse_render(t, frame, areas[i]);
        }
    }

    fn recurse_render(tree: RTRef, frame: &mut Frame, area_builder: AreaBuilder) {
        let lock = tree.borrow();
        let children = lock.children.clone();
        let ctype = lock.ctype;

        area_builder.render_into_area(frame.buffer_mut(), &lock.renderer);

        // a module never have any children
        if children.len() == 0 || !ctype.is_layout() {
            return;
        }

        // get margin for area
        let margin = lock
            .attributes
            .lock()
            .unwrap()
            .get("padding")
            .map(|f| f.parse::<u16>().unwrap());

        let areas: Vec<AreaBuilder> = Self::build_children_layout(ctype, &children, area_builder, margin);

        for (i, child) in children.into_iter().enumerate() {
            Self::recurse_render(child, frame, areas[i]);
        }
    }

    fn build_children_layout(
        ctype: ComponentType,
        children: &Vec<RTRef>,
        area_builder: AreaBuilder,
        margin: Option<u16>
    ) -> Vec<AreaBuilder> {
        let constraints: Vec<Constraint> = children
            .iter()
            .map(|f| f.borrow().size_constraint.clone())
            .collect();

        let areas = area_builder.layout(ctype.layout_direction(), constraints, margin);

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

    fn layout(
        self,
        direction: Direction,
        constraints: Vec<Constraint>,
        margin: Option<u16>,
    ) -> Vec<Self> {
        let res = Layout::default()
            .margin(margin.unwrap_or(0))
            .direction(direction)
            .constraints(constraints)
            .split(self.area);
        res.iter().map(|a| Self { area: *a }).collect::<Vec<Self>>()
    }

    fn render_into_area(&self, buf: &mut Buffer, widget: &Box<dyn WidgetRef>) {
        widget.render_ref(self.area, buf);
    }
}
