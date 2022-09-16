use lscolors::{Indicator, LsColors, Style};
use std::env;

fn main() {
    // TODO: get input from cli args
    let lscolors_env = env::var("LS_COLORS");
    if let Ok(le) = lscolors_env {
        let pieces: Vec<_> = le.split(":").collect();
        let lscolors = LsColors::from_string(&le);

        for p in pieces {
            if p.starts_with("*") {
                let path = "file".to_string()
                    + &p[1..].to_string().split("=").collect::<Vec<_>>()[0];
                let style = lscolors.style_for_path(&path);

                let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
                print!("{}  ", ansi_style.paint(path));
            } else {
                let indicator_str = p.split("=").collect::<Vec<_>>()[0];
                let indicator = Indicator::from(indicator_str);
                if let Some(ind) = indicator {
                    let path = indicator_str;
                    let style = lscolors.style_for_indicator(ind);

                    let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
                    print!("{}  ", ansi_style.paint(path));
                }
            }
        }
    }
}
