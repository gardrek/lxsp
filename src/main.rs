mod cli;
mod eval;
mod lisp;
mod parse;
mod scan;

use once_cell::sync::OnceCell;
use clap::Parser;

pub static BASE_ENV: OnceCell<eval::LispEnv<'_>> = OnceCell::new();

pub static STD_ENV: OnceCell<eval::LispEnv<'_>> = OnceCell::new();

pub fn parse_eval(
    source: &str,
    env: &eval::LispEnv,
) -> Result<lisp::LispValue, Box<dyn std::error::Error>> {
    Ok(env.eval(&parse_file(&source)?)?)
}

pub fn parse_file(source: &str) -> Result<lisp::LispValue, Box<dyn std::error::Error>> {
    let scanner = scan::Scanner::new(source);

    let tokens = scanner.collect::<Vec<_>>();

    let (parsed_exp, _rest) = parse::parse(&tokens)?;

    Ok(parsed_exp)
}

fn input_line() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line
}

fn add_lib<'e, 's>(
    base_env: &'e eval::LispEnv,
    src: &'s str,
) -> Result<eval::LispEnv<'e>, Box<dyn std::error::Error>> {
    let pairs = parse_file(&src)?;

    Ok(base_env.new_inner_from_pairs(&pairs)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let args = cli::ArgStruct::parse();
    
    BASE_ENV.set(eval::LispEnv::default()).unwrap();

    let base_env = BASE_ENV.get().unwrap();

    let env = if args.nostd {
        base_env
    } else {
        let src = std::fs::read_to_string("lisb/std.l")?;
        //~ let src = std::fs::read_to_string("lisb/lam.l")?;
    
        let std_inner = add_lib(&base_env, &src)?;
    
        STD_ENV.set(std_inner).unwrap();
    
        STD_ENV.get().unwrap()
    };

    let built_ins = base_env.sorted_list();

    print!("Built-ins: ");
    for s in &built_ins {
        print!("{} ", s);
    }
    println!();

    let std_bindings = env.sorted_list();

    print!("Standard Library: ");
    for s in &std_bindings {
        print!("{} ", s);
    }
    println!();

    println!();
    println!("---");
    println!();

    use cli::Session;

    let session = Session::new();

    use terminal::{Action, Color};

    'main: loop {
        /*
        session.terminal
            .act(Action::SetBackgroundColor(Color::White))?;
        */
        session
            .terminal
            .act(Action::SetForegroundColor(Color::Blue))?;
        print!("λ ");
        session
            .terminal
            .act(Action::SetForegroundColor(Color::Reset))?;

        std::io::stdout().flush()?;
        let line = input_line();
        /*let src = if line.trim().is_empty() {
            format!("(include 'std ())")
        } else {
            format!("(include 'std {})", &line)
        };*/
        if line.trim().is_empty() {
            continue;
        } else if line.trim() == "exit" {
            break Ok(());
        }
        let result = parse_eval(&line, env);
        match result {
            Ok(res) => {
                println!("=> {}", res);
                if res == lisp::LispValue::Symbol("exit".to_string()) {
                    break 'main Ok(());
                }
            }
            Err(e) => println!("[ERR] {}", e),
        }
    }
}

/*
#[rustfmt::skip]
const ALPHABET: [&str; 64] = [
     " ", "0", ")", // $1
          "1", "'", // $2
          "2",       "a", "g",
               ",", // $3
          "3",       "b", "h",
                     "f", "i", "j",
    "_9",      "(", // $4
          "4",       "c", "k", "d", "l", "m",
    "_8",            "e", "n", "o",
    "_7",            "p",
    "_6",
    "_5",
    "_@",      "@", // $5
           "5",
           "6",      "q",
           "7",      "r", "s",
    "_e",  "8",      "t", "u",
    "_d",            "v",
    "_c",
    "_4",
    "_(",  "9",      "w", "x",
    "_f",            "y",
    "_b",
    "_3",
    "_,",            "z",
    "_a",
    "_2",
    "_'",
    "_1",
    "_)",
    "_0",
   "DEL",
];
*/
//const fn calcuate_reverse_alphabet() -> [&'static str; 64] {}

mod tests; /*{
               #![cfg(test)]

               use super::*;
               use lisp::LispValue;
               use eval::LispEnv;
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

                   use LispValue::*;

                   let tests = &[
                       ("(id 37)", Integer(37)),
                       ("(car '(10 20 30))", Integer(10)),
                       ("(car (cdr '(10 20 30)))", Integer(20)),
                       ("(car (cdr (cdr '(10 20 30))))", Integer(30)),
                   ];

                   for (src, result) in tests {
                       assert_eq!(parse_eval(src, &env).unwrap(), *result);
                   }
           /*
               (div (let '((
                   divRecurse
                   (fn
                       (d v i)
                       (if
                           (lt (sub d v) 1)
                           (if (eq (sub d v) 0) (add i 1) i)
                           (divRecurse (sub d v) v (add i 1))))))
                   (fn (d v) (divRecurse d v 0))))

               (fib (fn (n) (if (eq n 0) 0 (if (eq n 1) 1 (add (fib (sub n 1)) (fib (sub n 2)))))))

               (nilP (fn (x) (if (eq x ()) true false)))

               (not (fn (x) (if x false true)))

               (truthyP (fn (x) (if (eq x false) false (not (nilP x)))))

               (longOr (fn (x y) (if (truthyP x) x y)))

               (mul (let
                   '((mulRecurse (fn (x y) (if (lt x 1) 0 (add y (mulRecurse (sub x 1) y))))))
                   (fn (x y) (if (lt x y) (mulRecurse x y) (mulRecurse y x)))))

               (pow (let
                   '((powRecurse (fn (x y) (if (lt x 1) 1 (mul y (powRecurse (sub x 1) y))))))
                   (fn (x y) (powRecurse x y))))

               (firsts (fn (l) (if (truthy l) (cons (car (car l)) (firsts (cdr l))) ())))

               (seconds (fn (l) (if (truthy l) (cons (car (cdr (car l))) (seconds (cdr l))) ())))
           */
               }
           }
           // */
