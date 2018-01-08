use std::io;
use std::io::prelude::*;

use sexp::Sexp;

pub struct Writer<W: io::Write>(io::BufWriter<W>);

impl <W: io::Write> Writer<W> {
    pub fn new(w: W) -> Writer<W> {
        Writer(io::BufWriter::new(w))
    }

    pub fn write(&mut self, expr: &Sexp) -> io::Result<()> {
        match expr {
            &Sexp::Atom(ref s) if s.contains(' ') || s.contains('(') || s.contains(')') => {
                let b = if s.contains('\'') {
                    b"\""
                } else {
                    b"'"
                };

                self.0.write_all(b)?;
                if s.contains('\'') && b == b"'" {
                    self.0.write_all(s.replace("'", r"\'").as_bytes())?;
                } else {
                    self.0.write_all(s.as_bytes())?;
                }
                self.0.write_all(b)
            },
            &Sexp::Atom(ref s) => self.0.write_all(s.as_str().as_bytes()),
            &Sexp::List(ref l) => {
                self.0.write_all(b"(")?;
                for (n, item) in l.iter().enumerate() {
                    if n > 0 {
                        self.0.write_all(b" ")?
                    }
                    self.write(item)?
                }
                self.0.write_all(b")")
            }
        }
    }

    pub fn into_inner(self) -> Option<W> {
        match self.0.into_inner() {
            Ok(x) => Some(x),
            Err(_) => None
        }
    }
}

pub fn to_string(expr: &Sexp) -> io::Result<String> {
    let mut dst = Vec::new();

    {
        let mut wr = Writer::new(dst);
        wr.write(expr)?;
        dst = wr.into_inner().unwrap();
    }


    unsafe {
        Ok(String::from_utf8_unchecked(dst))
    }
}

#[cfg(test)]
mod test {
    use sexp;

    #[test]
    fn test_write_simple(){
        let s = list![
            "a", "b", "c",
            list![1, 2, 3],
            "x y z'\""
        ];

        let dst = ::to_string(&s).unwrap();

        assert_eq!(dst, "(a b c (1 2 3) \"x y z'\"\")");
    }
}
