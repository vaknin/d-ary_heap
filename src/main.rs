use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use consts::*;

mod helpers;
mod consts;
mod dheap;

use dialoguer::Select;

use crate::helpers::file_to_nodes;

fn menu(filepath: Option<PathBuf>) -> Result<()>{
    let mut nodes: Vec<i32> = Vec::new();
    if let Some(filepath) = filepath {
        nodes = file_to_nodes(filepath)?;
    }
    loop {
        // Build the menu options
        let mut menu_selections = vec![BUILD];
        if !nodes.is_empty() {
            menu_selections.append(&mut vec![
                PRINT,
                EXTRACT,
                INSERT,
                INCREASE,
                REMOVE
            ]);
        }
        menu_selections.push(QUIT);
    
        let selection = Select::new()
            .with_prompt("Choose an action")
            .default(0)
            .items(&menu_selections)
            .interact()
            .unwrap();
    
            match menu_selections[selection] {
                BUILD => break,
                PRINT => println!("{nodes:?}"),
                EXTRACT => break,
                INSERT => break,
                INCREASE => break,
                REMOVE => break,
                _ => {},
            }
        // Quit the application
        if menu_selections[selection] == QUIT {
            break;
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What's the "d" of the heap, i.e. how many children can each node have?
    #[arg(short, value_name = "max-nodes")]
    d: String,

    /// The optional path to the comma separated list of numbers, e.g.: 2,3,4,5
    #[arg(short, value_name = "filepath")]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    menu(args.file);


    Ok(())
}