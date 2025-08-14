use anyhow::Result;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use ratatui::{layout::{Layout, Rect}, Frame};
use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

use crate::backend::{Component, ComponentType, SubRoutine};

pub struct Parser {
    components: Vec<Component>,
    subroutines: Vec<SubRoutine>,
    parser: Reader<BufReader<File>>,
}

impl Parser {
    pub fn new<P: Into<PathBuf>>(p: P) -> Result<Self> {
        return Ok(Self {
            components: vec![],
            subroutines: vec![],
            parser: Reader::from_file(p.into())?,
        });
    }

    pub fn parse(&mut self, frame: &Frame) -> Result<&mut Self> {
        let mut buf = Vec::new();
        let mut areas: Vec<Rect> = vec![frame.area()];
        let mut depth = 0;
        loop {
            let res = self.parser.read_event_into(&mut buf)?;

            match res {
                Event::Eof => break,
                Event::Start(e) => {
                    let t = self.handle_start(e, areas[depth])?;
                    depth += 1;
                },
                Event::End(_) => {
                    depth -= 1;
                }
                _ => (),
            }

            buf.clear();
        }

        return Ok(self)
    }

    fn handle_start(&mut self, e: BytesStart<'_>, area: Rect) -> Result<ComponentType> {
        let tag: &str = str::from_utf8(e.name().0)?;
        let mut attributes: BTreeMap<String, String> = BTreeMap::new();
        let mut t = ComponentType::Widget;

        match tag {
            "window" => { t = ComponentType::Window },
            _ => ()
        }

        return Ok(t);
    }

    fn create_new_area(parent: &Rect, ct: ComponentType) -> Rect {
        return match ct {
            ComponentType::Window => *parent,
            //ComponentType::Column => 
        }
    }

    pub fn ret(self) -> (Vec<Component>, Vec<SubRoutine>) {
        return (self.components, self.subroutines);
    }
}
