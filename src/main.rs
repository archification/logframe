mod common;
mod stuff;

use solarized::{
    print_colored, PrintMode::NewLine,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA
};
use stuff::{read_config, print_log_contents};
use std::sync::Arc;
use common::clear;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear();
    print_colored(
        &["Welcome ", "to ", "the ", "warframe ", "log ", "parser ", "of ", "doom."],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine
    );
    let config = read_config()?;
    let search_words = config.search_words.clone();
    let usernames = config.usernames.clone();
    let path = Arc::new(config.log_file_path);
    print_log_contents(path, search_words, usernames);
    Ok(())
}
