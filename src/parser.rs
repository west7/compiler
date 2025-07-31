use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "eletron.pest"]
pub struct EletronParser;

/* 
pub fn eval_expr(pair: Pair<Rule>, nl_fn: impl Fn(Pair<Rule>) -> f64) -> f64 {
    let mut inner = pair.into_inner();
    let mut acc = nl_fn(inner.next().unwrap());

    while let Some(op) = inner.next() {
        let rhs = nl_fn(inner.next().unwrap());
        match op.as_str() {
            "+" => acc += rhs,
            "-" => acc -= rhs,
            "*" => acc *= rhs,
            "/" => acc /= rhs,
            "**" => acc = acc.powf(rhs),
            _ => unreachable!(),
        }
    }
    acc
}

pub fn eval_sum_expr(pair: Pair<Rule>) -> f64 {
    eval_expr(pair, eval_mul_expr)
}

pub fn eval_mul_expr(pair: Pair<Rule>) -> f64 {
    eval_expr(pair, eval_pow_expr)
}

pub fn eval_pow_expr(pair: Pair<Rule>) -> f64 {
    let mut inner = pair.into_inner();
    let lhs = eval_factor_expr(inner.next().unwrap());

    if let Some(op) = inner.next() {
        let rhs = eval_pow_expr(inner.next().unwrap()); // Chamada recursiva aqui
        match op.as_str() {
            "**" | "^" => lhs.powf(rhs),
            _ => unreachable!(),
        }
    } else {
        lhs
    }
}   

pub fn eval_factor_expr(pair: Pair<Rule>) -> f64 {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => inner.as_str().trim().parse().unwrap(),
        Rule::sum_expr => eval_sum_expr(inner),
        _ => unreachable!(),
    }
}
 */