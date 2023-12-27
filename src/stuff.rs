use std::fs::{self, File};
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::path::PathBuf;
use toml;
use serde::Deserialize;
use solarized::{
    print_colored, print_fancy, PrintMode::NewLine,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};

#[derive(Deserialize)]
pub struct Config {
    pub log_file_path: PathBuf,
    pub search_words: Vec<String>,
    pub usernames: Vec<String>,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    print_colored(
        &["f", "i", "l", "e ", "r", "e", "a", "d", ": Config OK"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine
    );
    Ok(config)
}

pub fn print_log_contents(path: Arc<PathBuf>, search_words: Vec<String>, usernames: Vec<String>) {
    let mut reader = BufReader::new(File::open(&*path).unwrap());
    let last_pos = 0;
    loop {
        let mut file = File::open(&*path).unwrap();
        file.seek(SeekFrom::Start(last_pos)).unwrap();
        for line in (&mut reader).lines() {
            if let Ok(line) = line {
                let contains_killed = line.contains("killed");
                let contains_username = usernames.iter().any(|username| line.contains(username));
                if contains_killed && contains_username {
                    if let Some((before_killed, after_killed)) = line.split_once("killed") {
                        for username in &usernames {
                            if before_killed.contains(username) {
                                let busersplit = before_killed.split_once(username);
                                if let Some((before_username, after_username)) = busersplit {
                                    print_fancy(&[
                                        (before_username, CYAN, vec![]),
                                        (username, VIOLET, vec![BOLD]),
                                        (after_username, CYAN, vec![]),
                                        ("killed", RED, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after_killed, CYAN, vec![]),
                                    ], NewLine);
                                    break;
                                }
                            } else if after_killed.contains(username) {
                                let ausersplit = after_killed.split_once(username);
                                if let Some((before_username, after_username)) = ausersplit {
                                    print_fancy(&[
                                        (before_killed, CYAN, vec![]),
                                        ("killed", RED, vec![]),
                                        (before_username, CYAN, vec![]),
                                        (username, VIOLET, vec![BOLD]),
                                        (after_username, CYAN, vec![]),
                                    ], NewLine);
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    for word in &search_words {
                        if !contains_killed {
                            if let Some((before, after)) = line.split_once(word) {
                                print_fancy(&[
                                    (before, CYAN, vec![]),
                                    (word, GREEN, vec![BOLD, UNDERLINED, ITALIC]),
                                    (after, CYAN, vec![]),
                                ], NewLine);
                            }
                        }
                    }
                }
            }
        }
        let _last_pos = file.metadata().map(|m| m.len() as u64).unwrap_or(last_pos);
        thread::sleep(Duration::from_secs(1));
    }
}
