use lscolors::{Indicator, LsColors, Style};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lscolors_env = if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        env::var("LS_COLORS")
    };
    if let Ok(le) = lscolors_env {
        let pieces: Vec<_> = le.split(":").collect();
        let lscolors = LsColors::from_string(&le);

        for entry in pieces {
            let name = entry.split("=").collect::<Vec<_>>()[0];
            let style = if entry.starts_with("*") {
                Some(lscolors.style_for_path(&name))
            } else {
                Indicator::from(name).and_then(|x| Some(lscolors.style_for_indicator(x)))
            };
            if let Some(s) = style {
                let ansi_style = s.map(Style::to_ansi_term_style).unwrap_or_default();
                print!("{} ", ansi_style.paint(entry));
            }
        }
    }
    println!(""); // final newline
}
