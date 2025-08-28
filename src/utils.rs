use std::{fmt::Debug, str::FromStr};

use ratatui::{
    layout::Flex,
    widgets::{BorderType, Borders},
};

pub fn bool_from_optstr(o: Option<&String>) -> bool {
    o.map(|b| b == "true").unwrap_or_default()
}
pub fn parse_from_attributes<T>(o: Option<&String>) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    o.map(|f| f.parse::<T>().unwrap())
}

pub fn create_borders(o: Option<&String>) -> Borders {
    let mut border = Borders::NONE;
    if let Some(s) = o {
        if s == "all" {
            return Borders::ALL;
        } else if s == "none" {
            return Borders::NONE;
        }

        s.split("").for_each(|p| {
            if !p.is_empty() {
                border |= border_from_part(p)
            }
        });
    }

    border
}

fn border_from_part(p: &str) -> Borders {
    match p {
        "r" => Borders::RIGHT,
        "l" => Borders::LEFT,
        "b" => Borders::BOTTOM,
        "t" => Borders::TOP,
        _ => Borders::NONE,
    }
}

pub fn get_border_type(o: Option<&String>) -> BorderType {
    if o.is_none() {
        return BorderType::Plain;
    }

    match o.unwrap().as_str() {
        "rounded" => BorderType::Rounded,
        "double" => BorderType::Double,
        "thick" => BorderType::Thick,
        "ultrathick" => BorderType::QuadrantOutside,
        _ => BorderType::Plain,
    }
}

pub fn flex_from_str(o: Option<&String>) -> Flex {
    if let Some(f) = o {
        match f.as_str() {
            "end" => Flex::End,
            "center" => Flex::Center,
            "spaceBetween" => Flex::SpaceBetween,
            "spaceAround" => Flex::SpaceAround,
            _ => Flex::Start,
        }
    } else {
        Flex::Start
    }
}
