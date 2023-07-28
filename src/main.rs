mod cli;
mod infile_coresight;

use std::fs;
use clap::Parser;
use cli::PattyArgs;
//use pest::Parser as PestParser;
//use infile_coresight::InfileCoresightParser;


fn main() {
    include_str!("../Cargo.toml");
    println!("Hello Dublin! This is Patty talking.");

    let args: PattyArgs = PattyArgs::parse();

    println!("{:?}", args);

    let unparsed_file = fs::read_to_string(args.input[0].to_owned()).expect("Cannot Read input file.");

    println!("{:?}", unparsed_file);

    //let file = InfileCoresightParser::parse(Rule::file, &unparsed_file)
    //.expect("unsuccessful parse")
    //.next().unwrap();
}
