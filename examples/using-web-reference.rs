use std::fs::File;
use std::io;
use std::io::BufReader;

use web_reference::prelude::*;

fn main() -> io::Result<()> {
// retrieve json from file
    let json_file = File::open("html-5-specs.json")?;
    let json_reader = BufReader::new(json_file);

// deserialize reference from json
    let reference: WebReference = serde_json::from_reader(json_reader)?;

// use reference
    let div_tag = reference.get_tag("div").expect("expect div to be defined");
    let _div_attributes = reference.get_tag_attributes(div_tag);

    Ok(())
}