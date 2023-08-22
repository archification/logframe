mod solarized;
mod stuff;

use solarized::{print_colored, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};
use stuff::{read_config, print_log_contents};

fn main() {
    print_colored(
        &["Welcome", "to", "the", "warframe", "log", "parser", "of", "doom."],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    read_config();
    print_log_contents();
}
