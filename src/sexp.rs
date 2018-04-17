#[derive(Debug, Clone, PartialEq)]
pub enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

#[macro_export]
macro_rules! list {
    [$($x:expr),+] => {
        sexp::Sexp::List(vec![$(From::from($x)),+])
    }
}

#[macro_export]
macro_rules! atom {
    ($x:expr) => {
        From::from(x)
    }
}

impl Sexp {
    pub fn atom<S: AsRef<str>>(s: S) -> Sexp {
        Sexp::Atom(String::from(s.as_ref()))
    }

    pub fn list<X: AsRef<[Sexp]>>(x: X) -> Sexp {
        Sexp::List(x.as_ref().to_vec())
    }

    pub fn unit() -> Sexp {
        Sexp::List(Vec::new())
    }

    pub fn is_unit(&self) -> bool {
        match self {
            &Sexp::List(ref l) => l.len() == 0,
            _ => false,
        }
    }

    pub fn to_vec(&self) -> Vec<Sexp> {
        match self {
            x @ &Sexp::Atom(_) => vec![x.clone()],
            &Sexp::List(ref l) => l.clone(),
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            &Sexp::Atom(ref s) => Some(s.clone()),
            &Sexp::List(ref l) if l.len() == 1 => l[0].to_string(),
            _ => None,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self.to_string()?.parse::<i64>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self.to_string()?.parse::<f64>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}

impl From<Vec<Sexp>> for Sexp {
    fn from(v: Vec<Sexp>) -> Sexp {
        Sexp::List(v)
    }
}

impl<'a> From<&'a str> for Sexp {
    fn from(s: &'a str) -> Sexp {
        Sexp::atom(&s)
    }
}

impl From<String> for Sexp {
    fn from(s: String) -> Sexp {
        Sexp::Atom(s)
    }
}

impl From<i64> for Sexp {
    fn from(i: i64) -> Sexp {
        Sexp::Atom(format!("{}", i))
    }
}

impl From<f64> for Sexp {
    fn from(f: f64) -> Sexp {
        Sexp::Atom(format!("{}", f))
    }
}
