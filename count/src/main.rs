use anyhow::{Result};
use clap::Parser;

use count::count_in_path;

#[derive(Parser)]
/// Count lines or words in the specified files
struct Args {
  /// Counts words instead of lines
  #[arg(short, long)]
  words: bool,

  /// Files to be counted
  #[arg(required = true)]
  files: Vec<String>,
}


fn main() -> Result<()> {
  let args = Args::parse();

  for path in args.files {
   let count = count_in_path(&path)?;
   println!("{path}: {}", if args.words { count.words } else { count.lines });
  }
  Ok(())
}