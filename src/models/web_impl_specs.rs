use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;

use crate::prelude::*;

impl WebReference {
    /// # Errors
    pub fn load_specs() -> io::Result<Self> {
        let specs_path = find_specs_file()?;
        let input = File::open(specs_path)?;
        let reader = BufReader::new(input);

        serde_json::from_reader(reader)
            .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
    }
}

const SPECS_FILE: &str = "html-5-specs.json";

fn find_specs_file() -> io::Result<PathBuf> {
    #[allow(clippy::useless_transmute)] // caused by concatcp!
    const COMPILE_TARGET: &str = concatcp!(std::path::MAIN_SEPARATOR, "target");

    let mut file_path = current_dir()?;

    let has_target = file_path.to_string_lossy().contains(COMPILE_TARGET);

    loop {
        file_path.push(SPECS_FILE);

        if file_path.exists() {
            return Ok(file_path);
        } else if !has_target {
            return err();
        }

        while file_path.to_string_lossy().contains(COMPILE_TARGET) {
            if !file_path.pop() {
                return err();
            }
        }
    }

    fn err() -> io::Result<PathBuf> {
        Err(io::Error::new(ErrorKind::NotFound, format!("could not locate '{SPECS_FILE}'")))
    }
}