use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub mod all;
pub mod attempt;
pub mod download;
pub mod new_year;
pub mod read;
pub mod scaffold;
pub mod set_year;
pub mod solve;
pub mod time;

#[derive(Debug)]
enum WriteError {
    Open,
    Write,
}

fn open_file(filepath: &PathBuf) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).truncate(true).open(filepath)
}

fn write_file(filepath: &PathBuf, to_write: &[u8]) -> Result<(), WriteError> {
    let file = open_file(filepath);
    if file.is_err() {
        eprintln!("Failed to open file {}", filepath.to_str().unwrap());
        return Err(WriteError::Open);
    }
    let mut file = file.unwrap();

    match file.write_all(to_write) {
        Ok(()) => Ok(()),
        Err(e) => {
            let filepath = filepath.to_str().unwrap();
            eprintln!("Failed to write to {filepath}: {e}");
            Err(WriteError::Write)
        }
    }
}
