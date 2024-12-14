use advent_of_code::template::commands::{all, download, read, scaffold, solve, switchyear, time};
use advent_of_code::template::Day;
use advent_of_code::template::{ANSI_BOLD, ANSI_RESET};
use args::{parse, AppArguments};
use std::process;

mod args {
    use advent_of_code::template::{Day, Year};
    use std::process;

    pub enum AppArguments {
        Download {
            day: Day,
        },
        Read {
            day: Day,
        },
        Scaffold {
            day: Day,
            download: bool,
            overwrite: bool,
        },
        Solve {
            day: Day,
            release: bool,
            dhat: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
        },
        Time {
            all: bool,
            day: Option<Day>,
            store: bool,
        },
        Today,
        SwitchYear {
            year: Year,
        },
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArguments::All {
                release: args.contains("--release"),
            },
            Some("time") => {
                let all = args.contains("--all");
                let store = args.contains("--store");

                AppArguments::Time {
                    all,
                    day: args.opt_free_from_str()?,
                    store,
                }
            }
            Some("download") => AppArguments::Download {
                day: args.free_from_str()?,
            },
            Some("read") => AppArguments::Read {
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArguments::Scaffold {
                day: args.free_from_str()?,
                download: args.contains("--download"),
                overwrite: args.contains("--overwrite"),
            },
            Some("solve") => AppArguments::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                dhat: args.contains("--dhat"),
            },
            Some("today") => AppArguments::Today,
            Some("switch-year") => AppArguments::SwitchYear {
                year: args.free_from_str()?,
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    println!(
        "🎄{ANSI_BOLD} Advent of Code {} {ANSI_RESET}🎄",
        std::env::var("AOC_YEAR").unwrap()
    );
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::All { release } => all::handle(release),
            AppArguments::Time { day, all, store } => time::handle(day, all, store),
            AppArguments::Download { day } => download::handle(day),
            AppArguments::Read { day } => read::handle(day),
            AppArguments::Scaffold {
                day,
                download,
                overwrite,
            } => {
                scaffold::handle(day, overwrite);
                if download {
                    download::handle(day);
                }
            }
            AppArguments::Solve {
                day,
                release,
                dhat,
                submit,
            } => solve::handle(day, release, dhat, submit),
            AppArguments::Today => {
                match Day::today() {
                    Some(day) => {
                        switchyear::handle_today();
                        scaffold::handle(day, false);
                        download::handle(day);
                        read::handle(day)
                    }
                    None => {
                        eprintln!(
                            "`today` command can only be run between the 1st and \
                            the 25th of december. Please use `scaffold` with a specific day."
                        );
                        process::exit(1)
                    }
                };
            }
            AppArguments::SwitchYear { year } => {
                switchyear::handle(year);
            }
        },
    };
}
