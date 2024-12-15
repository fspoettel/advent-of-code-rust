use advent_of_code::template::commands::{
    all, attempt, download, new_year, read, scaffold, set_year, solve, time,
};
use args::{parse, AppArguments};

#[cfg(feature = "today")]
use advent_of_code::template::Day;
#[cfg(feature = "today")]
use std::process;

mod args {
    use advent_of_code::template::Day;
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
        Try {
            day: Day,
            release: bool,
            dhat: bool,
        },
        All {
            release: bool,
        },
        Time {
            all: bool,
            day: Option<Day>,
            store: bool,
        },
        NewYear {
            year: u32,
        },
        SetYear {
            year: u32,
        },
        GetYear,
        #[cfg(feature = "today")]
        Today,
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
            Some("try") => AppArguments::Try {
                day: args.free_from_str()?,
                release: args.contains("--submit"),
                dhat: args.contains("--dhat"),
            },
            Some("new-year") => AppArguments::NewYear {
                year: args.free_from_str()?,
            },
            Some("set-year") => AppArguments::SetYear {
                year: args.free_from_str()?,
            },
            Some("get-year") => AppArguments::GetYear,
            #[cfg(feature = "today")]
            Some("today") => AppArguments::Today,
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
            AppArguments::Try { day, release, dhat } => attempt::handle(day, release, dhat),
            #[cfg(feature = "today")]
            AppArguments::Today => {
                match Day::today() {
                    Some(day) => {
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
            AppArguments::NewYear { year } => new_year::handle(year),
            AppArguments::SetYear { year } => set_year::handle(year),
            AppArguments::GetYear => {
                let year = advent_of_code::template::get_year_exit_on_fail();
                println!("The repository is currently set to {}", year);
            }
        },
    };
}
