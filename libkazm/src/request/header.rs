use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::net::TcpStream;
use std::str::FromStr;

use log::error;

use super::method::Method;

pub struct Header {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {:?}", self.method.to_string(), self.path, self.version, self.headers)
    }
}

impl Header {
    pub fn get(stream: &mut TcpStream) -> Result<Header, &str> {
        let mut lines_iter = std::io::BufReader::new(stream).lines();
        let line = &*lines_iter.next().unwrap_or_else(|| Ok("".to_string())).unwrap();

        let mut split = line.split_whitespace();
        let method_string = split.next().unwrap_or_default();
        let method = Method::from_str(method_string);
        let path = split.next();
        let version = split.next();
        if method.is_err() {
            error!("Unknown method {}", method_string);
            return Err("Invalid method");
        }
        if path.is_none() || version.is_none() {
            error!("No path or version");
            return Err("No path or version");
        }

        let mut header_values = HashMap::new();
        loop {
            match lines_iter.next() {
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
                    return Err("error while reading header");
                }
                _ => { break; }
            }
        }

        Ok(Header {
            method: method.unwrap(),
            path: path.unwrap().to_string(),
            version: version.unwrap().to_string(),
            headers: header_values,
        })
    }
}
