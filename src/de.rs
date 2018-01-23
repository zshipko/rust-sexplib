use std::fmt;

use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, Error};

use sexp::Sexp;

struct SexpVisitor;

impl <'de> Visitor<'de> for SexpVisitor {
    type Value = Sexp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid s-expression")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Sexp::Atom(String::from(v)))
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(Sexp::Atom(String::from(v)))
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Sexp::Atom(v))
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
    {
        let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(elem) = access.next_element()? {
            seq.push(elem)
        }

        Ok(Sexp::List(seq))
    }
}


impl <'de> Deserialize<'de> for Sexp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(SexpVisitor)
    }
}
