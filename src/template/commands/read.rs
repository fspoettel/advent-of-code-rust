use std::process;

use crate::template::{aoc_cli, Day};

pub fn handle(day: Day) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    if let Err(e) = aoc_cli::read(day) {
        eprintln!("failed to call aoc-cli: {e}");
        process::exit(1);
    };
}
