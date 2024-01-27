use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use consts::*;
use dheap::Dheap;
use helpers::{increase_key_prompt, is_positive, main_menu, prompt_filepath};
use crate::helpers::{clear_screen, parse_file_to_nodes};

mod helpers;
mod consts;
mod dheap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What's the "d" of the heap, i.e. how many children can each node have?
    #[arg(short, value_name = "max-nodes", value_parser=is_positive)]
    d: usize,

    /// The optional path to the comma separated list of numbers, e.g.: 2,3,4,5
    #[arg(short, value_name = "filepath")]
    file: Option<PathBuf>
}

fn main() -> Result<()> {

    // Parse CLI arguments to create the 'Dheap' structure
    let args = Args::parse();
    let mut nodes: Vec<i32> = Vec::new();

    // If a file path is provided, parse it to initialize nodes
    if let Some(filepath) = args.file {
        nodes = parse_file_to_nodes(filepath)?;
    }

    // Create a new heap with the given branching factor and nodes
    let mut heap = Dheap::new(args.d, nodes);

    // Main menu loop for user interaction
    clear_screen();
    loop {
        // Display main menu and get user action
        let action = main_menu(heap.nodes.is_empty());
        match action.as_str() {

            // Load a new heap from a file
            LOAD => heap = prompt_filepath(args.d)?,

            // Print the current heap
            PRINT => heap.print(),

            // Extract the max element and display it
            EXTRACT => println!("Extracted: {}", heap.extract_max()),

            // Insert a new element into the heap
            INSERT => heap.insert(),

            // Increase the key of a node
            INCREASE => {
                let (i, key) = increase_key_prompt();
                heap.increase_key(i, key, false);
            },

            // Delete a node from the heap
            DELETE => heap.delete(),

            // Exit the loop and program on any other action
            _ => break
        }
        
        // Pause and wait for the user to press enter before clearing the screen
        println!("\nPress enter to continue...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        clear_screen();
    }
    Ok(()) // Return Ok to signify successful execution
}