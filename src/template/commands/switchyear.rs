use crate::template::year::Year;
use std::{env, fs, path::PathBuf};

extern crate fs_extra;

pub fn handle(year: Year) {
    let env_year = Year::__new_unchecked(env::var("AOC_YEAR").unwrap().parse().unwrap());
    if year == env_year {
        println!("🔔 You are already in the year you want to switch to.");
    } else {
        switch_to_year(year, env_year);
        println!(
            "---\n🎄 Successfully switched to year {}.",
            year.into_inner()
        );
    }
}

pub fn handle_today() {
    let year = Year::this_year().unwrap();
    let env_year = Year::new(env::var("AOC_YEAR").unwrap().parse().unwrap()).unwrap();
    if year != env_year {
        switch_to_year(year, env_year);
        println!(
            "🎄 Automatically switched to this year: {}.",
            year.into_inner()
        );
    }
}

fn create_folder_with_gitkeep(path: PathBuf) {
    fs_extra::dir::create_all(&path, false).unwrap();
    fs::write(path.join(".keep"), "").unwrap();
}

pub fn switch_to_year(year: Year, previous_year: Year) {
    let cwd = env::current_dir().unwrap();

    // Move src and data files to years/
    let src = cwd.join("src");
    let data = cwd.join("data");
    let bin = src.join("bin");
    let examples = data.join("examples");
    let inputs = data.join("inputs");
    let puzzles = data.join("puzzles");
    let years = cwd.join("years");
    let destination = years.join(previous_year.into_inner().to_string());

    let default_copy = fs_extra::dir::CopyOptions::new();
    let mut inside_copy = fs_extra::dir::CopyOptions::new();
    inside_copy.content_only = true;
    fs_extra::dir::create_all(&destination, false).unwrap();
    fs_extra::dir::move_dir(&bin, &destination, &default_copy).unwrap();
    fs_extra::dir::move_dir(&data, &destination, &inside_copy).unwrap();

    // Move years/ to src and data files
    let source = years.join(year.into_inner().to_string());
    if source.exists() {
        let source_bin = source.join("bin");
        fs_extra::dir::move_dir(&source_bin, &src, &default_copy).unwrap();
        fs_extra::dir::move_dir(&source, &data, &inside_copy).unwrap();
        println!(
            "Found existing files for year {}, moved them.",
            year.into_inner()
        );
    } else {
        println!(
            "No existing files for year {}, generating blank folders.",
            year.into_inner()
        );
        create_folder_with_gitkeep(bin);
        create_folder_with_gitkeep(examples);
        create_folder_with_gitkeep(inputs);
        create_folder_with_gitkeep(puzzles);
    }

    // Set the environment variable
    std::env::set_var("AOC_YEAR", year.into_inner().to_string());

    // Write Cargo.toml
    let config_toml = cwd.join(".cargo").join("config.toml");
    let config_toml_content = fs::read_to_string(&config_toml).unwrap();
    let config_toml_updated_content = config_toml_content.replace(
        &previous_year.into_inner().to_string(),
        &year.into_inner().to_string(),
    );
    fs::write(config_toml, config_toml_updated_content).unwrap();
}
