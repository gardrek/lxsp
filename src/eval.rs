use super::lisp::LambdaValue;
use super::lisp::LispValue;
use std::collections::HashMap;
use std::sync::Arc;

mod lua;

pub type Bindings = HashMap<String, LispValue>;

#[derive(Debug)]
pub struct LispEnv<'a> {
    bindings: HashMap<String, LispValue>,
    outer: Option<&'a LispEnv<'a>>,
}

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

impl LispEnv<'_> {
    fn get(&self, s: &String) -> Option<&LispValue> {
        match self.bindings.get(s) {
            Some(e) => Some(e),
            None => match &self.outer {
                Some(env) => env.get(s),
                None => None,
            },
        }
    }

    pub fn list<'a>(
        &'a self,
    ) -> std::collections::hash_map::Keys<'a, std::string::String, LispValue> {
        self.bindings.keys()
    }

    pub fn sorted_list<'a>(&'a self) -> Vec<&'a String> {
        let mut v: Vec<_> = self.list().collect();

        v.sort();

        v
    }

    // TODO: implement recursive version to list thing from parent
    pub fn _sorted_list_r<'a>(&'a self) -> Vec<&'a String> {
        let mut v: Vec<_> = self.list().collect();

        v.sort();

        v
    }

    fn flatten(&self) -> Bindings {
        let bindings = HashMap::new();
        self.flatten_recurse(bindings)
    }

    fn flatten_recurse(&self, mut bindings: Bindings) -> Bindings {
        bindings.extend(self.bindings.clone());

        match self.outer {
            Some(inner) => inner.flatten_recurse(bindings),
            None => bindings,
        }
    }

    pub fn eval(&self, val: &LispValue) -> Result<LispValue, EvalError> {
        use LispValue::*;
        Ok(match val {
            Bool(_)        | Integer(_) => val.fallible_clone()?,
            Symbol(s) => self
                .get(s)
                .ok_or(EvalError::String(format!(
                    "[internal eval] use of undeclared variable {}",
                    &s
                )))?
                .fallible_clone()?,
            List(list) => {
                if val.is_nil() {
                    val.fallible_clone()?
                } else {
                    let f = self.eval(&list[0])?;
                    self.apply(&f, &list[1..])?
                }
            }
            Func(_) | UnsafeFunc(_) | Lambda(_) => val.clone(),
        })
    }

    fn apply(&self, val: &LispValue, args: &[LispValue]) -> Result<LispValue, EvalError> {
        use LispValue::*;

        match val {
            Func(func) => func(args, self),
            Lambda(lambda) => {
                let inner_env = self.new_inner_env(lambda.args.clone(), args)?;
                let closure_env = LispEnv {
                    bindings: lambda.closure.clone(),
                    outer: Some(&inner_env),
                };
                closure_env.eval(&lambda.body)
            }
            UnsafeFunc(_func) => todo!(),
            Bool(_) | Integer(_) | Symbol(_) | List(_) => Err(EvalError::String(format!(
                "cannot apply {}; not a function",
                &val
            ))),
        }
    }

    pub fn new_inner_env<'a>(
        &'a self,
        params: Arc<LispValue>,
        arg_forms: &[LispValue],
    ) -> Result<LispEnv<'a>, EvalError> {
        let ks = parse_list_of_symbol_strings(params)?;
        if ks.len() != arg_forms.len() {
            return Err(EvalError::String(format!(
                "lambda call expected {} arguments, got {}",
                ks.len(),
                arg_forms.len()
            )));
        }
        let vs = self.eval_forms(arg_forms)?;
        let mut data: HashMap<String, LispValue> = HashMap::new();
        for (k, v) in ks.iter().zip(vs.iter()) {
            data.insert(k.clone(), v.clone());
        }
        Ok(LispEnv {
            bindings: data,
            outer: Some(self),
        })
    }

    fn eval_forms(&self, arg_forms: &[LispValue]) -> Result<Vec<LispValue>, EvalError> {
        arg_forms.iter().map(|x| self.eval(x)).collect()
    }

    // construct a new inner scope from a list of name-value pairs
    pub fn new_inner_from_pairs(&self, pairs: &LispValue) -> Result<LispEnv, String> {
        let mut names = vec![];
        let mut values = vec![];

        match &pairs {
            LispValue::List(l) => {
                for b in l.iter() {
                    match b {
                        LispValue::List(binding) => {
                            if binding.len() != 2 {
                                return Err(format!("[new_inner_from_pairs] not a list of pairs"));
                            }
                            // TODO: impl from/into for the error type and change this to not panic
                            names.push(binding[0].clone());
                            values.push(self.eval(&binding[1]).unwrap());
                        }
                        _ => {
                            return Err(format!("[new_inner_from_pairs] element is not a list"));
                        }
                    }
                }
            }
            _ => return Err(format!("[new_inner_from_pairs] not a list")),
        }

        // TODO: impl from/into for the error type and change this to not panic
        Ok(self
            .new_inner_env(Arc::new(LispValue::List(names.into())), &values)
            .unwrap())
    }
}

impl Default for LispEnv<'static> {
    fn default() -> LispEnv<'static> {
        default_env()
    }
}

