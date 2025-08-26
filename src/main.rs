use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct CliArgs {
    /// The pattern to look for
    pattern: String,
    /// The path of the file to search
    path: std::path::PathBuf,
}

/// Read a file and report missing param as Error
fn read_file(path: &std::path::PathBuf) -> Result<String, anyhow::Error> {
    return std::fs::read_to_string(path)
    .with_context(|| format!("Error reading the path `{}`", path.display()));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: CliArgs = CliArgs::parse();
    
    let result: String = read_file(&args.path)?;

    for line in result.lines() {
        if line.contains(&args.pattern) {
            println!("FOUND LINE: {}", line);
        }
    }
    
    Ok(())
}
