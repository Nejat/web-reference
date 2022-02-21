use std::io;

use web_reference::prelude::WebReference;

fn main() -> io::Result<()> {
    let specs = WebReference::load_json()?;

    println!("{specs:#?}");

    Ok(())
}