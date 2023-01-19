use super::lisp::Value;
use super::scan;
use scan::{Token, TokenPayload};
/*
pub struct Parser {
    list_stack: Vec<Vec<Value>>,
    current_list: Optin<Vec<Value>>,
    list: Vec<Value>,
}

impl Parser {
    fn parse_token(&mut self, token: Token) {
        match token {
            LeftParen => {
                let list = vec![];
                self.current_value = Some(list);
                // self.list_stack.push(Value::List(list));
            }
            RightParen => {
                let list = self.current_list.take();
                self.list_stack.push(list);
                self.current_list = Some();
            }
        }
    }
}
*/
#[derive(Debug)]
pub enum ParseError {
    Reason(String),
}

impl std::error::Error for ParseError {}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use ParseError::*;
        match &self {
            Reason(s) => write!(f, "{}", s),
        }
    }
}

pub fn parse<'a>(tokens: &'a [Token]) -> Result<(Value, &'a [Token]), ParseError> {
    use TokenPayload::*;
    let (token, rest) = tokens
        .split_first()
        .ok_or(ParseError::Reason("could not get token".to_string()))?;
    match &token.payload {
        LeftParen => read_seq(rest),
        RightParen => Err(ParseError::Reason("unexpected `)`".to_string())),
        Atom(s) => Ok((parse_atom(&s), rest)),
        Quote(inner) => match &**inner {
            LeftParen => {
                let (val, rest) = read_seq(rest)?;
                Ok((Value::quoted(val), rest))
            }
            Atom(s) => Ok((Value::quoted(parse_atom(&s)), rest)),
            /*
            Quote(q) => {
                let (token, _) = parse(&[q])?;
                Ok((Value::quoted(), rest))
            }
            */
            _ => return Err(ParseError::Reason("invalid quote syntax".to_string())),
        },
    }
}

fn read_seq<'a>(tokens: &'a [Token]) -> Result<(Value, &'a [Token]), ParseError> {
    let mut res: Vec<Value> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(ParseError::Reason("could not find closing `)`".to_string()))?;
        if let TokenPayload::RightParen = &next_token.payload {
            return Ok((Value::List(res.into()), rest)); // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> Value {
    match token.parse::<i64>() {
        Ok(v) => Value::Integer(v),
        Err(_) => Value::Symbol(token.to_string().clone()),
    }
}
