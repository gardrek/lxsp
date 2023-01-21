use super::eval::Bindings;
use super::eval::EvalError;
use super::eval::LispEnv as LispEnv;
use std::sync::Arc;

type ListValue<'v> = Arc<[Value<'v>]>;

pub type FuncValue = for<'e> fn(&[Value], &'e LispEnv) -> Result<Value<'e>, EvalError>;
//type FuncValue = for<'v> fn(&[Value], &LispEnv) -> Result<Value<'v>, EvalError>;

#[derive(Clone)]
pub enum Value<'e> {
    Bool(bool),
    Integer(i64),
    Symbol(String),
    List(ListValue<'e>),
    Func(FuncValue),
    UnsafeFunc(FuncValue),
    Macro(MacroValue<'e>),
    Lambda(LambdaValue<'e>),
    Env(Arc<LispEnv<'e>>),
    UnsafeCall(ListValue<'e>),
    // Partially evaluated value
    //~ Partial(Partial),
}

//~ pub struct Partial {}

#[derive(Clone)]
pub struct MacroValue<'v> {
    pub args: ListValue<'v>,
    pub body: Arc<Value<'v>>,
}

#[derive(Clone)]
pub struct LambdaValue<'v> {
    pub args: Arc<Value<'v>>,
    pub body: Arc<Value<'v>>,
    pub closure: Bindings<'v>,
}

impl MacroValue<'_> {
    pub fn new<'e>(args: ListValue, body: Arc<Value>) -> MacroValue<'e> {
        MacroValue { args, body }
    }
}

impl LambdaValue<'_> {
    pub fn new<'e>(args: Arc<Value>, body: Arc<Value>, closure: Bindings) -> LambdaValue<'e> {
        LambdaValue {
            args,
            body,
            closure,
        }
    }
}

impl Value<'_> {
    pub fn fallible_clone(&self) -> Result<Value, EvalError> {
        use Value::*;

        Ok(match self {
            Bool(b) => Bool(*b),
            Integer(i) => Integer(*i),
            Symbol(s) => Symbol(s.clone()),
            List(arc) => List(arc.clone()),
            Macro(m) => Macro(m.clone()),
            Func(f) => Func(*f),
            UnsafeFunc(f) => UnsafeFunc(*f),
            Lambda(lam) => Lambda(lam.clone()),

            UnsafeCall(_) => todo!(),
        })
    }

    pub fn cons<'v>(head: &Value, tail: &Value) -> Option<Value<'v>> {
        //~ // TODO: fancy iterator chain version
        //~ Some(iter::once(x).chain(arc.iter()).collect())
        let mut list = vec![head.clone()];
        list.extend_from_slice(tail.get_list()?);
        Some(Value::List(list.into()))
    }

    pub fn head(&self) -> Option<Value> {
        let list = self.get_list()?;
        if list.is_empty() {
            return Some(Value::nil());
        }
        Some(list[0].clone())
    }

    pub fn tail(&self) -> Option<Value> {
        let list = self.get_list()?;
        if list.is_empty() {
            return Some(Value::nil());
        }
        Some(Value::List(list[1..].into()))
    }

    pub fn get_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn quoted(v: Value) -> Value {
        Value::List(vec![Value::Symbol("quote".into()), v].into())
    }

    pub fn nil() -> Value<'static> {
        Value::List(Arc::new([]))
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_symbol(&self) -> Option<&String> {
        match self {
            Value::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /*
    pub fn get_owned_symbol(self) -> Option<String> {
        match self {
            Value::Symbol(s) => Some(s),
            _ => None,
        }
    }
    */

    pub fn is_nil(&self) -> bool {
        match self {
            Value::List(list) => list.len() == 0,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false,
        }
    }

    pub fn is_atom(&self) -> bool {
        !self.is_list() || self.is_nil()
    }

    /*
    pub fn is_nonnil_atom(&self) -> bool {
        !self.is_list()
    }

    pub fn is_nonempty_list(&self) -> bool {
        !self.is_atom()
    }
    */

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_sym) => true,
            _ => false,
        }
    }

    pub fn is_list_of_symbols(&self) -> bool {
        match self {
            Value::List(list) => list.iter().fold(true, |b, v| b && v.is_symbol()),
            _ => false,
        }
    }
}

impl PartialEq for Value<'_> {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            (Symbol(a), Symbol(b)) => a == b,
            (List(a), List(b)) => a == b,

            // NOTE: the following implementations are *not* reflexive.
            // do not impl Eq without fixing this, if possible
            (Macro(_a), Macro(_b)) => false,
            (Func(_a), Func(_b)) => false,
            (UnsafeFunc(_a), UnsafeFunc(_b)) => false,
            (Lambda(_a), Lambda(_b)) => false,
            (UnsafeCall(_a), UnsafeCall(_b)) => false,

            (Bool(_a), _) => false,
            (Integer(_a), _) => false,
            (Symbol(_a), _) => false,
            (List(_a), _) => false,
            (Macro(_a), _) => false,
            (Func(_a), _) => false,
            (Lambda(_a), _) => false,
            (UnsafeFunc(_a), _) => false,
            (UnsafeCall(_a), _) => false,
        }
    }
}

// Do not uncomment unless PartialEq is modified to be reflexive
//impl Eq for Value {}

impl core::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl core::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Value::*;
        match self {
            Bool(b) => write!(f, "[{}]", b),
            Integer(i) => write!(f, "{}", i),
            Symbol(s) => write!(f, "{}", s),
            Macro(_) => write!(f, "{}", "[Macro]"),
            Func(_) => write!(f, "[Function]"),
            UnsafeFunc(_) => write!(f, "[Unsafe Function]"),
            List(list) => {
                write!(f, "(")?;
                let mut first = true;
                for v in list.iter() {
                    if first {
                        write!(f, "{}", v)?;
                    } else {
                        write!(f, " {}", v)?;
                    }
                    first = false;
                }
                write!(f, ")")?;
                Ok(())
            }
            Lambda(_lambda) => write!(f, "[Lambda]"),
            UnsafeCall(_) => write!(f, "[UnsafeCall]"),
        }
    }
}

/*
fn display(val: &Value) {
    use Value::*;
    match &val {
        Nil => write!(f, "[nil]", s),
        Integer(i64) => write!(f, "{}", s),
        Number(f64) => write!(f, "{}", s),
        Symbol(String) => write!(f, "{}", s),
        List(Vec<Value>) => write!(f, "{}", s),
    }
}
*/

impl From<()> for Value<'_> {
    fn from(_v: ()) -> Value<'static> {
        Value::nil()
    }
}

impl From<bool> for Value<'_> {
    fn from(v: bool) -> Value<'static> {
        Value::Bool(v)
    }
}

impl From<i64> for Value<'_> {
    fn from(v: i64) -> Value<'static> {
        Value::Integer(v)
    }
}

impl From<&str> for Value<'_> {
    fn from(v: &str) -> Value<'static> {
        Value::Symbol(v.to_string())
    }
}
