use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[clap(
    name = "Skin",
    about = "Simple command to get all possible metadata of a file/folder.",
    version,
)]
pub struct Cli {
    
    
    
    
    #[clap(required=true)]
    files: Vec<PathBuf>,
}

impl Cli {
    //pub fn read() -> Vec<PathBuf> {
    //    let cli
    //}
}