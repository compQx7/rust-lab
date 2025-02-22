use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // TODO: change to BufReader
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            // TODO: BufWriter
            println!("{}", line);
        }
    }

    Ok(())
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    // println!("content: \n{:?}", content);
}
