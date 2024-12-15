use std::{env, fs, path::PathBuf, process, str::FromStr};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod run_multi;
pub mod runner;
pub mod timings;

pub use day::*;

mod day;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub fn get_year() -> Option<u32> {
    let config_path = get_config_path();
    let config_contents = read_config(&config_path);
    if let Err(()) = config_contents {
        process::exit(1);
    }
    let config_contents = config_contents.unwrap();
    let year: String = config_contents
        .lines()
        .filter(|s| s.contains("AOC_YEAR"))
        .collect();
    let year: Vec<&str> = year.split("\"").collect();
    let year = year.get(year.len() - 2).unwrap();
    year.parse::<u32>().ok()
}

pub fn get_year_exit_on_fail() -> u32 {
    let year = get_year();
    if year.is_none() {
        eprintln!("Failed to get the currently set AOC year");
        std::process::exit(1);
    }
    year.unwrap()
}

fn get_config_path() -> PathBuf {
    let config_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join("..")
        .join(".cargo")
        .join("config.toml");
    config_path.canonicalize().unwrap()
}

fn read_config(filepath: &PathBuf) -> Result<String, ()> {
    let f = fs::read_to_string(filepath);
    if f.is_err() {
        eprintln!("failed to read Cargo.toml");
        return Err(());
    }
    Ok(f.unwrap())
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
    };
    ($day:expr, 1) => {
        $crate::solution!(@impl $day, [part_one, 1]);
    };
    ($day:expr, 2) => {
        $crate::solution!(@impl $day, [part_two, 2]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::template::Day = $crate::day!($day);

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            use $crate::template::runner::*;
            let input = $crate::template::read_file("inputs", DAY);
            $( run_part($func, &input, DAY, $part); )*
        }
    };
}
