use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use consts::*;

mod helpers;
mod consts;
mod dheap;

use dheap::Dheap;
use helpers::{main_menu, prompt_filepath};

use crate::helpers::parse_file_to_nodes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What's the "d" of the heap, i.e. how many children can each node have?
    #[arg(short, value_name = "max-nodes")]
    d: usize,

    /// The optional path to the comma separated list of numbers, e.g.: 2,3,4,5
    #[arg(short, value_name = "filepath")]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Parse CLI args, create the 'Dheap' struct
    let args = Args::parse();
    let mut nodes: Vec<i32> = Vec::new();
    if let Some(filepath) = args.file {
        nodes = parse_file_to_nodes(filepath)?;
    }
    let mut heap = Dheap::new(args.d, nodes);

    // Main menu loop
    loop {
        let action = main_menu(heap.nodes.is_empty());
        match action.as_str() {
            LOAD => heap = prompt_filepath(heap)?,
            BUILD => break,
            PRINT => println!("Heap: {heap}"),
            EXTRACT => break,
            INSERT => break,
            INCREASE => break,
            REMOVE => break,
            _ => break,
        }
    }
    Ok(())
}