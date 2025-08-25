use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct CliArgs {
    /// The pattern to look for
    pattern: String,

    /// The path of the file to search
    path: std::path::PathBuf,
}

fn read_file(path: &std::path::PathBuf) -> String {
    return std::fs::read_to_string(path).expect("Could not read file");
}

fn main() {
    
    let args: CliArgs = CliArgs::parse();
    
    let content = read_file(&args.path);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("FOUND LINE: {}", line);
        }
    }

    println!("OUTPUT: {:?}", content);
}
