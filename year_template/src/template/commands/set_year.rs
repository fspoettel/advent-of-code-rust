use std::process;

use crate::template::{get_config_path, read_config};

use super::write_file;

pub fn handle(year: u32) {
    if set_year(year) {
        process::exit(1);
    }
}

pub fn set_year(year: u32) -> bool {
    let config_path = get_config_path();
    let new_aoc_year_line = format!("AOC_YEAR = \"{year}\"");
    let config_contents = read_config(&config_path);
    if config_contents.is_err() {
        return false;
    }
    let config_contents = config_contents.unwrap();
    let lines = config_contents.lines().map(|x| {
        if !x.contains("AOC_YEAR") {
            x
        } else {
            &new_aoc_year_line
        }
    });
    let new_contents: Vec<&str> = lines.collect();
    let new_contents = new_contents.join("\n");

    match write_file(&config_path, new_contents.as_bytes()) {
        Ok(_) => true,
        Err(_) => {
            eprintln!("failed to write new year to the config file");
            false
        }
    }
}
