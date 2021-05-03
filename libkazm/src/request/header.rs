use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use log::error;

use super::method::Method;

/// Struct containing all errors that can occur while parsing the header in [Header::parse]
#[derive(strum_macros::EnumString, strum_macros::ToString, PartialEq, Debug)]
pub enum HeaderError {
    Empty,
    InvalidMethod(String),
    MissingPath,
    MissingVersion,
    ParamError,
}

/// Contains all header information of a HTTP request
pub struct Header {
    pub method: Method,
    pub url: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {:?}", self.method.to_string(), self.url, self.version, self.headers)
    }
}

impl Header {
    /// Parse the header from a specified [Iterator]
    ///
    /// Parsing includes the prelude of the request (e.g. GET /Bar/Baz HTTP1.1)
    /// And all header fields
    ///
    /// # Returns
    ///
    /// An instance of [Header] if parsing was successful, or HeaderError indicating what went wrong otherwise
    pub fn parse(lines: &mut dyn Iterator<Item=std::io::Result<String>>) -> Result<Header, HeaderError> {
        let line = &*lines.next().unwrap_or_else(|| Ok("".to_string())).unwrap();
        if line.is_empty(){
            error!("Request contains no text");
            return Err(HeaderError::Empty);
        }
        let mut split = line.split_whitespace();
        let method_string = split.next().unwrap_or_default();
        let method = Method::from_str(method_string);
        let url = split.next();
        let version = split.next();
        if method.is_err() {
            error!("Unknown method {}", method_string);
            return Err(HeaderError::InvalidMethod(String::from(method_string)));
        }
        if url.is_none() {
            error!("No path");
            return Err(HeaderError::MissingPath);
        }

        if version.is_none() {
            error!("No request version");
            return Err(HeaderError::MissingVersion);
        }

        let mut header_values = HashMap::new();
        loop {
            match lines.next() {
                Some(Ok(hdr)) if !hdr.is_empty() => {
                    let mut header_split = hdr.split(": ");
                    let key = header_split.next().unwrap_or_default();
                    let value = header_split.next().unwrap_or_default();

                    if !key.is_empty() {
                        header_values.insert(key.to_string(), value.to_string());
                    }
                }
                Some(Err(e)) => {
                    error!("Error while reading header: {}", e);
                    return Err(HeaderError::ParamError);
                }
                _ => { break; }
            }
        }

        Ok(Header {
            method: method.unwrap(),
            url: url.unwrap().to_string(),
            version: version.unwrap().to_string(),
            headers: header_values,
        })
    }
}
