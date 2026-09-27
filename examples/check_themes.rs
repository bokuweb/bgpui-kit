//! Validate host-owned theme files against the shared token contract.

use bgpui_kit::ThemeTokens;
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let paths: Vec<_> = env::args().skip(1).collect();
    if paths.is_empty() {
        return Err("pass one or more theme JSON paths".into());
    }

    for path in paths {
        let theme = ThemeTokens::parse(&fs::read_to_string(&path)?)?;
        println!("{path}: {} ({})", theme.name, theme.appearance);
    }
    Ok(())
}
