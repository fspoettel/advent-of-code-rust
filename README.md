<img src="./.assets/christmas_ferris.png" width="164" align="center">

# 🎄 [Advent of Code](https://adventofcode.com/)

![Language](https://badgen.net/badge/Language/Rust/orange)
[![CI](https://github.com/fspoettel/advent-of-code-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/fspoettel/advent-of-code-rust/actions/workflows/ci.yml)

<!--- advent_readme_stars table --->

---

Generated from [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust).

## Create your own

 1. Open ☝️ template on Github.
 2. Click `Use this template` and create your repository.
 3. Clone the repository to your machine.

## Install

* Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
* (optional) Install [rust-analyzer](https://rust-analyzer.github.io/manual.html) for your editor.
* (optional) Install a native debugger, e.g. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for VS Code.
* (optional) Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) and follow their setup guide to use the `download` script for puzzle inputs. (see below)
* (optional) Setup the README stars github action. (see below)

## Commands

### Setup new day

```sh
# example: `./bin/scaffold 1`
./bin/scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo run --bin 01` to run your solution.
```

Individual solutions live in the `./src/bin` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/master/bin/scaffold#L21-L52) has _unit tests_ referencing the _example_ file. Use these tests to develop and debug your solution. When editing a solution, `rust-analyzer` will display buttons for these actions above the unit tests.

### Download inputs for a day

```sh
# example: `./bin/download 1`
./bin/download <day>

# output:
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year` flag. _(example: `./bin/download 1 --year 2020`)_

Puzzle inputs are not checked into git. [See here](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3) why.

### Run solutions for a day

```sh
# example: `cargo run --bin 01`
cargo run --bin <day>

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

To run an optimized version for benchmarking, use the `--release` flag or the alias `cargo rr --bin <day>`.

Displayed _timings_ show the raw execution time of your solution w/o overhead (e.g. file reads).

### Run solutions for all days

```sh
cargo run

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

To run an optimized version for benchmarking, use the `--release` flag or the alias `cargo rr`.

_Total timing_ is computed from individual solution _timings_ and excludes overhead.

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

## Setup readme stars

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, you need to do two things:

 1. set repository secrets.
 2. create a private leaderboard.

### Repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

* `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`
* `AOC_YEAR`: the year you want to track. Example: `2021`
* `AOC_SESSION`: an active session for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

### Private Leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.
