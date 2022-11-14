<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

---

## Template setup

This template supports all major OS (macOS, Linux, Windows).

### Create your repository

1.  Open [the template repository](https://github.com/fspoettel/advent-of-code-rust) on Github.
2.  Click `Use this template` and create your repository.
3.  Clone your repository to your computer.

### Setup rust

1.  Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2.  (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3.  (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solution against example inputs. For some puzzles, it might be easier to forgo the example file and hardcode inputs into the tests.

When editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloading input with aoc-cli...
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/aoc`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against example input

```sh
cargo test
```

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

## Optional template features

### Download puzzle inputs via aoc-cli

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli`.
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie[^1] into it. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie value.

Once installed, you can use the [download command](#download-input-for-a-day).

### Enable clippy lints in CI

Uncomment the `clippy` job in the `ci.yml` workflow to enable clippy checks in CI.

### Automatically track ⭐️ progress in readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.
-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`
-   `AOC_YEAR`: the year you want to track. Example: `2021`
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

### Use VS Code to debug your code

 1. Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
 2. Set breakpoints in your code. [^3]
 3. Click _Debug_ next to the unit test or the _main_ function. [^4]
 4. The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [hashbrown](https://crates.io/crates/hashbrown): Faster hashing functions for `HashMap` and `HashSet`.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessred.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

* **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]: <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" /> 
[^4]: <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />
[^5]: <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
