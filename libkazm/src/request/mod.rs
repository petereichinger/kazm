pub mod header;
pub mod method;
pub mod uri_parser;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::request::header::HeaderError;
    use crate::request::uri_parser::{UriParseError, UriValues};

    use super::*;

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
        assert_eq!(header.uri, path);
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

        let mut hash_map = HashMap::new();

        hash_map.insert(String::from("Foo"), String::from("Bar"));
        hash_map.insert(String::from("Baz"), String::from("Blubb"));

        assert_eq!(header.method, method::Method::Get);
        assert_eq!(header.uri, path);
        assert_eq!(header.version, version);
        assert_eq!(header.headers, hash_map);
    }


    #[test]
    fn uri_parser_does_not_parse_empty_uri_correctly() {
        let values = UriValues::from("").err();

        assert_eq!(values, Some(UriParseError::MissingPath));
    }

    #[test]
    fn uri_parser_basic_uri_successfully() {
        let values = UriValues::from("/Foo").unwrap();

        assert_eq!(values.path, String::from("/Foo"));
        assert_eq!(values.parameters, Default::default());
    }

    #[test]
    fn uri_parser_fails_when_path_is_missing() {
        let values = UriValues::from("?Foo=Bar").err();

        assert_eq!(values, Some(UriParseError::MissingPath));
    }

    #[test]
    fn uri_parser_fails_when_duplicate_paths_are_specified() {
        let values = UriValues::from("/Foo?Foo=Bar&Foo=Baz").err();

        assert_eq!(values, Some(UriParseError::DuplicateKey(String::from("Foo"))));
    }

    #[test]
    fn uri_parser_successfully() {
        let UriValues { path: x, parameters: y } = UriValues::from("/Foo?Foo=Bar&Bar=Baz").unwrap();

        assert_eq!(x, "/Foo");

        let mut hash_map = HashMap::new();
        hash_map.insert(String::from("Foo"), String::from("Bar"));
        hash_map.insert(String::from("Bar"), String::from("Baz"));

        assert_eq!(y, hash_map);
    }
}
