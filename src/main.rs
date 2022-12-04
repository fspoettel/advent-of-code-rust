/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use std::process::Command;

fn main() {
    let total: f64 = (1..=25)
        .map(|day| {
            let day = format!("{:02}", day);

            let mut args = vec!["run", "--bin", &day];
            if cfg!(not(debug_assertions)) {
                args.push("--release");
            }

            let cmd = Command::new("cargo").args(&args).output().unwrap();

            println!("----------");
            println!("{}| Day {} |{}", ANSI_BOLD, day, ANSI_RESET);
            println!("----------");

            let output = String::from_utf8(cmd.stdout).unwrap();
            let is_empty = output.is_empty();

            println!(
                "{}",
                if is_empty {
                    "Not solved."
                } else {
                    output.trim()
                }
            );

            if is_empty {
                0_f64
            } else {
                advent_of_code::parse_exec_time(&output)
            }
        })
        .sum();

    println!(
        "{}Total:{} {}{:.2}ms{}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total, ANSI_RESET
    );
}
