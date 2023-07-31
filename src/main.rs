mod cli;

use clap::Parser;
use cli::PattyArgs;

fn main() {
    include_str!("../Cargo.toml");
    println!("Hello Dublin! This is Patty talking.");

    let args: PattyArgs = PattyArgs::parse();
    println!("{:?}", args);

    // check for config file
    if let Some(config_file) = args.config.as_deref() {
        println!("Config file is: {:?}", config_file);
        println!("Value for config file: {}", config_file.display());
    }
}
