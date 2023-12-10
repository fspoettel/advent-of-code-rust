use std::collections::HashSet;

use crate::template::run_multi::run_multi;
use crate::template::timings::Timings;
use crate::template::{all_days, readme_benchmarks, Day};

pub fn handle(day: Option<Day>, recreate_all: bool) {
    let stored_timings = Timings::read_from_file();

    let days_to_run = day.map(|day| HashSet::from([day])).unwrap_or_else(|| {
        if recreate_all {
            all_days().collect()
        } else {
            // when the `--all` flag is not set, filter out days that are fully benched.
            all_days()
                .filter(|day| !stored_timings.is_day_complete(day))
                .collect()
        }
    });

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
