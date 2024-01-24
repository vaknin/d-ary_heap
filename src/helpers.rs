use regex::Regex;
use std::{fs::read_to_string, path::PathBuf};
use anyhow::{bail, Context, Result};

pub fn file_to_nodes(path: PathBuf) -> Result<Vec<i32>>{
    // Read the data from the file
    let file_data = read_to_string(path).context("Unable to read the list file.")?;

    // Check if the list is in a comma-separated format, e.g.: 1,2,3 using regex
    let pattern = Regex::new(r"^[0-9]+(?:,[0-9]+)*$").unwrap(); //unwrapping is okay since we can't panic here
    if !pattern.is_match(&file_data) {
        bail!("There's something wrong with the list.\n\
                Make sure it only contains digits and commas, example: 1,2,3,4,5,6")
    }

    let nodes: Vec<i32> = file_data.split(',').map(|num| num.parse().expect("must be a digit, cannot panic")).collect();
    Ok(nodes)
}