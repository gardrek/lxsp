#![cfg(test)]

use super::*;
use eval::LispEnv;
use lisp::LispValue;
//use eval::{LispEnv, EvalError};

fn get_std<'a>(base_env: &'a LispEnv) -> Result<LispEnv<'a>, Box<dyn std::error::Error>> {
    let src = std::fs::read_to_string("lisb/std.l")?;

    add_lib(&base_env, &src).into()
}

#[test]
fn lua_core() {
    let env = eval::LispEnv::default();

    parse_eval("(lua 'core)", &env).unwrap();
}

#[test]
fn std_test() {
    let base_env = eval::LispEnv::default();

    let env = get_std(&base_env).unwrap();

    let list_source = "'((a 1) (b 2) (c 3))";

    let firsts_source = "'(a b c)";

    let seconds_source = "'(1 2 3)";

    let _list = crate::parse_eval(list_source, &env).unwrap();

    let firsts = crate::parse_eval(firsts_source, &env).unwrap();

    let seconds = crate::parse_eval(seconds_source, &env).unwrap();

    use LispValue::*;

    let tests = &[
        ("(id 37)", Integer(37)),
        ("(car '(10 20 30))", Integer(10)),
        ("(car (cdr '(10 20 30)))", Integer(20)),
        ("(car (cdr (cdr '(10 20 30))))", Integer(30)),
        ("(div 100 30)", Integer(3)),
        ("(mul 100 30)", Integer(3000)),
        ("(fib 10)", Integer(55)),
        ("(nilP ())", Bool(true)),
        ("(nilP '())", Bool(true)),
        ("(nilP 't)", Bool(false)),
        ("(nilP '(t))", Bool(false)),
        ("(not false)", Bool(true)),
        ("(not true)", Bool(false)),
        ("(truthyP ())", Bool(false)),
        ("(truthyP '())", Bool(false)),
        ("(truthyP 't)", Bool(true)),
        ("(truthyP '(t))", Bool(true)),
        (&format!("(firsts {})", list_source), firsts),
        (&format!("(seconds {})", list_source), seconds),
    ];

    for (src, result) in tests {
        assert_eq!(parse_eval(src, &env).unwrap(), *result);
    }
    /*
        (longOr (fn (x y) (if (truthyP x) x y)))

        (pow (let
            '((powRecurse (fn (x y) (if (lt x 1) 1 (mul y (powRecurse (sub x 1) y))))))
            (fn (x y) (powRecurse x y))))

        (firsts (fn (l) (if (truthy l) (cons (car (car l)) (firsts (cdr l))) ())))

        (seconds (fn (l) (if (truthy l) (cons (car (cdr (car l))) (seconds (cdr l))) ())))
    */
}
