mod modules;
pub mod xmlparser;

use color_eyre::eyre::{Error, Result};
use parking_lot::RwLock;
use ratatui::{
    layout::{Constraint, Direction},
    widgets::WidgetRef,
};

use std::{cell::RefCell, collections::BTreeMap, fmt, ops::Range, rc::Rc, sync::Arc};

pub type Store = Arc<RwLock<BTreeMap<String, String>>>;
pub type Attributes = Arc<RwLock<BTreeMap<String, Attribute>>>;
pub type RTRef = Rc<RefCell<RenderTree>>;
pub type RenderCallback = Box<dyn WidgetRef>;

/// `derive` will be `Some` when it is templated
#[derive(Clone, Debug)]
pub struct Attribute {
    value: String,
    derive: Option<AttrDerive>,
}

#[derive(Clone, Debug)]
pub struct AttrDerive {
    derive_from: String,
    store: Option<Store>,
    template_at: Range<usize>,
}

impl Attribute {
    pub fn create(value: String, store: Option<Store>) -> Attribute {
        let mut derive: Option<AttrDerive> = None;
        if let Some(derive_start) = value.find("{{")
            && let Some(derive_end) = value.get(derive_start..).and_then(|f| f.find("}}"))
        {
            if derive_start == 0 // cannot be escaped
                // if it is None or escaped we don't want to template
                || value.get(derive_start - 1..derive_start).map(|f| f != "\\") == Some(true)
            {
                let template_at = derive_start..derive_end + 1;
                let derive_from = value.get(derive_start + 2..derive_end).unwrap().to_string(); // should be safe now;
                derive = Some(AttrDerive {
                    derive_from,
                    template_at,
                    store
                })
            }
        }

        Self {
            value,
            derive,
        }
    }

    pub fn read(&self) -> Result<String> {
        let mut value = self.value.clone();

        if let Some(derive) = &self.derive && let Some(s) = &derive.store {
            let lock = s.read();
            let store_val = lock.get(&derive.derive_from);

            if store_val.is_none() {
                return Err(Error::msg(format!(
                    "Key \"{}\" does not exist in plugin store",
                    &derive.derive_from
                )));
            }

            value.replace_range(derive.template_at.clone(), store_val.unwrap());
        }

        Ok(value)
    }
}

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
    pub attributes: Attributes,
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
