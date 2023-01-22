use super::eval::{eval_err, EvalError};
use super::value::MacroValue;
use super::value::Value as LispValue;
use std::collections::HashMap;
use std::sync::Arc;

pub type Bindings = HashMap<String, LispValue>;

type EvalFn = for<'e, 'v, 'i> fn(&'e LispEnv<'i>, &'v LispValue) -> Result<LispValue, EvalError>;

#[derive(Debug)]
pub struct LispEnv<'e> {
    bindings: Bindings,
    outer: Option<&'e LispEnv<'e>>,
    //_outer: Option<&'e LispEnv<'e>>,
    unsafe_level: usize,
}

impl LispEnv<'_> {
    pub fn _empty() -> LispEnv<'static> {
        LispEnv::from_hashmap(HashMap::new())
    }

    pub fn new<'e>(
        bindings: Bindings,
        outer: Option<&'e LispEnv<'e>>,
        unsafe_level: usize,
    ) -> LispEnv {
        LispEnv {
            bindings,
            outer,
            //_outer,
            unsafe_level,
        }
    }

    pub fn new_inner_from_parts(&self, bindings: Bindings, unsafe_level: usize) -> LispEnv {
        LispEnv::new(bindings, Some(self), unsafe_level)
    }

    pub fn from_hashmap(bindings: Bindings) -> LispEnv<'static> {
        LispEnv::new(bindings, None, 0)
    }

    pub fn get(&self, s: &String) -> Option<&LispValue> {
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

    pub fn flatten(&self) -> Bindings {
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
        LispEnv::_eval(self, val)
    }

    pub fn delegated_eval(&self, val: &LispValue, eval_fn: EvalFn) -> Result<LispValue, EvalError> {
        use LispValue::*;

        Ok(match val {
            Bool(_) | Integer(_) | Macro(_) | Func(_) | UnsafeFunc(_) | Lambda(_) => {
                val.fallible_clone()?
            }
            UnsafeCall(_) => todo!(),
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
                    let f = eval_fn(self, &list[0])?;
                    //self.apply(&f, &list[1..])?
                    match f {
                        Macro(_) => eval_fn(self, val)?,
                        _ => self.outer_apply(&f, &list[1..])?,
                    }
                }
            }
        })
    }

    fn outer_apply(&self, val: &LispValue, args: &[LispValue]) -> Result<LispValue, EvalError> {
        use LispValue::*;

        match val {
            Macro(_) => todo!("Macro expansion at runtime"),
            Lambda(_) | Func(_) | UnsafeFunc(_) => self.apply(val, args),
            //~ UnsafeFunc(_) => UnsafeCall(list.clone()),
            Bool(_) | Integer(_) | Symbol(_) | List(_) | UnsafeCall(_) => {
                return Err(EvalError::String(format!(
                    "[internal fn: eval] value cannot be called: {}",
                    val
                )))
            }
        }
    }

    pub fn _eval(env: &LispEnv, val: &LispValue) -> Result<LispValue, EvalError> {
        env.delegated_eval(val, LispEnv::_eval)
    }

    fn macro_expand(&self, mac: &MacroValue, args: &[LispValue]) -> Result<LispValue, EvalError> {
        let inner_env = self.new_inner_env(LispValue::List(mac.params.clone()).into(), args)?;
        let body = (*mac.body).clone();
        Ok(inner_env.macro_eval(&body)?)
    }

    pub fn macro_eval(&self, val: &LispValue) -> Result<LispValue, EvalError> {
        LispEnv::_macro_eval(self, val)
    }

    fn _macro_eval(env: &LispEnv, val: &LispValue) -> Result<LispValue, EvalError> {
        use LispValue::*;

        match val {
            //Macro(_mac) => Ok(val.clone()),
            LispValue::List(list) => {
                let f = env.delegated_eval(&list[0], LispEnv::_macro_eval)?;
                match f {
                    Macro(mac) => Ok(env.macro_expand(&mac, &list[1..])?),
                    _ => Ok(env.outer_apply(&f, &list[1..])?),
                }
            }
            _ => env.delegated_eval(val, LispEnv::_macro_eval),
        }
    }

    /*
    fn _reduce1() {
        let env = LispEnv::empty();
        let _ = env.reduce(&LispValue::nil());
        todo!()
    }
    */

    // TODO: change the things that can't be reduced to instead return LispValue::Partial
    pub fn reduce(&self, val: &LispValue) -> Result<LispValue, EvalError> {
        use LispValue::*;

        Ok(match val {
            Bool(_) | Integer(_) | Macro(_) | Func(_) | UnsafeFunc(_) | Lambda(_) => {
                val.fallible_clone()?
            }
            UnsafeCall(_) => todo!(),
            Symbol(s) => {
                let maybe_env_value = self.get(s);
                match maybe_env_value {
                    Some(env_value) => env_value.fallible_clone()?,
                    None => val.fallible_clone()?,
                }
            }
            List(list) => {
                if val.is_nil() {
                    val.fallible_clone()?
                } else {
                    let f = self.reduce(&list[0])?;
                    match f {
                        Macro(_) | Func(_) | Lambda(_) => self.apply(&f, &list[1..])?,
                        UnsafeFunc(_) => val.fallible_clone()?,
                        UnsafeCall(_) => todo!(),
                        Bool(_) | Integer(_) | Symbol(_) | List(_) => {
                            return Err(EvalError::String(format!(
                                "[internal fn: reduce] value cannot be called: {}",
                                &f
                            )))
                        }
                    }
                }
            }
        })
    }

    fn apply(&self, val: &LispValue, args: &[LispValue]) -> Result<LispValue, EvalError> {
        use LispValue::*;

        match val {
            Func(func) => func(args, self),
            UnsafeFunc(func) => {
                if self.unsafe_level == 0 {
                    return Err(eval_err(
                        "[internal fn: apply] attempt to call unsafe function without `unsafe`",
                    ));
                }
                func(args, self)
            }
            Lambda(lambda) => {
                let inner_env = self.new_inner_env(lambda.args.clone(), args)?;
                let closure_env =
                    inner_env.new_inner_from_parts(lambda.closure.clone(), self.unsafe_level);
                closure_env.eval(&lambda.body)
            }
            Macro(_) => todo!("runtime macro expansion"),
            Bool(_) | Integer(_) | Symbol(_) | List(_) | UnsafeCall(_) => Err(EvalError::String(
                format!("cannot apply {}; not a function", &val),
            )),
        }
    }

    pub fn new_unsafer_env<'a>(&'a self) -> LispEnv {
        self.new_inner_from_parts(Default::default(), self.unsafe_level + 1)
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
        let mut data: Bindings = HashMap::new();
        for (k, v) in ks.iter().zip(vs.iter()) {
            data.insert(k.clone(), v.clone());
        }
        Ok(self.new_inner_from_parts(data, 0))
    }

    fn eval_forms(&self, arg_forms: &[LispValue]) -> Result<Vec<LispValue>, EvalError> {
        arg_forms.iter().map(|x| self.eval(x)).collect()
    }

    // construct a new inner scope from a list of name-value pairs
    // TODO: make this take ListValue (LIST with a T) for the pairs arg
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
        crate::eval::default_env()
    }
}

// TODO: investigate if this should take ListValue as an argument
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
