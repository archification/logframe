use std::fs::{self, File};
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::thread;
use std::time::Duration;
use std::sync::RwLock;
use std::path::PathBuf;
use toml;
use serde::Deserialize;
use crate::solarized::{print_colored, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};

lazy_static::lazy_static! {
    static ref LOG_FILE_PATH: RwLock<PathBuf> = RwLock::new(PathBuf::new());
}

#[derive(Deserialize)]
struct Config {
    log_file_path: PathBuf,
}

pub fn read_config() {
    let config_str = fs::read_to_string("config.toml")
        .expect("Config file cannot be read.");
    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse config file.");
    let mut path = LOG_FILE_PATH.write().unwrap();
    *path = config.log_file_path;
    print_colored(
        &["f", "i", "l", "e ", "r", "e", "a", "d", ": Config OK"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
}

pub fn print_log_contents() {
    let path = LOG_FILE_PATH.read().unwrap().clone();
    let mut reader = BufReader::new(File::open(&path).expect("Log file cannot be read."));
    thread::spawn(move || {
        let last_pos = 0;
        loop {
            let mut file = File::open(&path).expect("Unable to open log file.");
            file.seek(SeekFrom::Start(last_pos)).expect("Unable to seek log file.");
            for line in (&mut reader).lines() {
                if let Ok(line) = line {
                    if line.contains("killed") {
                        if let Some((before, after)) = line.split_once("killed") {
                            print_colored(
                                &[before, "killed", after],
                                &[CYAN, RED, CYAN]
                            );
                        }
                    }
                }
            }
            let _last_pos = file.metadata().map(|m| m.len() as u64).unwrap_or(last_pos);
            thread::sleep(Duration::from_secs(1));
        }
    });
    loop {}
}
