use std::process::{Command, Stdio};

pub fn solve_handler(day: u8, release: bool, time: bool, submit_part: Option<u8>) {
    let day_padded = format!("{:02}", day);

    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day_padded];

    if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string())
    }

    if time {
        cmd_args.push("--time".to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
