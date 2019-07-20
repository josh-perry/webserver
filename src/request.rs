use std::fmt;
use std::collections::HashMap;
use crate::verb;

#[derive(Debug)]
pub struct Request {
    pub verb: verb::Verb,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
