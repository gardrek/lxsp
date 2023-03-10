use super::scan;
use super::value::Value;
use scan::{Token, TokenPayload};

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
        RemarkStart => read_remark(rest),
        RemarkEnd => Err(ParseError::Reason("unexpected `*)`".to_string())),
        LeftParen => read_seq(rest),
        RightParen => Err(ParseError::Reason("unexpected `)`".to_string())),
        Atom(s, is_number) => Ok((parse_atom(&s, is_number), rest)),
        Quote(inner) => match &**inner {
            LeftParen => {
                let (val, rest) = read_seq(rest)?;
                Ok((Value::quoted(val), rest))
            }
            Atom(s, is_number) => Ok((Value::quoted(parse_atom(&s, is_number)), rest)),
            /*..
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
    let mut previous_token = None;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(ParseError::Reason(format!(
            "could not find closing `)` near {:?}",
            previous_token.or(Some(0..0)).unwrap()
        )))?;
        previous_token = Some(next_token.span.clone());
        if let TokenPayload::RightParen = &next_token.payload {
            return Ok((Value::List(res.into()), rest)); // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn read_remark<'a>(tokens: &'a [Token]) -> Result<(Value, &'a [Token]), ParseError> {
    let mut res: Vec<Value> = vec![];
    let mut xs = tokens;
    let mut previous_token = None;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(ParseError::Reason(format!(
            "could not find closing `*)` near {:?}",
            previous_token.or(Some(0..0)).unwrap()
        )))?;
        previous_token = Some(next_token.span.clone());
        if let TokenPayload::RemarkEnd = &next_token.payload {
            return Ok((Value::nil(), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str, is_number: &bool) -> Value {
    match is_number {
        true => match token.parse::<i64>() {
            Ok(v) => Value::Integer(v),
            Err(e) => panic!("{:?}", e),
        },
        false => Value::Symbol(token.to_string().clone()),
    }
}
