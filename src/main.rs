mod solutions;
use anyhow::Result;
use clap::Parser;

use crate::solutions::load;

#[derive(Parser)]
struct Cli {
    /// Day of challenge
    #[arg(short, long)]
    day: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut soln = load(cli.day)?;
    soln.part1();
    soln.part2();

    Ok(())
}
