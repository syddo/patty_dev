use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PattyArgs {
    /// Config file to fill the setup args
    #[arg(long, short)]
    pub config: Option<PathBuf>,

    /// Pattern Generator Mode
    #[arg(long, short, required = true, default_value = "CSSerialWire")]
    pub mode: String,

    /// Pattern Pins
    #[arg(long, short, required = true)]
    pub pins: String,

    /// Subset Pins
    #[arg(long, short, required = true)]
    pub subset: String,

    /// Default vector
    #[arg(long, short, required = true)]
    pub vector: String,

    /// Vector format on output pattern files
    #[arg(long, short, required = true)]
    pub format: String,

    /// Specify the pattern generator inputs
    #[arg(long, short, required = true)]
    pub input: Vec<String>,

    /// Specify the input is a marco/library
    #[arg(long, short, required = true)]
    pub library: Vec<String>,
}
