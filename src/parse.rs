use super::lisp::LispValue;
use super::scan;
use scan::{Token, TokenPayload};
/*
pub struct Parser {
    list_stack: Vec<Vec<LispValue>>,
    current_list: Optin<Vec<LispValue>>,
    list: Vec<LispValue>,
}

impl Parser {
    fn parse_token(&mut self, token: Token) {
        match token {
            LeftParen => {
                let list = vec![];
                self.current_value = Some(list);
                // self.list_stack.push(LispValue::List(list));
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

pub fn parse<'a>(tokens: &'a [Token]) -> Result<(LispValue, &'a [Token]), ParseError> {
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
                Ok((LispValue::quoted(val), rest))
            }
            Atom(s) => Ok((LispValue::quoted(parse_atom(&s)), rest)),
            /*
            Quote(q) => {
                let (token, _) = parse(&[q])?;
                Ok((LispValue::quoted(), rest))
            }
            */
            _ => return Err(ParseError::Reason("invalid quote syntax".to_string())),
        },
    }
}

fn read_seq<'a>(tokens: &'a [Token]) -> Result<(LispValue, &'a [Token]), ParseError> {
    let mut res: Vec<LispValue> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(ParseError::Reason("could not find closing `)`".to_string()))?;
        if let TokenPayload::RightParen = &next_token.payload {
            return Ok((LispValue::List(res.into()), rest)); // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> LispValue {
    match token.parse::<i64>() {
        Ok(v) => LispValue::Integer(v),
        Err(_) => LispValue::Symbol(token.to_string().clone()),
    }
}
