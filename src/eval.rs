use super::env::LispEnv;
use super::value::LambdaValue;
use super::value::MacroValue;
use super::value::Value as LispValue;
use std::collections::HashMap;
use std::sync::Arc;

mod lua;

#[derive(Debug)]
pub enum EvalError {
    Static(&'static str),
    String(String),
    AttemptToClone,
}

impl std::error::Error for EvalError {}

impl core::fmt::Display for EvalError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use EvalError::*;
        match &self {
            Static(s) => write!(f, "{}", s),
            String(s) => write!(f, "{}", s),
            AttemptToClone => write!(f, "Attempt To Clone"),
        }
    }
}

pub fn eval_err(s: &'static str) -> EvalError {
    EvalError::Static(s)
}

pub fn default_env<'a>() -> LispEnv<'a> {
    use LispValue::*;

    fn func(
        s: &'static str,
        f: fn(&[LispValue], &LispEnv) -> Result<LispValue, EvalError>,
    ) -> (String, LispValue) {
        (s.into(), Func(f))
    }

    fn unsafe_func(
        s: &'static str,
        f: fn(&[LispValue], &LispEnv) -> Result<LispValue, EvalError>,
    ) -> (String, LispValue) {
        (s.into(), UnsafeFunc(f))
    }

    let bindings = HashMap::from([
        ("exit".into(), Symbol("exit".to_string())),
        ("false".into(), Bool(false)),
        ("true".into(), Bool(true)),
        func(
            "cons",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[cons] Wrong number of arguments"));
                }
                LispValue::cons(&env.eval(&args[0])?, &env.eval(&args[1])?)
                    .ok_or(eval_err("[cons] tail not a list"))
            },
        ),
        func(
            "car",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[car] Wrong number of arguments"));
                }
                env.eval(&args[0])?
                    .head()
                    .ok_or(eval_err("[car] Wrong argument type"))
            },
        ),
        func(
            "cdr",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[cdr] Wrong number of arguments"));
                }
                env.eval(&args[0])?
                    .tail()
                    .ok_or(eval_err("[car] Wrong argument type"))
            },
        ),
        func(
            "quote",
            |args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[quote] Wrong number of arguments"));
                }
                Ok(args[0].clone())
            },
        ),
        func(
            "eq",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[eq] Wrong number of arguments"));
                }
                let a = env.eval(&args[0])?;
                let b = env.eval(&args[1])?;
                Ok(Bool(a == b))
            },
        ),
        func(
            "macro",
            |args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[macro] Wrong number of arguments"));
                }
                if !args[0].is_list_of_symbols() {
                    return Err(eval_err("[macro] Wrong argument type"));
                }
                let a = args[0].get_list().unwrap().to_owned();
                let b = args[1].clone();
                Ok(Macro(MacroValue::new(a.into(), Arc::new(b))))
            },
        ),
        func(
            "fn",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[fn] Wrong number of arguments"));
                }
                if !args[0].is_list_of_symbols() {
                    return Err(eval_err("[fn] Wrong argument type"));
                }
                let a = args[0].clone();
                let b = args[1].clone();
                Ok(Lambda(LambdaValue::new(
                    Arc::new(a),
                    Arc::new(b),
                    env.flatten(),
                )))
            },
        ),
        func(
            "if",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 3 {
                    return Err(eval_err("[if] Wrong number of arguments"));
                }
                let condition = env
                    .eval(&args[0])?
                    .get_bool()
                    .ok_or(eval_err("[if] Wrong argument type"))?;
                let result = if condition {
                    env.eval(&args[1])
                } else {
                    env.eval(&args[2])
                };
                Ok(result?)
            },
        ),
        func(
            "atom",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[atom] Wrong number of arguments"));
                }
                let a = env.eval(&args[0])?;
                Ok(Bool(a.is_atom()))
            },
        ),
        func(
            "lt",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[lt] Wrong number of arguments"));
                }
                let a = env
                    .eval(&args[0])?
                    .get_int()
                    .ok_or(eval_err("[lt] Wrong argument type"))?;
                let b = env
                    .eval(&args[1])?
                    .get_int()
                    .ok_or(eval_err("[lt] Wrong argument type"))?;
                Ok(Bool(a < b))
            },
        ),
        func(
            "gt",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[lt] Wrong number of arguments"));
                }
                let a = env
                    .eval(&args[0])?
                    .get_int()
                    .ok_or(eval_err("[lt] Wrong argument type"))?;
                let b = env
                    .eval(&args[1])?
                    .get_int()
                    .ok_or(eval_err("[lt] Wrong argument type"))?;
                Ok(Bool(a > b))
            },
        ),
        func(
            "add",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                // TODO: fix this to have proper error propogation instead of panicking
                let sum = args
                    .iter()
                    .fold(0, |sum, a| sum + env.eval(a).unwrap().get_int().unwrap());

                //.ok_or(eval_err("[add] Wrong argument type"))?

                Ok(Integer(sum))
            },
        ),
        func(
            "sub",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[sub] Wrong number of arguments"));
                }
                let a = env
                    .eval(&args[0])?
                    .get_int()
                    .ok_or(eval_err("[sub] Wrong argument type"))?;
                let b = env
                    .eval(&args[1])?
                    .get_int()
                    .ok_or(eval_err("[sub] Wrong argument type"))?;
                Ok(Integer(a - b))
            },
        ),
        func(
            "list",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                let mut v = Vec::with_capacity(args.len());
                for arg in args {
                    v.push(env.eval(arg)?);
                }
                Ok(List(v.into()))
            },
        ),
        func(
            "let",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[let] Wrong number of arguments"));
                }

                let bindings_list = env.eval(&args[0])?;

                let mut names = vec![];
                let mut values = vec![];

                //eprintln!("{}", &bindings_list);

                match &bindings_list {
                    LispValue::List(l) => {
                        for b in l.iter() {
                            match b {
                                LispValue::List(binding) => {
                                    if binding.len() != 2 {
                                        return Err(eval_err("[let] Wrong argument format"));
                                    }
                                    names.push(binding[0].clone());
                                    values.push(env.eval(&binding[1])?);
                                }
                                _ => {
                                    return Err(eval_err(
                                        "[let] Wrong argument type (not list of list)",
                                    ))
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(EvalError::String(format!(
                            "[let] Wrong argument type: [[{}]]",
                            bindings_list
                        )))
                    }
                }

                env.new_inner_env(Arc::new(LispValue::List(names.into())), &values)?
                    .eval(&args[1])
            },
        ),
        func(
            "eval",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[eval] Wrong number of arguments"));
                }
                env.eval(&env.eval(&args[0])?)
            },
        ),
        func(
            "unsafe",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[unsafe] Wrong number of arguments"));
                }

                let inner_env = env.new_unsafer_env();

                Ok(inner_env.eval(&args[0])?)

                /*
                let val = env.eval(&args[0])?;
                if let UnsafeCall(list) = val {
                    if list.len() == 0 {
                        return Ok(().into());
                    } else {
                        let f = env.eval(&list[0])?;
                        Ok(env.apply(&f, &list[1..])?)
                    }
                } else {
                    Ok(val)
                }
                */
                /*
                if let Some(list) = val.get_list() {
                    if list.len() == 0 {
                        return Ok(().into());
                    } else {
                        let f = env.eval(&list[0])?;
                        Ok(env.apply(&f, &list[1..])?)
                    }
                } else {
                    Ok(val)
                }
                */
            },
        ),
        unsafe_func(
            "spookyAdd",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    return Err(eval_err("[sub] Wrong number of arguments"));
                }
                let a = env
                    .eval(&args[0])?
                    .get_int()
                    .ok_or(eval_err("[sub] Wrong argument type"))?;
                let b = env
                    .eval(&args[1])?
                    .get_int()
                    .ok_or(eval_err("[sub] Wrong argument type"))?;
                Ok(Integer(a + b))
            },
        ),
        unsafe_func("lua", lua::run_lua_file_from_lisp_args),
        unsafe_func(
            "readf",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                let filename = match args.len() {
                    1 => {
                        if let LispValue::Symbol(package) = env.eval(&args[0])? {
                            format!("lisb/{}.l", package)
                        } else {
                            return Err(eval_err("[readf] Wrong argument type (single argument)"));
                        }
                    }
                    2 => {
                        if let LispValue::Symbol(main) = env.eval(&args[0])? {
                            if let LispValue::Symbol(ext) = env.eval(&args[1])? {
                                format!("lisb/{}.{}", main, ext)
                            } else {
                                return Err(eval_err("[readf] Wrong argument type (first of two)"));
                            }
                        } else {
                            return Err(eval_err("[readf] Wrong argument type (second of two)"));
                        }
                    }
                    3 => {
                        if let LispValue::Symbol(folder) = env.eval(&args[0])? {
                            if let LispValue::Symbol(main) = env.eval(&args[1])? {
                                if let LispValue::Symbol(ext) = env.eval(&args[2])? {
                                    format!("{}/{}.{}", folder, main, ext)
                                } else {
                                    return Err(eval_err(
                                        "[readf] Wrong argument type (first of three)",
                                    ));
                                }
                            } else {
                                return Err(eval_err(
                                    "[readf] Wrong argument type (second of three)",
                                ));
                            }
                        } else {
                            return Err(eval_err("[readf] Wrong argument type (third of three)"));
                        }
                    }
                    _ => return Err(eval_err("[readf] Wrong number of arguments")),
                };
                let source = std::fs::read_to_string(&filename).expect("[readf] IO error");
                let data = crate::parse_string(&source).expect("[readf] Parse error");
                //~ Ok(env.eval(&data)?)
                Ok(data)
            },
        ),
        unsafe_func(
            "include",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    eprintln!("{} args", args.len());
                    return Err(eval_err("[include] Wrong number of arguments"));
                }

                /*
                let filename = if let LispValue::Symbol(package) = env.eval(&args[0])? {
                    format!("lisb/{}.l", package)
                } else {
                    return Err(eval_err("[include] Wrong argument type (first of three)"));
                };

                let source = std::fs::read_to_string(&filename).expect("[include] IO error");
                let data = crate::parse_string(&source).expect("[include] Parse error");
                &data
                */

                match crate::parse_eval(&format!("(let (readf {}) {})", &args[0], &args[1]), &env) {
                    Err(e) => panic!("{}", e),
                    Ok(ok) => Ok(ok),
                }
            },
        ),
        /*
        func(
            "",
            |_args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> {
                todo!()
            },
        ),
        */
    ]);

    LispEnv::from_hashmap(bindings)
}
