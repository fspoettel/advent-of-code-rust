use std::{
    fs::{self},
    path::PathBuf,
    process,
    str::FromStr,
};

use super::write_file;

pub fn handle(year: u32) {
    if !set_year(year) {
        process::exit(1);
    }
    println!("Set repository to AOC year {}", year);
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

fn get_config_path() -> PathBuf {
    PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join(".cargo")
        .join("config.toml")
}

fn read_config(filepath: &PathBuf) -> Result<String, ()> {
    let f = fs::read_to_string(filepath);
    if f.is_err() {
        eprintln!("failed to read Cargo.toml");
        return Err(());
    }
    Ok(f.unwrap())
}
