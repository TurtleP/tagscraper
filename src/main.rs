use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;
use clap::Parser;

mod collection;
mod scanner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: PathBuf,

    #[arg(short, long, default_value = "music.txt")]
    filename: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let collection = scanner::scan(&args.directory)?;

    let mut output = File::create(args.filename)?;
    write!(output, "{collection}")?;

    Ok(())
}
