mod solarized;
mod stuff;

use solarized::{print_colored, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};
use stuff::{read_config, print_log_contents};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_colored(
        &["Welcome", "to", "the", "warframe", "log", "parser", "of", "doom."],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    let path = Arc::new(read_config()?);
    print_log_contents(path);
    Ok(())
}
