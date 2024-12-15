use std::{
    fs::{self},
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    str::FromStr,
};

use crate::template::commands::set_year;

use super::{write_file, WriteError};

const YEAR_NUMBER_FILES: [&str; 7] = [
    "Cargo.toml",
    "src/main.rs",
    "src/template/aoc_cli.rs",
    "src/template/run_multi.rs",
    "src/template/template.txt",
    "src/template/commands/scaffold.rs",
    ".cargo/config.toml",
];

pub fn handle(year: u32) {
    let project_root = PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join("year_template");
    let new_root = PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join(format!("{}", year));

    copy_year_template(&project_root, &new_root);
    set_year_numbers(year, &new_root);
    set_year(year);
    add_to_workspace(year);
    println!("Created AOC year {} workspace module", year);
}

fn copy_year_template(project_root: &Path, new_root: &Path) {
    let cmd_args = vec![
        project_root.to_str().unwrap(),
        &new_root.to_str().unwrap(),
        "-r",
    ];
    let mut cmd = Command::new("cp")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}

fn set_year_numbers(year: u32, new_root: &Path) {
    for filename in YEAR_NUMBER_FILES {
        let filepath = new_root.join(filename);

        let original_contents = match fs::read_to_string(filepath.clone()) {
            Ok(original) => original,
            Err(_) => {
                eprintln!("Could not read from file to set year numbers");
                cleanup(year);
                process::exit(1);
            }
        };
        let mut new_contents = original_contents.clone();
        new_contents = new_contents.replace("%YEAR_NUMBER%", &year.to_string());
        if !filename.contains("scaffold") {
            new_contents = new_contents.replace("YEAR_NUMBER", &year.to_string());
        }
        let new_contents = new_contents.as_bytes();

        if write_file(&filepath, new_contents).is_err() {
            cleanup(year);
            process::exit(1);
        }
    }
}

fn set_year(year: u32) {
    if !set_year::set_year(year) {
        cleanup(year);
        process::exit(1);
    }
}

fn add_to_workspace(year: u32) {
    let filepath = PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join("Cargo.toml");
    let original_contents = read_toml_file();
    if original_contents.is_err() {
        cleanup(year);
        process::exit(1);
    }
    let original_contents = original_contents.unwrap();
    let new_contents = add_year_to_toml_str(year, &original_contents);
    match write_file(&filepath, new_contents.to_string().as_bytes()) {
        Ok(()) => (),
        Err(WriteError::Open) => (),
        Err(WriteError::Write) => {
            cleanup(year);
            write_file(&filepath, original_contents.to_string().as_bytes()).unwrap();
            process::exit(1);
        }
    }
}

fn read_toml_file() -> Result<String, ()> {
    let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let f = fs::read_to_string(filepath);
    if f.is_err() {
        eprintln!("failed to read Cargo.toml");
        return Err(());
    }
    Ok(f.unwrap())
}

fn add_year_to_toml_str(year: u32, original: &str) -> String {
    let end_pos = get_end_pos_of_members(original);
    if end_pos.is_err() {
        cleanup(year);
        process::exit(1);
    }
    let end_pos = end_pos.unwrap();

    let start = &original[..end_pos];
    let end = &original[end_pos..];
    let new = format!(", \"{}\"", year);
    format!("{}{}{}", start, new, end)
}

fn get_end_pos_of_members(original: &str) -> Result<usize, ()> {
    let start_idx = original[..].find("members = [");
    if start_idx.is_none() {
        eprintln!("failed to find a members section of Cargo.toml");
        return Err(());
    }
    let start_idx = start_idx.unwrap();
    let end_idx = original[start_idx..].find("]");
    match end_idx {
        Some(i) => Ok(i + start_idx),
        None => {
            eprintln!("failed to find the end of the members section of Cargo.toml");
            Err(())
        }
    }
}

fn cleanup(year: u32) {
    let mut new_root = String::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    new_root.push_str(&format!("/{}/", year));

    let cmd_args = vec![&new_root, "-r"];
    let mut cmd = Command::new("rm")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
}
