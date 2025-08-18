use std::process::exit;

use ratatui::widgets::Borders;

pub fn bool_from_optstr(o: Option<&String>) -> bool {
    o.map(|b| b == "true").unwrap_or_default()
}

pub fn create_borders(o: Option<&String>) -> Borders {
    let mut border = Borders::NONE;
    if let Some(s) = o {
        if s == "all" {
            return Borders::ALL;
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
        _ => {
            eprintln!("{} is not a valid border specifier", p);
            exit(1);
        }
    }
}
