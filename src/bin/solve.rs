/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use std::process::{self, Command, Stdio};

struct Args {
    day: u8,
    release: bool,
    submit: Option<u8>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        release: args.contains("--release"),
        submit: args.opt_value_from_str("--submit")?,
    })
}

fn run_solution(day: u8, release: bool, submit_part: Option<u8>) -> Result<(), std::io::Error> {
    let day_padded = format!("{:02}", day);

    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day_padded];

    if release {
        cmd_args.push("--release".to_string());
    }

    if let Some(submit_part) = submit_part {
        cmd_args.push("--".to_string());
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string())
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    cmd.wait()?;
    Ok(())
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            process::exit(1);
        }
    };

    run_solution(args.day, args.release, args.submit).unwrap();
}
