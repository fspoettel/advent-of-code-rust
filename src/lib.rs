/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 * Prefer `./helpers.rs` if you want to extract code from your solutions.
 */
use std::env;
use std::fs;

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solve {
    ($day:expr, $part:expr, $solver:ident, $input:expr) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET, aoc_cli};
        use std::fmt::Display;
        use std::time::Instant;
        use std::env;
        use std::process;

        fn submit_if_requested<T: Display>(result: T) {
            let args: Vec<String> = env::args().collect();

            if args.contains(&"--submit".into()) {
                if aoc_cli::check().is_err() {
                    eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
                    process::exit(1);
                }

                if args.len() < 3 {
                    eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
                    process::exit(1);
                }

                let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;
                let part_submit =  match args[part_index].parse::<u8>() {
                    Ok(x) => x,
                    Err(_) => {
                        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
                        process::exit(1);
                    }
                };

                if part_submit == $part {
                    println!("Submitting puzzle answer for part {}...", $part);
                    aoc_cli::submit($day, $part, result).unwrap();
                }
            }
        }

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Some(result) => {
                    println!(
                        "{} {}(elapsed: {:.2?}){}",
                        result, ANSI_ITALIC, elapsed, ANSI_RESET
                    );
                    submit_if_requested(result);
                }
                None => {
                    println!("not solved.")
                }
            }
        }

        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part, ANSI_RESET);
        print_result($solver, $input);
    }};
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("elapsed:") {
            acc
        } else {
            let timing = l.split("(elapsed: ").last().unwrap();
            // use `contains` istd. of `ends_with`: string may contain ANSI escape sequences.
            // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
            if timing.contains("ns)") {
                acc // range below rounding precision.
            } else if timing.contains("µs)") {
                acc + parse_time(timing, "µs") / 1000_f64
            } else if timing.contains("ms)") {
                acc + parse_time(timing, "ms")
            } else if timing.contains("s)") {
                acc + parse_time(timing, "s") * 1000_f64
            } else {
                acc
            }
        }
    })
}

/// copied from: https://github.com/rust-lang/rust/blob/1.64.0/library/std/src/macros.rs#L328-L333
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exec_time() {
        assert_approx_eq!(
            parse_exec_time(&format!(
                "🎄 Part 1 🎄\n0 (elapsed: 74.13ns){}\n🎄 Part 2 🎄\n0 (elapsed: 50.00ns){}",
                ANSI_RESET, ANSI_RESET
            )),
            0_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 755µs)\n🎄 Part 2 🎄\n0 (elapsed: 700µs)"),
            1.455_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 70µs)\n🎄 Part 2 🎄\n0 (elapsed: 1.45ms)"),
            1.52_f64
        );

        assert_approx_eq!(
            parse_exec_time(
                "🎄 Part 1 🎄\n0 (elapsed: 10.3s)\n🎄 Part 2 🎄\n0 (elapsed: 100.50ms)"
            ),
            10400.50_f64
        );
    }
}

pub mod aoc_cli {
    use std::{
        fmt::Display,
        process::{Command, Output, Stdio},
    };

    #[derive(Debug)]
    pub enum AocCliError {
        CommandNotFound,
        CommandNotCallable,
        BadExitStatus(Output),
        IoError,
    }

    impl Display for AocCliError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AocCliError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
                AocCliError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
                AocCliError::BadExitStatus(_) => {
                    write!(f, "aoc-cli exited with a non-zero status.")
                }
                AocCliError::IoError => write!(f, "could not write output files to file system."),
            }
        }
    }

    pub fn check() -> Result<(), AocCliError> {
        Command::new("aoc")
            .arg("-V")
            .output()
            .map_err(|_| AocCliError::CommandNotFound)?;
        Ok(())
    }

    pub fn read(day: u8) -> Result<Output, AocCliError> {
        // TODO: output local puzzle if present.
        let puzzle_path = get_puzzle_path(day);

        let args = build_args(
            "read",
            &[
                "--description-only".into(),
                "--puzzle-file".into(),
                puzzle_path,
            ],
            day,
        );

        call_aoc_cli(&args)
    }

    pub fn download(day: u8) -> Result<Output, AocCliError> {
        let input_path = get_input_path(day);
        let puzzle_path = get_puzzle_path(day);

        let args = build_args(
            "download",
            &[
                "--overwrite".into(),
                "--input-file".into(),
                input_path.to_string(),
                "--puzzle-file".into(),
                puzzle_path.to_string(),
            ],
            day,
        );

        let output = call_aoc_cli(&args)?;

        if output.status.success() {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &input_path);
            println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzle_path);
            Ok(output)
        } else {
            Err(AocCliError::BadExitStatus(output))
        }
    }

    pub fn submit<T: Display>(day: u8, part: u8, result: T) -> Result<Output, AocCliError> {
        // workaround: the argument order is inverted for submit.
        let mut args = build_args("submit", &[], day);
        args.push(part.to_string());
        args.push(result.to_string());
        call_aoc_cli(&args)
    }

    fn get_input_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/inputs/{}.txt", day_padded)
    }

    fn get_puzzle_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/puzzles/{}.md", day_padded)
    }

    fn get_year() -> Option<u16> {
        match std::env::var("AOC_YEAR") {
            Ok(x) => x.parse().ok().or(None),
            Err(_) => None,
        }
    }

    fn build_args(command: &str, args: &[String], day: u8) -> Vec<String> {
        let mut cmd_args = args.to_vec();

        if let Some(year) = get_year() {
            cmd_args.push("--year".into());
            cmd_args.push(year.to_string());
        }

        cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

        cmd_args
    }

    fn call_aoc_cli(args: &[String]) -> Result<Output, AocCliError> {
        if cfg!(debug_assertions) {
            println!("Calling >aoc with: {}", args.join(" "));
        }

        Command::new("aoc")
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|_| AocCliError::CommandNotCallable)
    }
}
