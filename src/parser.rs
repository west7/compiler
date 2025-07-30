use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "eletron.pest"]
pub struct EletronParser;