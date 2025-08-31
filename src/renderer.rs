use std::collections::BTreeMap;

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    widgets::WidgetRef,
};

use crate::{
    backend::{Attribute, ComponentType, RTRef},
    utils::{flex_from_str, parse_from_attributes},
};

#[derive(Default)]
struct LayoutProperties {
    margin: u16,
    spacing: u16,
    flex: Flex,
}

impl LayoutProperties {
    fn new() -> Self {
        Self::default()
    }

    fn margin(mut self, margin: Option<u16>) -> Self {
        self.margin = margin.unwrap_or(0);
        self
    }

    fn spacing(mut self, spacing: Option<u16>) -> Self {
        self.spacing = spacing.unwrap_or(0);
        self
    }

    fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    fn from_attributes(a: &BTreeMap<String, Attribute>) -> Self {
        Self::new()
            .margin(parse_from_attributes(a.get("padding")))
            .flex(flex_from_str(a.get("flex")))
            .spacing(parse_from_attributes(a.get("spacing")))
    }
}

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
        let areas = Self::build_children_layout(
            ComponentType::Window,
            &self.tree,
            builder,
            LayoutProperties::new()
        );

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

        let attributes_read = lock.attributes.read();

        // get properties for layout
        let props = LayoutProperties::from_attributes(&attributes_read);

        let areas: Vec<AreaBuilder> =
            Self::build_children_layout(ctype, &children, area_builder, props);

        for (i, child) in children.into_iter().enumerate() {
            Self::recurse_render(child, frame, areas[i]);
        }
    }

    fn build_children_layout(
        ctype: ComponentType,
        children: &Vec<RTRef>,
        area_builder: AreaBuilder,
        layout_properties: LayoutProperties
    ) -> Vec<AreaBuilder> {
        let constraints: Vec<Constraint> = children
            .iter()
            .map(|f| f.borrow().size_constraint.clone())
            .collect();

        let areas = area_builder.layout(ctype.layout_direction(), constraints, layout_properties);

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
        props: LayoutProperties,
    ) -> Vec<Self> {
        let res = Layout::default()
            .margin(props.margin)
            .flex(props.flex)
            .spacing(props.spacing)
            .direction(direction)
            .constraints(constraints)
            .split(self.area);

        res.iter().map(|a| Self { area: *a }).collect::<Vec<Self>>()
    }

    fn render_into_area(&self, buf: &mut Buffer, widget: &Box<dyn WidgetRef>) {
        widget.render_ref(self.area, buf);
    }
}
