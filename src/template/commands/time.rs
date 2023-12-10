use std::collections::HashSet;

use crate::template::run_multi::run_multi;
use crate::template::timings::Timings;
use crate::template::{all_days, readme_benchmarks, Day};

pub fn handle(day: Option<Day>, recreate_all: bool) {
    let stored_timings = Timings::read_from_file();

    let mut days_to_run = HashSet::new();

    match day {
        Some(day) => {
            days_to_run.insert(day);
        }
        None => {
            all_days().for_each(|day| {
                // when the `--all` flag is not set, filter out days that are fully benched.
                if recreate_all || !stored_timings.is_day_complete(&day) {
                    days_to_run.insert(day);
                }
            });
        }
    };

    let timings = run_multi(days_to_run, true, true).unwrap();

    let merged_timings = stored_timings.merge(&timings);
    merged_timings.store_file().unwrap();

    println!();
    match readme_benchmarks::update(merged_timings) {
        Ok(()) => {
            println!("Stored updated benchmarks.")
        }
        Err(_) => {
            eprintln!("Failed to store updated benchmarks.");
        }
    }
}
