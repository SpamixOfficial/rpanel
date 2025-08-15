pub mod xmlparser;

use ratatui::{
    layout::{Constraint, Rect},
    widgets::Widget,
};

use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

type Store = Arc<Mutex<BTreeMap<String, String>>>;
type RTRef = Rc<RefCell<RenderTree>>;

pub struct SubRoutine {
    store: Store,
    attributes: Store,
    routine: Box<dyn FnMut(&mut Self)>,
}

pub struct RenderTree {
    children: Vec<RTRef>,
    store_ref: Store,
    size_constraint: Constraint,
    ctype: ComponentType,
    renderer: Option<Box<dyn Fn(Store, Rect) -> Box<dyn Widget>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ComponentType {
    Row,
    Column,
    Window,
    Plugin,
    Normal,
}

impl ComponentType {
    pub fn is_layout(&self) -> bool {
        matches!(
            self,
            ComponentType::Window | ComponentType::Column | ComponentType::Row
        )
    }
}
