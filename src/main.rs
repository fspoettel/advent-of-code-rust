use aoc::{ANSI_BOLD, ANSI_RESET};
use std::process::Command;

fn main() {
    (1..=25).for_each(|day| {
        let day = format!("{:02}", day);

        let cmd = Command::new("cargo")
            .args(&["run", "--release", "--bin", &day])
            .output()
            .unwrap();

        let output = String::from_utf8(cmd.stdout).unwrap();
        println!("----------");
        println!("{}| Day {} |{}", ANSI_BOLD, day, ANSI_RESET);
        println!("----------");
        println!(
            "{}",
            if !output.is_empty() {
                output
            } else {
                "Not solved.".to_string()
            }
        );
    });
}
