use std::error::Error;
use std::sync::Arc;

use super::value::ListValue;
use super::value::Value;

#[derive(Clone)]
pub struct MacroValue {
    pub params: ListValue,
    pub body: Arc<Value>,
}

impl MacroValue {
    pub fn new(params: ListValue, body: Arc<Value>) -> MacroValue {
        MacroValue { params, body }
    }

    pub fn get<'a>(&self, args: &'a [Value], pattern: &Value) -> Option<&'a Value> {
        if let Some(index) = self.params.iter().position(|s| s == pattern) {
            Some(&args[index])
        } else {
            None
        }
    }

    fn expand_symbol<'a>(&'a self, args: &'a [Value], symbol: &'a Value) -> &'a Value {
        match self.get(args, symbol) {
            Some(arg) => arg, // just passing the argument through unchanged
            None => symbol,
        }
    }

    pub fn expand_body(&self, body: &Value, call: &ListValue) -> Result<Value, Box<dyn Error>> {
        use Value::*;

        let args = &call[1..];
        // TODO: check at some point that params.len() == args.len()

        let env = super::eval::default_env();

        Ok(match body {
            Macro(_) => todo!("macros making macros not implemented"),
            Symbol(ref _sym) => match self.get(args, &body) {
                Some(arg) => arg.fallible_clone()?,
                None => body.fallible_clone()?,
            },
            List(list) => {
                let mut new_list = vec![];
                if list.len() >= 1 {
                    let f = {
                        let f = &list[0];
                        match f {
                            Symbol(_) => env.eval(f)?,
                            _ => f.clone(),
                        }
                    };
                    match f {
                        Macro(_) => todo!(),
                        _ => {
                            new_list.push(f.clone());
                            for val in list[1..].iter() {
                                new_list.push(self.expand_body(val, call)?);
                            }
                        }
                    }
                }
                List(new_list.into())
            }
            UnsafeCall(_) => todo!(),
            Bool(_) | Integer(_) | Func(_) | UnsafeFunc(_) | Lambda(_) => body.fallible_clone()?,
        })
    }

    pub fn expand_recurse(val: &Value) -> Result<Value, Box<dyn Error>> {
        use Value::*;

        eprint!("UUUHHHHH\r\n");
        Ok(match val {
            Bool(_) | Integer(_) | Macro(_) | Func(_) | UnsafeFunc(_) | Lambda(_) | Symbol(_) => {
                val.fallible_clone()?
            }
            UnsafeCall(_) => todo!(),
            List(list) => {
                if val.is_nil() {
                    val.fallible_clone()?
                } else {
                    let f = &list[0];
                    let _args = &list[1..];
                    match f {
                        Macro(mac) => mac.expand_body(&mac.body.clone(), &list)?,
                        _ => {
                            let mut new_list = vec![];

                            let first = match f {
                                Symbol(sym) => super::eval::default_env().eval(f)?,
                                _ => f.clone(),
                            };
                            
                            new_list.push(first);
                            for v in list[1..].iter() {
                                new_list.push(MacroValue::expand_recurse(v)?);
                                //new_list.push(9001.into());
                            }
                            List(new_list.into())
                        }
                    }
                }
            }
        })
    }
}
