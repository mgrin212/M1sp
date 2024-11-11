use std::collections::HashMap;

use crate::{BinaryOp, Definition, Expr, Program, UnaryOp};

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    Pair(Box<Value>, Box<Value>),
    Function(String),
}

pub fn interpret(Program(defs, body): Program) -> Result<Value, Expr> {
    interpret_expr(defs, HashMap::new(), &body)
}

fn is_defn(defs: Vec<Definition>, name: String) -> bool {
    match defs.into_iter().find(|d| d.0 == name) {
        Some(_) => true,
        _ => false,
    }
}

pub fn interpret_expr(
    defns: Vec<Definition>,
    env: HashMap<String, Value>,
    expr: &Expr,
) -> Result<Value, Expr> {
    match expr {
        Expr::Num(n) => Ok(Value::Number(*n)),
        Expr::Bool(b) => Ok(Value::Boolean(*b)),
        Expr::Id(s) if env.contains_key(s) => env.get(s).map(|r| Ok(r.clone())).unwrap(),
        Expr::Id(s) if is_defn(defns.clone(), s.clone()) => Ok(Value::Function(s.clone())),
        Expr::BinOp(op, e1, e2) => interp_binary_prim(defns.clone(), env, op, e1, e2),
        Expr::UnOp(op, e) => interp_unary_prim(defns.clone(), env, op, e),
        Expr::Call(f, args) => match defns.clone().into_iter().find(|d| d.0 == *f) {
            Some(d) if args.len() == d.1.len() => {
                let vals = args
                    .into_iter()
                    .map(|arg| interpret_expr(defns.clone(), env.clone(), arg))
                    .flatten()
                    .collect::<Vec<Value>>();
                let fenv =
                    d.1.into_iter()
                        .zip(vals)
                        .collect::<HashMap<String, Value>>();
                interpret_expr(defns, fenv, &d.2)
            }
            _ => Err(expr.clone()),
        },
        Expr::If(cond, then_expr, else_expr) => {
            match interpret_expr(defns.clone(), env.clone(), cond) {
                Ok(Value::Boolean(true)) => interpret_expr(defns.clone(), env.clone(), then_expr),
                Ok(Value::Boolean(false)) => interpret_expr(defns.clone(), env.clone(), else_expr),
                _ => Err(expr.to_owned()),
            }
        }
        Expr::Let(vars, body) => {
            let mut f_env = env.clone();
            vars.into_iter()
                .map(|(s, e)| (s, interpret_expr(defns.clone(), env.clone(), e)))
                .for_each(|(s, eval)| match eval {
                    Ok(v) => drop(f_env.insert(s.clone(), v)),
                    Err(_) => (),
                });

            interpret_expr(defns, f_env, body)
        }
        _ => Err(expr.clone()),
    }
}

fn interp_unary_prim(
    defns: Vec<Definition>,
    env: HashMap<String, Value>,
    op: &UnaryOp,
    e: &Expr,
) -> Result<Value, Expr> {
    match op {
        UnaryOp::Add1 => match interpret_expr(defns, env, e) {
            Ok(Value::Number(n)) => Ok(Value::Number(n + 1)),
            _ => Err(e.clone()),
        },
        UnaryOp::Not => match interpret_expr(defns, env, e) {
            Ok(Value::Boolean(val)) => Ok(Value::Boolean(val)),
            _ => Err(e.clone()),
        },
        UnaryOp::IsZero => match interpret_expr(defns, env, e) {
            Ok(Value::Number(n)) => Ok(Value::Boolean(n == 0)),
            _ => Err(e.clone()),
        },
        _ => Err(e.clone()),
    }
}

fn interp_binary_prim(
    defns: Vec<Definition>,
    env: HashMap<String, Value>,
    op: &BinaryOp,
    e1: &Expr,
    e2: &Expr,
) -> Result<Value, Expr> {
    match (
        op,
        interpret_expr(defns.clone(), env.clone(), e1),
        interpret_expr(defns.clone(), env.clone(), e2),
    ) {
        (BinaryOp::Add, Ok(Value::Number(n1)), Ok(Value::Number(n2))) => Ok(Value::Number(n1 + n2)),
        (BinaryOp::Sub, Ok(Value::Number(n1)), Ok(Value::Number(n2))) => Ok(Value::Number(n1 - n2)),
        (BinaryOp::Eq, v1, v2) => match (v1, v2) {
            (Ok(Value::Number(n1)), Ok(Value::Number(n2))) => Ok(Value::Boolean(n1 == n2)),
            (Ok(Value::Boolean(n1)), Ok(Value::Boolean(n2))) => Ok(Value::Boolean(n1 == n2)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            _ => Err(Expr::Id("Eq".to_string())),
        },
        (BinaryOp::Lt, Ok(Value::Number(n1)), Ok(Value::Number(n2))) => Ok(Value::Boolean(n1 < n2)),
        _ => Err(Expr::Id("Eq".to_string())),
    }
}
