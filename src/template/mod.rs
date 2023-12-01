use crate::Day;
use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        /// The current day.
        const DAY: advent_of_code::Day = advent_of_code::day!($day);

        fn main() {
            use advent_of_code::template::runner::*;
            let input = advent_of_code::template::read_file("inputs", DAY);
            run_part(part_one, &input, DAY, 1);
            run_part(part_two, &input, DAY, 2);
        }
    };
}
