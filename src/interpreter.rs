use crate::{EletronParser, Rule};
use core::fmt;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub type Environment = HashMap<String, Value>;

pub fn evaluate(env: &mut Environment, pair: Pair<Rule>) -> Value {
    match pair.as_rule() {
        // Regras Estruturais (despacham para a função apropriada)
        Rule::stmt => eval_stmt_logic(env, pair),
        Rule::let_stmt => eval_let_stmt(env, pair),

        // Regras de Expressão Binária (usam a mesma lógica)
        Rule::expr | Rule::sum_expr | Rule::mul_expr | Rule::pow_expr => {
            eval_binary_expression(env, pair)
        }

        // Casos Base da Recursão
        Rule::factor => eval_factor(env, pair),
        Rule::number => Value::Float(pair.as_str().parse().unwrap()),
        Rule::ident => {
            //dbg!(&env);
            env.get(pair.as_str()).expect(format!("Variável não encontrada {}",pair.as_str()).as_str()).clone()
        },
        // Ignora regras que não produzem valor
        Rule::EOI => Value::Null,
        
        _ => unreachable!("Regra de avaliação não tratada: {:?}", pair.as_rule()),
    }
}

/// Função HELPER genérica para todas as expressões binárias.
/// Substitui `eval_sum_expr`, `eval_mul_expr`, e `eval_pow_expr`.
fn eval_binary_expression(env: &mut Environment, pair: Pair<Rule>) -> Value {
    let mut inner = pair.into_inner();
    // O primeiro filho é sempre o operando inicial
    let mut acc = evaluate(env, inner.next().unwrap());

    // Loop para os pares de (operador, operando)
    while let Some(op) = inner.next() {
        let rhs = evaluate(env, inner.next().unwrap());
        acc = apply_binary_op(acc, op.as_str(), rhs);
    }
    acc
}

/// Lógica específica para `let`
fn eval_let_stmt(env: &mut Environment, pair: Pair<Rule>) -> Value {
    let mut inner_let = pair.into_inner();

    let _keyword = inner_let.next();

    let name = inner_let.next().unwrap().as_str().to_string();
    let value = evaluate(env, inner_let.next().unwrap()); 

    //dbg!(&name, &value);

    env.insert(name, value);
    Value::Null
}

/// Lógica que desembrulha `stmt`
fn eval_stmt_logic(env: &mut Environment, pair: Pair<Rule>) -> Value {
    // `stmt` sempre tem um filho: `let_stmt` ou `expr`
    let inner_pair = pair.into_inner().next().unwrap();
    evaluate(env, inner_pair) // Usa o despachante principal
}

/// A função `factor` agora também usa o despachante principal.
fn eval_factor(env: &mut Environment, pair: Pair<Rule>) -> Value {
    let inner = pair.into_inner().next().unwrap();
    // A chamada recursiva agora é sempre para `evaluate`.
    evaluate(env, inner)
}

// A função que aplica o operador (sem alterações)
fn apply_binary_op(lhs: Value, op: &str, rhs: Value) -> Value {
    // ... seu código para apply_binary_op continua aqui ...
    match (lhs, rhs) {
        (Value::Float(l), Value::Float(r)) => match op {
            "+" => Value::Float(l + r),
            "-" => Value::Float(l - r),
            "*" => Value::Float(l * r),
            "/" => Value::Float(l / r),
            "**" | "^" => Value::Float(l.powf(r)),
            _ => panic!("Operador '{}' não suportado para floats", op),
        },
        (Value::String(l), Value::String(r)) if op == "+" => Value::String(l + &r),
        (l, r) => panic!("Erro de tipo: não é possível aplicar '{}' entre {} e {}", op, l, r),
    }
}
