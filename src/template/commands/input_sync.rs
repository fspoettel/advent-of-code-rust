use std::fs::{exists};
use crate::template::{all_days};
use crate::template::aoc_cli::{download, DownloadMode};

pub fn handle(replace_existing: bool) {
    for day in all_days() {
        let input_path = format!("data/inputs/{day}.txt");
        let existence = exists(input_path);
        if existence.is_err() {
            eprintln!("Unable to check whether input already exists?");
            eprintln!("Hint: Check for listing permissions");
            eprintln!("Skipping day: {}", day);
            continue;
        }
        if !replace_existing && existence.unwrap() { continue; }
        let result = download(day, DownloadMode::InputOnly);
        if result.is_err() {
            eprintln!("Unable to download input for day: {}", day);
            eprintln!("Halting further downloads!");
            return;
        }
    }
}
