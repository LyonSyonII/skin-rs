mod structs;

use std::fs::FileType;

use colored::Colorize;
use structs::Cli;

fn file_type_to_human(ftype: FileType) -> &'static str {
    if ftype.is_dir() {
        "Directory"
    } else if ftype.is_file() {
        "File"
    } else if ftype.is_symlink() {
        "Symlink"
    } else {
        "Could not determine file type"
    }
}


fn bytes_to_human(bytes: u64) -> String {
    const GiB: u64 = 1024*1024*1024;
    const MiB: u64 = 1024*1024;
    const KiB: u64 = 1024;

    if bytes > GiB {
        format!("{:.2}GiB", bytes as f64/GiB as f64)
    } else if bytes > MiB {
        format!("{:.2}MiB", bytes as f32/MiB as f32)
    } else if bytes > KiB {
        format!("{:.2}KiB", bytes as f32/KiB as f32)
    } else {
        format!("{bytes}B")
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::read()?;
    let files = cli.files;
    println!("{files:?}");
    
    for file in files {
        let metadata = std::fs::metadata(&file)?;
        println!("{}", file.file_name().unwrap().to_string_lossy().bright_red().bold());
        println!("Type: {}", file_type_to_human(metadata.file_type()));
        println!("Size: {}", bytes_to_human(metadata.len()))
    }
    
    

    Ok(())
}
