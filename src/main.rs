#![allow(unused)]

mod parser;

use parser::*;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use std::fs;

fn main() {
    let src = fs::read_to_string("src/main.e").expect("File Not Found");

    match EletronParser::parse(Rule::program, &src) {
        Ok(mut pairs) => {
            let inner_pairs = pairs.next().unwrap().into_inner();
            
            for pair in inner_pairs {
                match &pair.as_rule() {
                    Rule::sum_expr => {
                        let expr = pair.as_str().split("//").next().unwrap_or("").trim();
                        print!("{} ", expr);
                        let result = eval_sum_expr(pair);
                        println!("= {}", result);
                    }   
                    Rule::EOI => {}
                    _ => eprintln!("Unhandled: {:#?}", pair)                 
                }
            }
        }
        Err(e) => {
            eprintln!("Parsing Failed: {}\n", e);
        }
    }
}
