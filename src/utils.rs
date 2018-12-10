use std::fs::File;
use std::io::prelude::*;

fn get_day_input(filename: String) -> Result<String> {
    let mut file = File::open(format!("inputs/{}", filename)?
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}