fn eval_err(s: &'static str) -> EvalError {
    EvalError::Static(s)
}

fn parse_list_of_symbol_strings(form: Arc<LispValue>) -> Result<Vec<String>, EvalError> {
    let list = match form.as_ref() {
        LispValue::List(s) => Ok(s.clone()),
        _ => Err(EvalError::Static("expected args form to be a list")),
    }?;
    list.iter()
        .map(|x| match x {
            LispValue::Symbol(s) => Ok(s.clone()),
            _ => Err(EvalError::Static("expected symbols in the argument list")),
        })
        .collect()
}

/*
macro_rules! built_in_fn {
  ($check_fn:expr) => {{
    |args: &[LispValue], env: &LispEnv| -> Result<LispValue, LispError> {
      let floats = parse_list_of_floats(args)?;
      let first = floats.first().ok_or(LispError::Static("expected at least one number"))?;
      let rest = &floats[1..];
      fn f (prev: &f64, xs: &[f64]) -> bool {
        match xs.first() {
          Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
          None => true,
        }
      };
      Ok(LispValue::Bool(f(first, rest)))
    }
  }};
}
*/

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
        //(s.into(), UnsafeFunc(f))
        (s.into(), Func(f))
    }

    let bindings = HashMap::from([
        ("exit".into(), Symbol("exit".to_string())),
        /*func(
            "exit",
            |_args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> {
                Ok(Symbol("exit".to_string()))
            }
        ),*/
        ("false".into(), Bool(false)),
        ("true".into(), Bool(true)),
        func(
            "readf",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                let filename = match args.len() {
                    1 => {
                        if let LispValue::Symbol(package) = env.eval(&args[0])? {
                            format!("lisb/{}", format!("{}.lisp", package))
                        } else {
                            return Err(eval_err("[readf] Wrong argument type"));
                        }
                    }
                    2 => {
                        if let LispValue::Symbol(main) = env.eval(&args[0])? {
                            if let LispValue::Symbol(ext) = env.eval(&args[1])? {
                                format!("lisb/{}.{}", main, ext)
                            } else {
                                return Err(eval_err("[readf] Wrong argument type"));
                            }
                        } else {
                            return Err(eval_err("[readf] Wrong argument type"));
                        }
                    }
                    3 => {
                        if let LispValue::Symbol(folder) = env.eval(&args[0])? {
                            if let LispValue::Symbol(main) = env.eval(&args[1])? {
                                if let LispValue::Symbol(ext) = env.eval(&args[2])? {
                                    format!("{}/{}.{}", folder, main, ext)
                                } else {
                                    return Err(eval_err("[readf] Wrong argument type"));
                                }
                            } else {
                                return Err(eval_err("[readf] Wrong argument type"));
                            }
                        } else {
                            return Err(eval_err("[readf] Wrong argument type"));
                        }
                    }
                    _ => return Err(eval_err("[readf] Wrong number of arguments")),
                };
                let source = std::fs::read_to_string(&filename).expect("[readf] IO error");
                let data = crate::parse_file(&source).expect("[readf] Parse error");
                Ok(env.eval(&data)?)
            },
        ),
        func(
            "eval",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 1 {
                    return Err(eval_err("[readf] Wrong number of arguments"));
                }
                env.eval(&env.eval(&args[0])?)
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
                    _ => return Err(eval_err("[let] Wrong argument type (not list)")),
                }

                env.new_inner_env(Arc::new(LispValue::List(names.into())), &values)?
                    .eval(&args[1])
            },
        ),
        func(
            "include",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                if args.len() != 2 {
                    eprintln!("{} args", args.len());
                    return Err(eval_err("[include] Wrong number of arguments"));
                }
                match crate::parse_eval(&format!("(let (readf {}) {})", &args[0], &args[1]), env) {
                    Err(e) => panic!("{}", e),
                    Ok(ok) => Ok(ok),
                }
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
            "unsafe",
            |_args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> { Ok(().into()) },
        ),
        unsafe_func("lua", lua::run_lua_file_from_lisp_args),
        /*unsafe_func(
            "lua",
            |args: &[LispValue], env: &LispEnv| -> Result<LispValue, EvalError> {
                use std::process::Command;

                if args.len() != 1 {
                    return Err(eval_err("[lua] Wrong number of arguments"));
                }

                fn get_lua_lib_filename(lib_name: &str) -> String {
                    format!("src/{}.lua", lib_name)
                }

                let a = env.eval(&args[0])?;

                let lib_name = a
                    .get_symbol()
                    .ok_or(eval_err("argument to lua not a symbol"))?;

                let filename = get_lua_lib_filename(lib_name);

                let mut child = Command::new("lua")
                    .arg(&filename)
                    .spawn()
                    .expect("Lua failed to start");

                let _ecode = child.wait().expect("failed to wait on child");

                Ok(().into())
            },
        ),*/
        /*
        func(
            "",
            |_args: &[LispValue], _env: &LispEnv| -> Result<LispValue, EvalError> {
                Ok(())
            },
        ),
        */
    ]);

    LispEnv {
        bindings,
        outer: None,
    }
}
