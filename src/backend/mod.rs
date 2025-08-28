mod modules;
pub mod xmlparser;

use parking_lot::RwLock;
use ratatui::{
    layout::{Constraint, Direction, Flex},
    widgets::WidgetRef,
};

use std::{cell::RefCell, collections::BTreeMap, fmt, rc::Rc, sync::Arc};

pub type Store = Arc<RwLock<BTreeMap<String, String>>>;
pub type RTRef = Rc<RefCell<RenderTree>>;
pub type RenderCallback = Box<dyn WidgetRef>;

pub trait Module {
    fn subroutine(routine: &mut SubRoutine) {}
}

pub struct SubRoutine {
    store: Store,
    attributes: Store,
    routine: fn(&mut Self),
}

pub struct RenderTree {
    pub children: Vec<RTRef>,
    pub store: Option<Store>,
    pub attributes: Store,
    pub size_constraint: Constraint,
    pub ctype: ComponentType,
    pub renderer: RenderCallback,
}

impl fmt::Debug for RenderTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderTree")
            .field("type", &self.ctype)
            .field("size", &self.size_constraint)
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .field("store", &self.store)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ComponentType {
    Row,
    Column,
    Window,
    Text,
    Block,
    Plugin,
}

impl ComponentType {
    pub fn from_tag(tag: &str) -> Self {
        match tag {
            "window" => ComponentType::Window,
            "column" => ComponentType::Column,
            "row" => ComponentType::Row,
            "text" => ComponentType::Text,
            "block" => ComponentType::Block,
            _ => ComponentType::Plugin,
        }
    }

    pub fn is_layout(&self) -> bool {
        matches!(
            self,
            ComponentType::Window | ComponentType::Column | ComponentType::Row
        )
    }

    pub fn layout_direction(&self) -> Direction {
        // this should never panic but if it does we know why
        assert!(self.is_layout());

        if *self == ComponentType::Row {
            Direction::Horizontal
        } else {
            Direction::Vertical
        }
    }
}