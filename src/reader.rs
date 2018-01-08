use std::io;
use std::io::prelude::*;

use sexp::Sexp;

pub struct Reader<R: Read>(io::BufReader<R>);

impl <R: Read> Reader<R> {
    pub fn new(r: R) -> Reader<R> {
        Reader(io::BufReader::new(r))
    }

    pub fn read(self) -> io::Result<Sexp> {
        let mut it = self.0.bytes().map(|x| x.unwrap() as char);
        Self::parse(&mut it)
    }

    pub fn parse<T: Iterator<Item = char>>(chars: &mut T) -> io::Result<Sexp> {
        let mut expr = vec![];
        let mut token = String::new();
        let mut root = true;
        let mut indq = false;
        let mut insq = false;
        loop {
            match chars.next() {
                Some('\\') if insq || indq  => {
                    match chars.next() {
                        Some(c) if c == '\'' || c == '"' => token.push(c),
                        Some('n') => token.push('\n'),
                        Some('t') => token.push('\t'),
                        Some('r') => token.push('\r'),
                        _ => ()
                    }
                },
                Some('\'') => {
                    if indq {
                        token.push('\'');
                    } else if insq {
                        expr.push(Sexp::from(token));
                        token = String::new();
                        insq = false;
                    } else {
                        insq = true;
                    }
                },
                Some('"') => {
                    if insq {
                        token.push('"');
                    } else if indq {
                        expr.push(Sexp::from(token));
                        token = String::new();
                        indq = false;
                    } else {
                        indq = true;
                    }
                },
                Some(c) if indq || insq => {
                    token.push(c)
                },
                Some('(') => {
                    if root {
                        root = false;
                    } else {
                        expr.push(Self::parse(chars)?)
                    }
                },
                Some(')') => {
                    if token.len() > 0 {
                        expr.push(Sexp::from(token));
                    }

                    return Ok(Sexp::List(expr));
                },
                Some(c) => if c.is_whitespace() {
                    if token.len() > 0 {
                        expr.push(Sexp::from(token));
                        token = String::new();
                    }
                } else {
                    token.push(c);
                },
                None => return Err(io::Error::from(io::ErrorKind::InvalidInput))
            }
        }
    }
}

pub fn from_string<S: AsRef<str>>(s: &S) -> io::Result<Sexp> {
    let rd = Reader::new(io::Cursor::new(s.as_ref()));
    rd.read()
}

#[test]
fn test_roundtrip(){
    let s = "(a b c (1 2 3) (x) 't e s t')";
    let res = from_string(&s).unwrap();
    assert_eq!(::to_string(&res).unwrap(), s);
}
