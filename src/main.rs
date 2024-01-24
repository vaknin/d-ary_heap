use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use helpers::parse_list_file;

mod helpers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What's the "d" of the heap, i.e. how many children can each node have?
    #[arg(short, value_name = "max-nodes")]
    d: String,

    /// The path to the comma separated list of numbers, e.g.: 2,3,4,5
    #[arg(short, value_name = "filepath")]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let list = parse_list_file(args.file)?;
    println!("{list}");

    Ok(())
}