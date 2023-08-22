use std::fs::{self, File};
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::thread;
use std::time::Duration;
use std::sync::RwLock;
use toml;
use serde::Deserialize;
use crossterm::style::{SetBackgroundColor, SetForegroundColor, ResetColor};
use crate::solarized::{BACK, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};

lazy_static::lazy_static! {
    static ref LOG_FILE_PATH: RwLock<String> = RwLock::new(String::new());
}

#[derive(Deserialize)]
struct Config {
    log_file_path: String,
}

pub fn read_config() {
    let config_str = fs::read_to_string("config.toml")
        .expect("Config file cannot be read.");
    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse config file.");
    let mut path = LOG_FILE_PATH.write().unwrap();
    *path = config.log_file_path;
    println!("{}{}f{}i{}l{}e {}r{}e{}a{}d{}: Config OK",
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
}

pub fn print_log_contents() {
    let path: String = LOG_FILE_PATH.read().unwrap().clone();
    println!("{}", path);
    let mut reader = BufReader::new(File::open(&path).expect("Log file cannot be read."));
    thread::spawn(move || {
        let last_pos = 0;
        loop {
            let mut file = File::open(&path).expect("Unable to open log file.");
            file.seek(SeekFrom::Start(last_pos)).expect("Unable to seek log file.");
            for line in (&mut reader).lines().skip(last_pos as usize) {
                if let Ok(line) = line {
                    if line.contains("killed") {
                        let parts = line.split("killed");
                        let collection = parts.collect::<Vec<&str>>();
                        if let Some(first) = collection.get(0) {
                            let first_string = (*first).to_string();
                            if let Some(second) = collection.get(1) {
                                let second_string = (*second).to_string();
                                println!(
                                    "{}{}{}{}killed{}{}{}",
                                    SetBackgroundColor(BACK),
                                    SetForegroundColor(CYAN),
                                    first_string,
                                    SetForegroundColor(RED),
                                    SetForegroundColor(CYAN),
                                    second_string,
                                    ResetColor
                                );
                            }
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
