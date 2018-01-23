use serde::ser::{Serialize, Serializer, SerializeSeq};

use sexp::Sexp;

impl Serialize for Sexp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            &Sexp::Atom(ref s) => serializer.serialize_str(s),
            &Sexp::List(ref l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for elem in l {
                    seq.serialize_element(elem)?;
                }
                seq.end()
            }
        }
    }
}
