pub mod xmlparser;

use ratatui::{layout::Rect, widgets::Widget};
use tokio::sync::Mutex;
use std::{collections::BTreeMap, rc::Rc};

pub struct SubRoutine {
    store: Mutex<Rc<BTreeMap<String, String>>>,
    attributes: Mutex<Rc<BTreeMap<String, String>>>,
    routine: Box<dyn FnMut(Self)>
}

pub struct Component {
    area: Rc<Rect>,
    renderer: Box<dyn Fn(Rc<BTreeMap<String, String>>) -> dyn Widget>,
}

enum ComponentType {
    Row,
    Column,
    Window,
    Widget
}