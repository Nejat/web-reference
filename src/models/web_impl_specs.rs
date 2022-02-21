use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind};

use crate::prelude::*;

impl WebReference {
    /// # Errors
    pub fn load_json() -> io::Result<Self> {
        let input = File::open("html-5-specs.json")?;
        let reader = BufReader::new(input);

        serde_json::from_reader(reader)
            .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
    }
}