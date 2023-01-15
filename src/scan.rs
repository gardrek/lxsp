#[derive(Debug)]
pub enum TokenPayload {
    LeftParen,
    RightParen,
    Atom(String),
    Quote(Box<TokenPayload>),
}

#[derive(Debug)]
pub struct Token {
    pub payload: TokenPayload,
    pub span: core::ops::Range<usize>,
}

#[derive(Debug)]
pub struct ScanError {
    s: &'static str,
    pub span: core::ops::Range<usize>,
}

impl std::error::Error for ScanError {}

impl core::fmt::Display for ScanError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Scan error {} at {:?}", self.s, self.span)
    }
}

impl Token {
    fn new(payload: TokenPayload, span: core::ops::Range<usize>) -> Token {
        Token { payload, span }
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &self.payload)
    }
}

impl core::fmt::Display for TokenPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use TokenPayload::*;
        match &self {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Atom(string) => write!(f, "{:?}", &string),
            Quote(token) => write!(f, "'{}", &token),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner { source, cursor: 0 }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        use TokenPayload::*;

        let token = 'token: loop {
            let short_span = self.cursor..(self.cursor + 1);
            let ch = char_at_index(self.source, self.cursor)?;
            match ch {
                '(' => {
                    self.cursor += ch.len_utf8();
                    break 'token Some(Token::new(LeftParen, short_span));
                }
                ')' => {
                    self.cursor += ch.len_utf8();
                    break 'token Some(Token::new(RightParen, short_span));
                }
                '\'' => {
                    let first = self.cursor;
                    self.cursor += ch.len_utf8();
                    let next = self.scan_token()?;
                    // TODO: match on next to make sure it's not invalid
                    let payload = TokenPayload::Quote(Box::new(next.payload));
                    let token = Token::new(payload, first..self.cursor);
                    break 'token Some(token);
                }
                ch if ch.is_whitespace() => {
                    self.cursor += ch.len_utf8();
                    continue;
                }
                ch if ch.is_digit(36) => {
                    let first = self.cursor;
                    let mut ch = ch;
                    let mut atom = String::new();
                    while ch.is_digit(36) {
                        atom.push(ch);
                        self.cursor += ch.len_utf8();
                        match char_at_index(self.source, self.cursor) {
                            Some(ch_in) => {
                                ch = ch_in;
                                if ch.is_whitespace() || ch == '(' || ch == ')' {
                                    let token = Token::new(Atom(atom), first..self.cursor);
                                    break 'token Some(token);
                                }
                            }
                            None => {
                                let token = Token::new(Atom(atom), first..self.cursor);
                                break 'token Some(token);
                            }
                        }
                    }
                }
                _ => break 'token None,
            }
            match char_at_index(self.source, self.cursor) {
                Some(ch) => self.cursor += ch.len_utf8(),
                None => return None,
            }
        };
        return token;
    }
}

fn char_at_index(string: &str, index: usize) -> Option<char> {
    string[index..].chars().next()
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_math_test() {
        let src = "(add
(add 10 5 2)
(sub 10 7)
(sub 10 (add 5 2))
(sub(add 1 1)1)
(    add    5       6       )

)";

        let scanner = Scanner::new(src);

        let tokens = scanner.collect::<Vec<_>>();

        assert_eq!(tokens.len(), 37);
    }
}
