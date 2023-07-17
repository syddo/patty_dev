use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PattyArgs {
    /// Config file to fill the setup args
    #[arg(long, short)]
    pub config: Option<PathBuf>,

    /// Specify the pattern generator inputs
    #[arg(long, short, required = true)]
    pub input: Vec<String>,

    /// Specify the input is a marco/library
    #[arg(long, short, required = true)]
    pub library: Vec<String>,
}
