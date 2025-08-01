use std::fs::{self, File};
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::sync::Arc;
use std::path::PathBuf;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use solarized::{
    print_colored, print_fancy, PrintMode::NewLine,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use crate::worldstate::worldstate;


#[derive(Deserialize)]
pub struct Config {
    pub log_file_path: PathBuf,
    pub search_words: Vec<String>,
    pub usernames: Vec<String>,
    pub enable_worldstate: bool,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    print_colored(
        &["f", "i", "l", "e ", "r", "e", "a", "d", ": Config OK"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine);
    Ok(config)
}

pub async fn print_log_contents(path: Arc<PathBuf>, search_words: Vec<String>, usernames: Vec<String>, enable_worldstate: bool) {
    let mut file = match File::open(&*path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening log file: {}", &e);
            return;
        }
    };
    let mut last_pos = file.seek(SeekFrom::End(0)).unwrap_or(0);
    loop {
        if file.metadata().map(|m| m.len()).unwrap_or(last_pos) < last_pos {
            last_pos = 0;
        }
        file.seek(SeekFrom::Start(last_pos)).unwrap();
        let reader = BufReader::new(&mut file);
        for line in reader.lines().map_while(Result::ok) {
            let contains_killed = line.contains("killed");
            let contains_username = usernames.iter().any(|username| line.contains(username));
            if contains_killed && contains_username {
                if let Some((before_killed, after_killed)) = line.split_once("killed") {
                    for username in &usernames {
                        if before_killed.contains(username) {
                            if let Some((before_username, after_username)) = before_killed.split_once(username) {
                                print_fancy(&[
                                    (before_username, CYAN, vec![]),
                                    (username, VIOLET, vec![BOLD]),
                                    (after_username, CYAN, vec![]),
                                    ("killed", RED, vec![BOLD, UNDERLINED, ITALIC]),
                                    (after_killed, CYAN, vec![]),
                                ], NewLine);
                                break;
                            }
                        } else if after_killed.contains(username) && let Some((before_username, after_username)) = after_killed.split_once(username) {
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
            } else {
                for word in &search_words {
                    if !contains_killed && let Some((before, after)) = line.split_once(word) {
                        print_fancy(&[
                            (before, CYAN, vec![]),
                            (word, GREEN, vec![BOLD, UNDERLINED, ITALIC]),
                            (after, CYAN, vec![]),
                        ], NewLine);
                    }
                }
            }
        }
        last_pos = file.stream_position().unwrap();
        if enable_worldstate && let Err(e) = worldstate().await {
            eprintln!("Error fetching worldstate: {}", e);
        }
        sleep(Duration::from_secs(1)).await;
    }
}
