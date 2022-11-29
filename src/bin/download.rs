/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::io::Write;
use std::path::PathBuf;
use std::{env::temp_dir, io, process::Command};
use std::{fs, process};

struct Args {
    day: u8,
    year: Option<i16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn remove_file(path: &PathBuf) {
    #[allow(unused_must_use)]
    {
        fs::remove_file(path);
    }
}

fn exit_with_status(status: i32, path: &PathBuf) -> ! {
    remove_file(path);
    process::exit(status);
}

fn main() {
    // acquire a temp file path to write aoc-cli output to.
    // aoc-cli expects this file not to be present - delete just in case.
    let mut tmp_file_path = temp_dir();
    tmp_file_path.push("aoc_input_tmp");
    remove_file(&tmp_file_path);

    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    };

    let day_padded = format!("{:02}", args.day);
    let input_path = format!("src/inputs/{}.txt", day_padded);

    // check if aoc binary exists and is callable.
    if Command::new("aoc").arg("-V").output().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        exit_with_status(1, &tmp_file_path);
    }

    let mut cmd_args = vec![];

    if let Some(year) = args.year {
        cmd_args.push("--year".into());
        cmd_args.push(year.to_string());
    }

    cmd_args.append(&mut vec![
        "--input-file".into(),
        tmp_file_path.to_string_lossy().to_string(),
        "--day".into(),
        args.day.to_string(),
        "download".into(),
    ]);

    println!("Downloading input with >aoc {}", cmd_args.join(" "));

    match Command::new("aoc").args(cmd_args).output() {
        Ok(cmd_output) => {
            io::stdout()
                .write_all(&cmd_output.stdout)
                .expect("could not write cmd stdout to pipe.");
            io::stderr()
                .write_all(&cmd_output.stderr)
                .expect("could not write cmd stderr to pipe.");
            if !cmd_output.status.success() {
                exit_with_status(1, &tmp_file_path);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }

    match fs::copy(&tmp_file_path, &input_path) {
        Ok(_) => {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &input_path);
            exit_with_status(0, &tmp_file_path);
        }
        Err(e) => {
            eprintln!("could not copy downloaded input to input file: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }
}
