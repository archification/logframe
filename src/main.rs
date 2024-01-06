mod common;
mod stuff;

use std::env;
use solarized::{
    print_colored, print_fancy, PrintMode::NewLine,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA, GREY
};
use stuff::{read_config, print_log_contents};
use std::sync::Arc;
use common::clear;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear();
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        print_fancy(&[
            ("config.toml", BLUE, vec![]),
            (" file has these three(3) fields.\n", CYAN, vec![]),
            ("log_file_path", GREY, vec![]),
            (" indicates the full path to the ", CYAN, vec![]),
            ("EE.log", BLUE, vec![]),
            (" file.\n", CYAN, vec![]),
            ("search_words", GREEN, vec![]),
            (" indicates which words to search for in each line.\n", CYAN, vec![]),
            ("usernames", VIOLET, vec![]),
            (" indicates which usernames to search for in each line.\n", CYAN, vec![]),
        ], NewLine);
    } else {
        print_colored(
            &["Welcome ", "to ", "the ", "warframe ", "log ", "parser ", "of ", "doom."],
            &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
            NewLine
        );
        let config = read_config()?;
        let search_words = config.search_words.clone();
        let usernames = config.usernames.clone();
        let path = Arc::new(config.log_file_path);
        let enable_daynight = config.enable_daynight.clone();
        print_log_contents(path, search_words, usernames, enable_daynight);
    }
    Ok(())
}
