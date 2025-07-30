#![allow(unused)]

mod parser;

use parser::EletronParser;
use parser::Rule;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use std::fs;

fn main() {
    let file_e = fs::read_to_string("src/main.e").expect("File Not Found");

    match EletronParser::parse(Rule::program, &file_e) {
        Ok(pairs) => {
            //println!("{:#?}", pairs);
            for pair in pairs.into_iter() {
                for expr in pair.into_inner() {
                    // println!("PAR {:#?}", pair);
                    match expr.as_rule() {
                        Rule::expr => {
                            let result = evaluate_expr(expr);
                            println!("= {}", result)
                        }
                        _ => println!("Unhandled: {:#?}", expr),
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parsing Failed: {}\n", e);
        }
    }
}

fn read_pair(pair: Pair<Rule>) {
    todo!()
}

fn evaluate_expr(pair: Pair<Rule>) -> f64 {
    let mut inner = pair.into_inner();
    let mut acc = evaluate_term(inner.next().unwrap());

    while let Some(op) = inner.next() {
        let next = evaluate_term(inner.next().unwrap());

        match op.as_str() {
            "+" => acc += next,
            "-" => acc -= next,
            "*" => acc *= next,
            "/" => acc /= next,
            _ => unreachable!(),
        }
    }

    acc
}

fn evaluate_term(pair: Pair<Rule>) -> f64 {
    match pair.as_rule() {
        Rule::number => pair.as_str().trim().parse().unwrap(),
        _ => unreachable!(),
    }
}
