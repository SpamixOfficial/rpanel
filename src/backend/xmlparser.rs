use anyhow::{Result, anyhow};
use ratatui::{layout::Constraint, widgets::Clear};
use roxmltree::{Document, Node, ParsingOptions};

use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::backend::{ComponentType, RTRef, RenderTree, Store, SubRoutine};

pub struct Parser {
    components: Vec<RTRef>,
    subroutines: Vec<SubRoutine>,
    contents: String,
    current_depth: usize,
}

impl Parser {
    pub fn new<P: Into<PathBuf>>(p: P) -> Result<Self> {
        return Ok(Self {
            components: vec![],
            subroutines: vec![],
            contents: fs::read_to_string(p.into())?,
            current_depth: 0,
        });
    }

    pub fn parse(&mut self) -> Result<&mut Self> {
        let opts = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };
        let contents_clone = self.contents.clone();
        let doc = Document::parse_with_options(&contents_clone, opts)?;

        self.recurse(doc.root_element(), None)?;
        dbg!(&self.components);
        return Ok(self);
    }

    fn recurse(&mut self, node: Node, parent: Option<RTRef>) -> Result<()> {
        let (render_tree, subroutine) = create_item(node)?;

        // parsing is only needed if it isn't the root
        if let Some(p) = parent {
            let ct = &render_tree.borrow().ctype;
            let mut lock = p.borrow_mut();

            println!("{:?}, {:?}", ct, lock.ctype);
            if lock.ctype != ComponentType::Window {
                lock.children.push(render_tree.clone());
            } else {
                self.components.push(render_tree.clone());
            }
        }

        // this will never execute if there are no children, so no need for has_children() check!
        for child in node.children() {
            if child.is_text() {
                continue;
            }
            self.recurse(child, Some(render_tree.clone()))?;
        }
        
        return Ok(());
    }

    pub fn ret(self) -> (Vec<Rc<RefCell<RenderTree>>>, Vec<SubRoutine>) {
        (self.components, self.subroutines)
    }
}

fn size_from_attr(ct: &ComponentType, attr: Option<&String>) -> Result<Constraint> {
    let val: Constraint;

    // custom sizes are only allowed for layouts, ignored otherwise
    if ct.is_layout()
        && let Some(sz) = attr
    {
        let ratios = sz.split_terminator("/").collect::<Vec<&str>>();
        // ratio (eg. 1/1)
        if ratios.len() == 2 {
            val = Constraint::Ratio(ratios[0].parse()?, ratios[1].parse()?);
        } else {
            // other type
            val = match sz.char_indices().nth_back(0) {
                Some((_, '%')) => Constraint::Percentage(sz.strip_suffix("%").unwrap().parse()?),
                Some((_, '/')) => Constraint::Fill(sz.strip_suffix("/").unwrap().parse()?),
                _ => Constraint::Length(sz.parse()?),
            };
        }
    } else {
        val = Constraint::Fill(1)
    }
    Ok(val)
}

fn create_item(node: Node) -> Result<(RTRef, Option<SubRoutine>)> {
    let t = node.tag_name().name();
    let ct = match t {
        "window" => ComponentType::Window,
        "column" => ComponentType::Column,
        "row" => ComponentType::Row,
        "plugin" => ComponentType::Plugin,
        _ => ComponentType::Normal,
    };

    // map attributes and process core-attributes
    let mut attributes: BTreeMap<String, String> = BTreeMap::new();
    for x in node.attributes() {
        attributes.insert(x.name().to_string(), x.value().to_string());
    }

    let size_constraint = match size_from_attr(&ct, attributes.get("size")) {
        Ok(x) => x,
        Err(e) => {
            return Err(anyhow!(
                "Failed to parse size \"{}\": {}",
                attributes.get("size").unwrap(), // it is safe to unwrap here as it can only error if it is Some
                e.to_string()
            ));
        }
    };

    // create clean data store
    let store: Store = Arc::new(Mutex::new(BTreeMap::new()));

    let rt = RenderTree {
        children: vec![],
        store_ref: store.clone(),
        size_constraint,
        ctype: ct,
        renderer: Some(Box::new(|_, _| Box::new(Clear))),
    };
    let sr: Option<SubRoutine> = None;

    Ok((Rc::new(RefCell::new(rt)), sr))
}
