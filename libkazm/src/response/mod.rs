pub mod status_code;
pub mod response_writer;


#[cfg(test)]
mod tests {
    use crate::response::response_writer::{write_empty_response, write_response};
    use crate::response::status_code::StatusCode;

    #[test]
    fn write_empty_response_writes_correct_status_code() {
        let mut response = Vec::new();

        let result = write_empty_response(&mut response, StatusCode::Ok);

        assert_eq!(result.unwrap(), ());

        assert_eq!(String::from_utf8(response).unwrap(), String::from("HTTP/1.1 200 OK\r\n\r\n"));
    }

    #[test]
    fn write_response_writes_correct_status_code_and_message() {
        let mut response = Vec::new();

        let result = write_response(&mut response, StatusCode::Ok, "FooBar");

        assert_eq!(result.unwrap(), ());

        assert_eq!(String::from_utf8(response).unwrap(), String::from("HTTP/1.1 200 OK\r\n\r\nFooBar"));
    }
}