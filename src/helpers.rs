use dialoguer::{theme::ColorfulTheme, Input, Select};
use regex::Regex;
use std::{fs::read_to_string, path::PathBuf, process::Command};
use anyhow::{bail, Context, Result};
use crate::{consts::*, dheap::Dheap};

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub fn parse_file_to_nodes(path: PathBuf) -> Result<Vec<i32>>{
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

pub fn main_menu(heap_is_empty: bool) -> String {

    // Build the menu options
    let mut menu_selections = vec![];
    if !heap_is_empty {
        menu_selections.append(&mut vec![
            PRINT,
            EXTRACT,
            INCREASE,
            DELETE
        ]);
    }
    menu_selections.append(&mut vec![INSERT, LOAD, QUIT]);
    
    // Wait for user input
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&menu_selections)
        .interact()
        .unwrap();

    // Return the selection
    menu_selections[selection].to_string()
}

pub fn prompt_filepath(d: usize) -> Result<Dheap> {
    let filepath: String = Input::new()
                .with_prompt("Enter the list's filepath")
                .interact_text()
                .unwrap();
    let nodes = parse_file_to_nodes(PathBuf::from(filepath))?;
    let heap = Dheap::new(d, nodes);
    Ok(heap)
}

pub fn increase_key_prompt() -> (usize, i32) {

    let i: usize = Input::new()
        .with_prompt("Enter the index to increase")
        .interact_text()
        .unwrap();

    let key: i32 = Input::new()
        .with_prompt("Enter the new key")
        .interact_text()
        .unwrap();
    (i, key)
}