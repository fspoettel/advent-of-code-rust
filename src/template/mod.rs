use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use] pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day:02}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// main! produces a block setting up the input and runner for each part.
#[macro_export]
macro_rules! main {
    ($day:expr) => {
        fn main() {
            use advent_of_code::template::runner::*;
            let input = advent_of_code::template::read_file("inputs", $day);
            run_part(part_one, &input, $day, 1);
            run_part(part_two, &input, $day, 2);
        }
    };
}
