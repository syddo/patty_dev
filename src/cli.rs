use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PattyArgs {
    /// Config file to fill the setup args
    #[arg(long, short, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Pattern Generator Mode
    #[arg(long, short, default_value = "CSSerialWire")]
    pub mode: String,

    /// Pattern Pins
    #[arg(long, short)]
    pub pins: String,

    /// Subset Pins
    #[arg(long, short)]
    pub subset: String,

    /// Default vector
    #[arg(long, short)]
    pub vector: String,

    /// Vector format on output pattern files
    #[arg(long, short)]
    pub format: String,

    /// Specify the pattern generator inputs
    #[arg(long, short)]
    pub input: Vec<String>,

    /// Specify the input is a marco/library
    #[arg(long, short)]
    pub library: Vec<String>,
}


/// Reutrns the parsed CLI arguments
/// 
pub fn get_cli_args() -> PattyArgs {
    return PattyArgs::parse();
}

