use implicit;
use explicit;
use typed;

use std::collections::HashMap;
use explicit::Type;

fn convert(env: &mut HashMap<String, Type>, uniq: &HashMap<String, String>, expr: &explicit::Expr) -> (implicit::Expr) {
    implicit::Expr::Empty
}

pub fn extract(expr: &explicit::Expr) -> (implicit::Expr, HashMap<String, Type>) {
    let mut env: HashMap<String, Type> = HashMap::new();
    let mut uniq: HashMap<String, String> = HashMap::new();

    let iexpr = convert(&mut env, &uniq, expr);

    (iexpr, env)
}
