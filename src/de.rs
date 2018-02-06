use std::{io, fmt};

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};

use sexp::Sexp;
use error;
use reader::Reader;
use writer::Writer;

struct SexpVisitor;

impl<'de> Visitor<'de> for SexpVisitor {
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
    where
        A: SeqAccess<'de>,
    {
        let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(elem) = access.next_element()? {
            seq.push(elem)
        }

        Ok(Sexp::List(seq))
    }
}

impl<'de> Deserialize<'de> for Sexp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SexpVisitor)
    }
}

macro_rules! deserialize_number {
    ($name:ident, $visit:ident, $t:ty) => {
        fn $name<V>(self, visitor: V) -> error::Result<V::Value>
        where
            V: Visitor<'de>
        {
            let v = self.read()?;
            match v {
                Sexp::Atom(s) => match s.parse::<$t>() {
                    Ok(x) => visitor.$visit(x),
                    Err(_) => Err(error::Error::InvalidType)
                },
                _ => Err(error::Error::InvalidType)
            }
        }
    }
}

impl <'de, R: io::Read> Deserializer<'de> for Reader<R> {
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::Atom(s) => visitor.visit_string(s),
            Sexp::List(l) => visitor.visit_seq(l)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::Atom(s) | Sexp::Atom(s) | Sexp::Atom(s) if s == "true" || s == "TRUE" || s == "1" => visitor.visit_bool(true),
            Sexp::Atom(s) | Sexp::Atom(s) | Sexp::Atom(s) if s == "false" || s == "FALSE" || s == "0" => visitor.visit_bool(false),
            _ => Err(error::Error::InvalidType)
        }
    }

    deserialize_number!(deserialize_i8, visit_i8, i8);
    deserialize_number!(deserialize_i16, visit_i16, i16);
    deserialize_number!(deserialize_i32, visit_i32, i32);
    deserialize_number!(deserialize_i64, visit_i64, i64);

    deserialize_number!(deserialize_u8, visit_u8, u8);
    deserialize_number!(deserialize_u16, visit_u16, u16);
    deserialize_number!(deserialize_u32, visit_u32, u32);
    deserialize_number!(deserialize_u64, visit_u64, u64);

    deserialize_number!(deserialize_f32, visit_f32, f32);
    deserialize_number!(deserialize_f64, visit_f64, f64);
    deserialize_number!(deserialize_char, visit_char, char);

    fn deserialize_str<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::Atom(s) => visitor.visit_borrowed_str(s.as_str()),
            _ => Err(error::Error::InvalidType)
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::Atom(s) => visitor.visit_string(s),
            _ => Err(error::Error::InvalidType)
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::Atom(s) => visitor.visit_borrowed_bytes(s.as_bytes()),
            _ => Err(error::Error::InvalidType)
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::List(l) if l.len() == 1 => {
                let mut dst = Vec::new();
                let wr = Writer::new(dst);
                wr.write(&l[0]);
                let mut dst = match wr.into_inner() {
                    Some(inner) => inner,
                    None => return Err(error::Error::InvalidType)
                };
                let rd = Reader::new(dst.as_slice());
                visitor.visit_some(rd)
            },
            x => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let v = self.read()?;
        match v {
            Sexp::List(l) if l.len() == 0 => visitor.visit_unit(),
            _ => Err(error::Error::InvalidType)
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }


    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_seq<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_map<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        Err(error::Error::NotImplemented)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>
    {
        let _r = self.read()?;
        visitor.visit_unit()
    }


}
