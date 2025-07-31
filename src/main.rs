#![allow(unused)]

mod interpreter;
mod parser;

use interpreter::*;
use parser::*;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use std::fs;

fn main() {
    let src = fs::read_to_string("src/main.e").expect("File Not Found");
    let mut env: Environment = HashMap::new();

    match EletronParser::parse(Rule::program, &src) {
        Ok(mut pairs) => {
            let inner_pairs = pairs.next().unwrap().into_inner();
            //println!("{:#?}", inner_pairs);
            for pair in inner_pairs {
                if pair.as_rule() == Rule::stmt {
                    let result = evaluate(&mut env, pair);

                    // Só imprime se o resultado não for nulo (ou seja, não foi um `let`)
                    if !matches!(result, Value::Null) {
                        println!("= {}", result); 
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parsing Failed: {}", e);
        }
    }
}
