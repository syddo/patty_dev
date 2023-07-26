use pest::Parser;
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "infile_coresight.pest"]
pub struct InfileCoresightParser;
