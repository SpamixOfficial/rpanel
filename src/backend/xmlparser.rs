use color_eyre::{eyre::{Context, Error}, Result};
use ratatui::layout::Constraint;
use roxmltree::{Document, Node, ParsingOptions};

use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::backend::{ComponentType, RTRef, RenderTree, SubRoutine, modules::create_renderer};

pub struct Parser {
    components: Vec<RTRef>,
    subroutines: Vec<SubRoutine>,
    contents: String,
}

impl Parser {
    pub fn new<P: Into<PathBuf>>(p: P) -> Result<Self> {
        Ok(Self {
            components: vec![],
            subroutines: vec![],
            contents: fs::read_to_string(p.into())?,
        })
    }

    pub fn parse(mut self) -> Result<Self> {
        let opts = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };
        let contents_clone = self.contents.clone();
        let doc = Document::parse_with_options(&contents_clone, opts)?;

        if doc.root_element().tag_name().name() != "window" {
            panic!("Invalid root tag! (should be <window>)");
        }

        self.recurse(doc.root_element(), &doc, None)?;

        Ok(self)
    }

    fn recurse(&mut self, node: Node, doc: &Document, parent: Option<RTRef>) -> Result<()> {
        if node.tag_name().name() == "window" && doc.root_element() != node {
            return Err(Error::msg("Window is only allowed as a root tag."))
        }

        let (render_tree, subroutine, ct) = create_item(node)?;
        
        if let Some(s) = subroutine {
            self.subroutines.push(s);
        }

        // window tags are never pushed to the component lists
        if let Some(p) = parent {
            let mut lock = p.borrow_mut();

            if lock.ctype != ComponentType::Window {
                lock.children.push(render_tree.clone());
            } else {
                self.components.push(render_tree.clone());
            }
        }

        // only layouts can have children
        if ct.is_layout() {
            // this will never execute if there are no children, so no need for has_children() check
            for child in node.children() {
                if !child.is_element() {
                    continue;
                }
                self.recurse(child, doc, Some(render_tree.clone()))
                    .wrap_err_with(|| {
                        format!(
                            "Error while parsing at {}",
                            doc.text_pos_at(node.range().start)
                        )
                    })?;
            }
        }

        Ok(())
    }

    pub fn ret(self) -> Result<(Vec<Rc<RefCell<RenderTree>>>, Vec<SubRoutine>)> {
        Ok((self.components, self.subroutines))
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

fn create_item(node: Node) -> Result<(RTRef, Option<SubRoutine>, ComponentType)> {
    let t = node.tag_name().name();
    let ct = match t {
        "window" => ComponentType::Window,
        "column" => ComponentType::Column,
        "row" => ComponentType::Row,
        "text" => ComponentType::Text,
        _ => ComponentType::Plugin,
    };

    /* Setup */
    // map attributes and process core-attributes
    let mut pre_attributes: BTreeMap<String, String> = BTreeMap::new();
    for x in node.attributes() {
        pre_attributes.insert(x.name().to_string(), x.value().to_string());
    }

    // create clean data store
    let mut pre_store: BTreeMap<String, String> = BTreeMap::new();

    // collect text for text module
    if ct == ComponentType::Text {
        collect_text(node, &mut pre_store);
    }

    /* Properties */

    let size_constraint = size_from_attr(&ct, pre_attributes.get("size")).wrap_err_with(|| {
        format!(
            "Failed to parse attribute size \"{}\"",
            pre_attributes.get("size").unwrap(), // it is safe to unwrap here as it can only error if it is Some
        )
    })?;

    let store = Arc::new(Mutex::new(pre_store));
    let attributes = Arc::new(Mutex::new(pre_attributes.clone()));
    let renderer = create_renderer(&ct, store.clone(), attributes.clone());

    /* Final Object Creation */
    let rt = RenderTree {
        children: vec![],
        store: store.clone(),
        attributes: attributes,
        size_constraint,
        ctype: ct,
        renderer,
    };

    let sr: Option<SubRoutine> = None;

    Ok((Rc::new(RefCell::new(rt)), sr, ct))
}

fn collect_text(node: Node, store: &mut BTreeMap<String, String>) {
    let res = node
        .children()
        .map(|f| {
            if f.is_text() {
                f.text().unwrap_or("")
            } else {
                ""
            }
        })
        .collect::<Vec<&str>>()
        .join("");
    store.insert("text".to_string(), res);
}
