use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// reads a single extracted dictionary JSONL and understand the structure
    Explore {
        /// the dictionary file to read
         #[arg(short, long)]
        file: String
    }
}

fn main() {
    println!("Hello, world!");
}
