/// Encapsulates code that interacts with solution functions.
use std::fmt::Display;
use std::hint::black_box;
use std::io::{Write, stdout};
use std::process::Output;
use std::time::{Duration, Instant};
use std::{cmp, process};

use crate::template::ANSI_BOLD;
use crate::template::{ANSI_ITALIC, ANSI_RESET, Day, aoc_cli};

pub fn run_part<I: Copy, T: Display>(func: fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let part_str = format!("Part {part}");

    let (result, duration, samples) =
        run_timed(func, input, |result| print_result(result, &part_str, ""));

    print_result(&result, &part_str, &format_duration(&duration, samples));

    if let Some(result) = result {
        submit_result(result, day, part);
    }
}

/// Run a solution part. The behavior differs depending on whether the `--time` flag is passed:
///  1. without it, the function is executed once.
///  2. with `--time`, the function is benched (approx. 1 second of execution time or 10 samples, whatever takes longer.)
fn run_timed<I: Copy, T>(func: fn(I) -> T, input: I, hook: impl Fn(&T)) -> (T, Duration, u128) {
    let timer = Instant::now();
    let result = {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        func(input)
    };
    let base_time = timer.elapsed();

    hook(&result);

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input)
    } else {
        (base_time, 1)
    };

    (result, run.0, run.1)
}

const WARM_UP_TIME: Duration = Duration::from_millis(100);
const SAMPLE_TIME: Duration = Duration::from_millis(1);
const MEASUREMENT_TIME: Duration = Duration::from_secs(1);

fn bench<I: Copy, T>(func: fn(I) -> T, input: I) -> (Duration, u128) {
    // hide the pointer's target from the optimizer, so the solution can't be inlined into the loop.
    let func = black_box(func);
    let mut stdout = stdout();

    print!(" > {ANSI_ITALIC}benching{ANSI_RESET}");
    let _ = stdout.flush();

    // warm up caches and branch predictors, and estimate the time per iteration.
    let warm_up = Instant::now();
    let mut warm_up_iterations: u32 = 0;
    while warm_up_iterations == 0 || warm_up.elapsed() < WARM_UP_TIME {
        black_box(func(black_box(input)));
        warm_up_iterations += 1;
    }

    let estimate = cmp::max(
        warm_up.elapsed() / warm_up_iterations,
        Duration::from_nanos(1),
    );

    let batch_size = cmp::max(SAMPLE_TIME.as_nanos() / estimate.as_nanos(), 1);
    let sample_count =
        (MEASUREMENT_TIME.as_nanos() / (estimate.as_nanos() * batch_size)).clamp(10, 1000);

    #[allow(clippy::cast_precision_loss)]
    let mut samples: Vec<f64> = (0..sample_count)
        .map(|_| {
            let timer = Instant::now();
            for _ in 0..batch_size {
                black_box(func(black_box(input)));
            }
            timer.elapsed().as_nanos() as f64 / batch_size as f64
        })
        .collect();

    samples.sort_unstable_by(f64::total_cmp);

    (
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Duration::from_nanos(samples[samples.len() / 2].round() as u64),
        sample_count * batch_size,
    )
}

fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!(" ({duration:.1?})")
    } else {
        format!(" ({duration:.1?} @ {samples} samples)")
    }
}

fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    let is_intermediate_result = duration_str.is_empty();

    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{part}: ▼ {duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                    println!("{result}");
                }
            } else {
                let str = format!("{part}: {ANSI_BOLD}{result}{ANSI_RESET}{duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                }
            }
        }
        None => {
            if is_intermediate_result {
                print!("{part}: ✖");
            } else {
                print!("\r");
                println!("{part}: ✖             ");
            }
        }
    }
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if:
///  1. we are in `--release` mode.
///  2. aoc-cli is installed.
fn submit_result<T: Display>(
    result: T,
    day: Day,
    part: u8,
) -> Option<Result<Output, aoc_cli::AocCommandError>> {
    let part_submit = match pico_args::Arguments::from_env().opt_value_from_str::<_, u8>("--submit")
    {
        Ok(Some(part_submit)) => part_submit,
        Ok(None) => return None,
        Err(_) => {
            eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
            process::exit(1);
        }
    };

    if part_submit != part {
        return None;
    }

    if aoc_cli::check().is_err() {
        eprintln!(
            "command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it."
        );
        process::exit(1);
    }

    println!("Submitting result via aoc-cli...");
    Some(aoc_cli::submit(day, part, &result.to_string()))
}
