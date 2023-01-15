use super::eval::Bindings;
use super::eval::EvalError;
use super::eval::LispEnv;
use std::sync::Arc;

#[derive(Clone)]
pub enum LispValue {
    Bool(bool),
    Integer(i64),
    Symbol(String),
    List(Arc<[LispValue]>),
    Func(fn(&[LispValue], &LispEnv) -> Result<LispValue, EvalError>),
    UnsafeFunc(fn(&[LispValue], &LispEnv) -> Result<LispValue, EvalError>),
    Lambda(LambdaValue),
    //~ Env(Arc<LispEnv<'static>>),

    // Partially evaluated value
    //~ Partial(Partial),
}

//~ pub struct Partial {}

#[derive(Clone)]
pub struct LambdaValue {
    pub args: Arc<LispValue>,
    pub body: Arc<LispValue>,
    pub closure: Bindings,
}

impl LambdaValue {
    pub fn new(args: Arc<LispValue>, body: Arc<LispValue>, closure: Bindings) -> LambdaValue {
        LambdaValue {
            args,
            body,
            closure,
        }
    }
}

impl LispValue {
    pub fn fallible_clone(&self) -> Result<LispValue, EvalError> {
        use LispValue::*;

        Ok(match self {
            Bool(b) => Bool(*b),
            Integer(i) => Integer(*i),
            Symbol(s) => Symbol(s.clone()),
            List(arc) => List(arc.clone()),
            Func(f) => Func(*f),
            UnsafeFunc(f) => UnsafeFunc(*f),
            Lambda(lamda) => Lambda(lamda.clone()),
        })
    }

    pub fn cons(head: &LispValue, tail: &LispValue) -> Option<LispValue> {
        //~ // TODO: fancy iterator chain version
        //~ Some(iter::once(x).chain(arc.iter()).collect())
        let mut list = vec![head.clone()];
        list.extend_from_slice(tail.get_list()?);
        Some(LispValue::List(list.into()))
    }

    pub fn head(&self) -> Option<LispValue> {
        let list = self.get_list()?;
        if list.is_empty() {
            return Some(LispValue::nil());
        }
        Some(list[0].clone())
    }

    pub fn tail(&self) -> Option<LispValue> {
        let list = self.get_list()?;
        if list.is_empty() {
            return Some(LispValue::nil());
        }
        Some(LispValue::List(list[1..].into()))
    }

    pub fn get_list(&self) -> Option<&[LispValue]> {
        match self {
            LispValue::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn quoted(v: LispValue) -> LispValue {
        LispValue::List(vec![LispValue::Symbol("quote".into()), v].into())
    }

    pub fn nil() -> LispValue {
        LispValue::List(Arc::new([]))
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            LispValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_symbol(&self) -> Option<&String> {
        match self {
            LispValue::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            LispValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /*
    pub fn get_owned_symbol(self) -> Option<String> {
        match self {
            LispValue::Symbol(s) => Some(s),
            _ => None,
        }
    }
    */

    pub fn is_nil(&self) -> bool {
        match self {
            LispValue::List(list) => list.len() == 0,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            LispValue::List(_) => true,
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
            LispValue::Symbol(_sym) => true,
            _ => false,
        }
    }

    pub fn is_list_of_symbols(&self) -> bool {
        match self {
            LispValue::List(list) => list.iter().fold(true, |b, v| b && v.is_symbol()),
            _ => false,
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &Self) -> bool {
        use LispValue::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            (Symbol(a), Symbol(b)) => a == b,
            (List(a), List(b)) => a == b,

            (Func(_a), Func(_b)) => false, // NOTE: not reflexive, do not impl Eq without fixing this, if possible
            (UnsafeFunc(_a), UnsafeFunc(_b)) => false, // NOTE: not reflexive, do not impl Eq without fixing this
            (Lambda(_a), Lambda(_b)) => false, // NOTE: not reflexive, do not impl Eq without fixing this

            (Bool(_a), _) => false,
            (Integer(_a), _) => false,
            (Symbol(_a), _) => false,
            (List(_a), _) => false,
            (Func(_a), _) => false,
            (Lambda(_a), _) => false,
            (UnsafeFunc(_a), _) => false,
        }
    }
}

// Do not uncomment unless PartialEq is modified to be reflexive
//impl Eq for LispValue {}

impl core::fmt::Debug for LispValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl core::fmt::Display for LispValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use LispValue::*;
        match self {
            Bool(b) => write!(f, "[{}]", b),
            Integer(i) => write!(f, "{}", i),
            Symbol(s) => write!(f, "{}", s),
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
        }
    }
}

/*
fn display(val: &LispValue) {
    use LispValue::*;
    match &val {
        Nil => write!(f, "[nil]", s),
        Integer(i64) => write!(f, "{}", s),
        Number(f64) => write!(f, "{}", s),
        Symbol(String) => write!(f, "{}", s),
        List(Vec<LispValue>) => write!(f, "{}", s),
    }
}
*/

impl From<()> for LispValue {
    fn from(_v: ()) -> LispValue {
        LispValue::nil()
    }
}

impl From<bool> for LispValue {
    fn from(v: bool) -> LispValue {
        LispValue::Bool(v)
    }
}

impl From<i64> for LispValue {
    fn from(v: i64) -> LispValue {
        LispValue::Integer(v)
    }
}

impl From<&str> for LispValue {
    fn from(v: &str) -> LispValue {
        LispValue::Symbol(v.to_string())
    }
}
