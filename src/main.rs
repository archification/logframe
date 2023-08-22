mod solarized;
mod stuff;

use crossterm::style::{SetBackgroundColor, SetForegroundColor, ResetColor};
use solarized::{BACK, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};
use stuff::{read_config, print_log_contents};

fn main() {
    println!(
        "{}{}Welcome {}to {}the {}warframe {}log {}parser {}of {}doom.{}",
        SetBackgroundColor(BACK),
        SetForegroundColor(VIOLET),
        SetForegroundColor(BLUE),
        SetForegroundColor(CYAN),
        SetForegroundColor(GREEN),
        SetForegroundColor(YELLOW),
        SetForegroundColor(ORANGE),
        SetForegroundColor(RED),
        SetForegroundColor(MAGENTA),
        ResetColor
    );
    read_config();
    print_log_contents();
}
