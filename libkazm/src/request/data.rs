use crate::request::method::Method;
use std::fmt::{Formatter, Display};

pub struct RequestData {
    pub method: Method,
    pub path: String,
    pub version: String,
}

impl Display for RequestData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "METHOD: {} PATH: {} VERSION: {}", self.method.to_string(), self.path, self.version)
    }
}