use std::io::Write;

use super::status_code::StatusCode;

pub fn write_empty_response(stream: &mut dyn Write, code: StatusCode) -> std::io::Result<()> {
    write_response(stream, code, "")
}

pub fn write_response(stream: &mut dyn Write, code: StatusCode, message: &str) -> std::io::Result<()> {
    write!(stream, "HTTP/1.1 {}\r\n\r\n{}", StatusCode::to_response(code), message)
}