use std::{path::{PathBuf}, io::{Read}};

use clap::Parser;
use colored::Colorize;

#[derive(clap::Parser, Debug)]
#[clap(
    name = "Skin",
    about = "Simple command to get all possible metadata of a file/folder.",
    version,
)]
pub struct Cli {
    #[clap(long)]
    pub debug: bool,
    
    
    #[clap()]
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn read() -> std::io::Result<Cli> {
        let mut cli = Cli::parse();
        let mut stdin = std::io::stdin().lock();
        let mut pipe = String::new();
        
        if cli.debug {
            dbg!(atty::is(atty::Stream::Stdin));
            dbg!(atty::is(atty::Stream::Stdout));
            dbg!(atty::is(atty::Stream::Stderr));
        }
        
        // If something is piped in, read it and append
        if atty::isnt(atty::Stream::Stdin) {
            stdin.read_to_string(&mut pipe)?;
        }

        let pipe = pipe.split_ascii_whitespace().map(PathBuf::from);
        cli.files.extend(pipe);
        cli.files.sort();
        cli.files.dedup();

        if cli.files.is_empty() {
            println!("{} The following required arguments were not provided:", "error:".bright_red().bold());
            println!("    {}", "<FILES>...\n".green());
            println!("USAGE:\n    skin [OPTIONS] <FILES>...\n");
            println!("For more information try {}", "--help".green());
            std::process::exit(1)
        }
        
        Ok(cli)
    }
}