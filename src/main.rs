mod structs;
use structs::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}")
}
