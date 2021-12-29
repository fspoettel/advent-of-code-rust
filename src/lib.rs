use std::env;
use std::fs;

pub static ANSI_ITALIC: &str = "\x1b[3m";
pub static ANSI_BOLD: &str = "\x1b[1m";
pub static ANSI_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solve_day {
    ($input:expr, $part_one:ident, $part_two:ident) => {{
        use aoc::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> T, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let time = timer.elapsed();
            println!(
                "{} {}(elapsed: {:.2?}){}",
                result, ANSI_ITALIC, time, ANSI_RESET
            );
        }

        println!("🎄 {}Part 1{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        print_result($part_one, $input);
        println!("");
        println!("🎄 {}Part 2{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        print_result($part_two, $input);
    }};
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
