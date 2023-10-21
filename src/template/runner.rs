/// Encapsulates code that interacts with solution functions.
use crate::template::{aoc_cli, ANSI_ITALIC, ANSI_RESET};
use std::fmt::Display;
use std::io::{stdout, Write};
use std::process::Output;
use std::time::{Duration, Instant};
use std::{cmp, env, process};

use super::ANSI_BOLD;

pub fn run_part<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: u8, part: u8) {
    let part_str = format!("Part {}", part);

    let (result, duration, samples) =
        run_timed(func, input, |result| print_result(result, &part_str, ""));

    print_result(&result, &part_str, &format_duration(&duration, samples));

    if let Some(result) = result {
        submit_result(result, day, part);
    }
}

/// Run a solution part. The behavior differs depending on whether we are running a release or debug build:
///  1. in debug, the function is executed once.
///  2. in release, the function is benched (approx. 1 second of execution time or 10 samples, whatever take longer.)
fn run_timed<I: Clone, T>(
    func: impl Fn(I) -> T,
    input: I,
    hook: impl Fn(&T),
) -> (T, Duration, u128) {
    let timer = Instant::now();
    let result = func(input.clone());
    let base_time = timer.elapsed();

    hook(&result);

    let run = match std::env::args().any(|x| x == "--time") {
        true => bench(func, input, &base_time),
        false => (base_time, 1),
    };

    (result, run.0, run.1)
}

fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> (Duration, u128) {
    let mut stdout = stdout();

    print!(" > {}benching{}", ANSI_ITALIC, ANSI_RESET);
    let _ = stdout.flush();

    let bench_iterations = cmp::min(
        10000,
        cmp::max(
            Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_nanos(), 10),
            10,
        ),
    );

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        // need a clone here to make the borrow checker happy.
        let cloned = input.clone();
        let timer = Instant::now();
        func(cloned);
        timers.push(timer.elapsed());
    }

    (
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations,
    )
}

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers.iter().map(|d| d.as_nanos()).sum::<u128>() / numbers.len() as u128
}

fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!(" ({:.1?})", duration)
    } else {
        format!(" ({:.1?} @ {} samples)", duration, samples)
    }
}

fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    let is_intermediate_result = duration_str.is_empty();

    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{}: ▼ {}", part, duration_str);
                if is_intermediate_result {
                    print!("{}", str);
                } else {
                    print!("\r");
                    println!("{}", str);
                    println!("{}", result);
                }
            } else {
                let str = format!(
                    "{}: {}{}{}{}",
                    part, ANSI_BOLD, result, ANSI_RESET, duration_str
                );
                if is_intermediate_result {
                    print!("{}", str);
                } else {
                    print!("\r");
                    println!("{}", str);
                }
            }
        }
        None => {
            if is_intermediate_result {
                print!("{}: ✖", part);
            } else {
                print!("\r");
                println!("{}: ✖             ", part);
            }
        }
    }
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if:
///  1. we are in `--release` mode.
///  2. aoc-cli is installed.
fn submit_result<T: Display>(
    result: T,
    day: u8,
    part: u8,
) -> Option<Result<Output, aoc_cli::AocCliError>> {
    let args: Vec<String> = env::args().collect();

    if !args.contains(&"--submit".into()) {
        return None;
    }

    if args.len() < 3 {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    }

    let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;
    let part_submit = match args[part_index].parse::<u8>() {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
            process::exit(1);
        }
    };

    if part_submit != part {
        return None;
    }

    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    println!("Submitting result via aoc-cli...");
    Some(aoc_cli::submit(day, part, &result.to_string()))
}
