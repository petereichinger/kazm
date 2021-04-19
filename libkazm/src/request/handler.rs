use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{BufRead};
use crate::request::data;
use crate::request::method;
use std::str::FromStr;


fn get_headers(line: &str) -> Option<data::RequestData> {
    let mut split = line.split(" ");
    let method_string = split.next().unwrap_or_default();
    let method = method::Method::from_str(method_string);
    let path = split.next();
    let version = split.next();
    if method.is_err() {
        eprintln!("Unknown method {}", method_string);
        return None;
    }
    if path.is_none() || version.is_none() {
        return None;
    }
    Some(data::RequestData {
        method: method.unwrap(),
        path: path.unwrap().to_string(),
        version: path.unwrap().to_string(),
    })
}

pub fn handle(stream: TcpStream) {
    let mut lines_iter = std::io::BufReader::new(stream).lines();

    let first_header = get_headers(&*lines_iter.next().unwrap_or(Ok("".to_string())).unwrap_or_default());

    if first_header.is_none() {
        return;
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
            _ => { break; }
        }
    }

    println!("{}\n\nHEADERS\n{:?}\n", first_header.unwrap(), header_values);
}
