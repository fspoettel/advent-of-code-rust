use std::env;
use std::fs;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solve {
    ($input:expr, $part_one:ident, $part_two:ident) => {{
        use aoc::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> T, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            println!(
                "{} {}(elapsed: {:.2?}){}",
                result, ANSI_ITALIC, elapsed, ANSI_RESET
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

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("elapsed:") {
            acc
        } else {
            let timing = l.split("(elapsed: ").last().unwrap();
            // use `contains` istd. of `ends_with`: string may contain ANSI escape sequences.
            // possible time formats: see [rust/library/core/src/time.rs](https://github.com/rust-lang/rust/blob/1.57.0/library/core/src/time.rs#L1225-L1249).
            if timing.contains("ns)") {
                acc // range below rounding precision.
            } else if timing.contains("µs)") {
                acc + parse_time(timing, "µs") / 1000_f64
            } else if timing.contains("ms)") {
                acc + parse_time(timing, "ms")
            } else if timing.contains("s)") {
                acc + parse_time(timing, "s") * 1000_f64
            } else {
                acc
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// copied from: [rust/library/std/src/macros.rs](https://github.com/rust-lang/rust/blob/1.57.0/library/std/src/macros.rs#L311-L316)
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = (&$a, &$b);
            assert!(
                (*a - *b).abs() < 1.0e-6,
                "{} is not approximately equal to {}",
                *a,
                *b
            );
        }};
    }

    #[test]
    fn test_parse_exec_time() {
        assert_approx_eq!(
            parse_exec_time(&format!(
                "🎄 Part 1 🎄\n0 (elapsed: 74.13ns){}\n🎄 Part 2 🎄\n0 (elapsed: 50.00ns){}",
                ANSI_RESET, ANSI_RESET
            )),
            0_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 755µs)\n🎄 Part 2 🎄\n0 (elapsed: 700µs)"),
            1.455_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 70µs)\n🎄 Part 2 🎄\n0 (elapsed: 1.45ms)"),
            1.52_f64
        );

        assert_approx_eq!(
            parse_exec_time(
                "🎄 Part 1 🎄\n0 (elapsed: 10.3s)\n🎄 Part 2 🎄\n0 (elapsed: 100.50ms)"
            ),
            10400.50_f64
        );
    }
}
