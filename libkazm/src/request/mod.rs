pub mod header;
pub mod method;
pub mod url_matcher;

#[cfg(test)]
mod tests {
    use crate::request::header::HeaderError;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn empty_request_results_in_error() {
        let request = String::from("");

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        assert_eq!(parse_result.err().unwrap(), HeaderError::Empty);
    }

    #[test]
    fn wrong_method_results_in_error() {
        let request = String::from("FOO /PATH VERSION");

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        assert_eq!(parse_result.err().unwrap(), HeaderError::InvalidMethod(String::from("FOO")));
    }

    #[test]
    fn missing_path_results_in_error() {
        let request = String::from("GET  VERSION");

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        assert_eq!(parse_result.err().unwrap(), HeaderError::MissingVersion);
    }

    #[test]
    fn missing_version_results_in_error() {
        let request = String::from("GET /PATH");

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        assert_eq!(parse_result.err().unwrap(), HeaderError::MissingVersion);
    }

    #[test]
    fn valid_prelude_is_successfully_parsed() {
        let path = String::from("/This/Is/A/Path.html");
        let version = String::from("HTTP666");

        let request = format!("GET {} {}", path, version);

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        let header = parse_result.unwrap();

        assert_eq!(header.method, method::Method::Get);
        assert_eq!(header.path, path);
        assert_eq!(header.version, version);
        assert_eq!(header.headers, Default::default())
    }

    #[test]
    fn header_params_are_successfully_parsed() {
        let path = String::from("/This/Is/A/Path.html");
        let version = String::from("HTTP666");

        let request = format!("GET {} {}\nFoo: Bar\nBaz: Blubb\n\n", path, version);

        let parse_result = header::Header::parse(&mut request.lines().map(|x| std::result::Result::Ok(String::from(x))));

        let header = parse_result.unwrap();

        let mut hash_map= HashMap::new();

        hash_map.insert(String::from("Foo"), String::from("Bar"));
        hash_map.insert(String::from("Baz"), String::from("Blubb"));

        assert_eq!(header.method, method::Method::Get);
        assert_eq!(header.path, path);
        assert_eq!(header.version, version);
        assert_eq!(header.headers, hash_map);
    }
}
