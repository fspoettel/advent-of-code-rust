use advent_of_code::template::commands::{
    all::all_handler, download::download_handler, read::read_handler, scaffold::scaffold_handler,
    solve::solve_handler,
};
use args::{parse_args, AppArgs};

mod args {
    use std::process;

    pub enum AppArgs {
        Download {
            day: u8,
        },
        Read {
            day: u8,
        },
        Scaffold {
            day: u8,
        },
        Solve {
            day: u8,
            release: bool,
            time: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
            time: bool,
        },
    }

    pub fn parse_args() -> Result<AppArgs, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArgs::All {
                release: args.contains("--release"),
                time: args.contains("--time"),
            },
            Some("download") => AppArgs::Download {
                day: args.free_from_str()?,
            },
            Some("read") => AppArgs::Read {
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArgs::Scaffold {
                day: args.free_from_str()?,
            },
            Some("solve") => AppArgs::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                time: args.contains("--time"),
            },
            Some(x) => {
                eprintln!("Unknown command: {}", x);
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {:?}.", remaining);
        }

        Ok(app_args)
    }
}

fn main() {
    match parse_args() {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArgs::All { release, time } => all_handler(release, time),
            AppArgs::Download { day } => download_handler(day),
            AppArgs::Read { day } => read_handler(day),
            AppArgs::Scaffold { day } => scaffold_handler(day),
            AppArgs::Solve {
                day,
                release,
                time,
                submit,
            } => solve_handler(day, release, time, submit),
        },
    };
}
