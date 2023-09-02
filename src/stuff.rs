use std::fs::{self, File};
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::path::PathBuf;
use toml;
use serde::Deserialize;
use crate::solarized::{print_colored, VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA};

#[derive(Deserialize)]
struct Config {
    log_file_path: PathBuf,
}

pub fn read_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    print_colored(
        &["f", "i", "l", "e ", "r", "e", "a", "d", ": Config OK"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    Ok(config.log_file_path)
}

pub fn print_log_contents(path: Arc<PathBuf>) {
    let mut reader = BufReader::new(File::open(&*path).unwrap());
    let last_pos = 0;
    loop {
        let mut file = File::open(&*path).unwrap();
        file.seek(SeekFrom::Start(last_pos)).unwrap();
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
}
