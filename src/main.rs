mod cli;
mod pattern_file;

use clap::Parser;
use cli::PattyArgs;

fn main() {
    include_str!("../Cargo.toml");
    println!("Hello Dublin! This is Patty talking.");

    let args: PattyArgs = PattyArgs::parse();

    println!("{:?}", args);
}
