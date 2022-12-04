/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::{
    fs::{File, OpenOptions},
    io::{Error, Read, Seek, Write},
    process::{self, Command},
};

const MODULE_LIB_TEMPLATE: &str = r###"pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = crate::read_file("examples", DAY);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = crate::read_file("examples", DAY);
        assert_eq!(part_two(&input), None);
    }
}
"###;

const MODULE_BIN_TEMPLATE: &str = r###"use advent_of_code::dayDAYPAD::{part_one, part_two};

fn main() {
    let input = &advent_of_code::read_file("inputs", DAY);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
"###;

fn parse_args() -> Result<u8, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    args.free_from_str()
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn modify_lib(day: u8) -> Result<(), Error> {
    let lib_path = "src/lib.rs";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(lib_path)?;

    let mut old_code = String::new();
    file.read_to_string(&mut old_code)?;

    file.rewind()?;
    file.set_len(0)?;

    match file.write_all(
        old_code
            .replace(
                &format!("// pub mod day{:02};", day),
                &format!("pub mod day{:02};", day),
            )
            .as_bytes(),
    ) {
        Ok(_) => {
            println!("Updated lib.rs to include 'day{:02}' module", day);
        }
        Err(e) => {
            eprintln!(
                "Failed to write updateded lib.rs to include 'day{:02}' module: {}",
                day, e
            );
            process::exit(1);
        }
    }

    file.flush()?;

    Ok(())
}

fn main() {
    let day = match parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo scaffold 7`");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day);

    let input_path = format!("src/inputs/{}.txt", day_padded);
    let example_path = format!("src/examples/{}.txt", day_padded);
    let module_bin_path = format!("src/bin/{}.rs", day_padded);
    let module_lib_path = format!("src/day{}.rs", day_padded);

    match modify_lib(day) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to update lib.rs file: {}", e);
            process::exit(1);
        }
    }

    let mut bin_file = match safe_create_file(&module_bin_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module bin file: {}", e);
            process::exit(1);
        }
    };

    match bin_file.write_all(
        MODULE_BIN_TEMPLATE
            .replace("DAYPAD", &day_padded)
            .replace("DAY", &day.to_string())
            .as_bytes(),
    ) {
        Ok(_) => {
            println!("Created module bin file \"{}\"", &module_bin_path);
        }
        Err(e) => {
            eprintln!("Failed to write bin module contents: {}", e);
            process::exit(1);
        }
    }

    let mut lib_file = match safe_create_file(&module_lib_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module lib file: {}", e);
            process::exit(1);
        }
    };

    match lib_file.write_all(
        MODULE_LIB_TEMPLATE
            .replace("DAYPAD", &day_padded)
            .replace("DAY", &day.to_string())
            .as_bytes(),
    ) {
        Ok(_) => {
            println!("Created module lib file \"{}\"", &module_bin_path);
        }
        Err(e) => {
            eprintln!("Failed to write lib module contents: {}", e);
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    // We need to execute cargo clean -p advent_of_code otherwise there can be cache issues when executing cargo check
    // As far as I can tell this relates to https://github.com/rust-lang/cargo/issues/6529
    let clean_output = Command::new("cargo")
        .args(["clean", "-p", "advent_of_code"])
        .output()
        .unwrap();

    if !clean_output.status.success() {
        eprint!("Failed to execute 'cargo clean -p advent_of_code'.");
    }

    println!("---");
    println!(
        "🎄 Type `cargo solve {}` to run your solution.",
        &day_padded
    );
}
