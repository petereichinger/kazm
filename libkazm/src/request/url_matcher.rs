use std::collections::HashMap;
use std::fmt::Formatter;

const PARAM_BEGIN_DELIMITER: &str = "?";
const PARAM_DELIMITER: &str = "&";
const PARAM_SPLITTER: &str = "=";

pub enum UrlMatcherErrors {
    DuplicateKey(String)
}

impl std::fmt::Display for UrlMatcherErrors {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            UrlMatcherErrors::DuplicateKey(key) => write!(fmt, "Duplicate Key '{}'", key)
        }
    }
}


fn parse_params(params_string: &str) -> Result<HashMap<String, String>, UrlMatcherErrors> {
    let mut params_map = HashMap::new();

    for param in params_string.split(PARAM_DELIMITER) {
        let kvp: Vec<&str> = param.split(PARAM_SPLITTER).collect();

        if kvp.len() == 2 {
            if params_map.contains_key(kvp[0]) {
                return Err(UrlMatcherErrors::DuplicateKey(kvp[0].to_string()));
            }

            params_map.insert(kvp[0].to_owned(), kvp[1].to_owned());
        }
    }
    Ok(params_map)
}

/// Split the URL of an HTTP request into the path (e.g. `/foo/bar`) and a map of the params (e.g `test=1&bar=foo`)
///
///
pub fn parse_url(path: &str) -> Result<(String, HashMap<String, String>), UrlMatcherErrors> {
    let path_end_index = path.find(PARAM_BEGIN_DELIMITER).unwrap_or_else(|| path.len());

    let (path, params_string) = path.split_at(path_end_index);

    match parse_params(params_string.strip_prefix(PARAM_BEGIN_DELIMITER).unwrap_or(params_string)) {
        Ok(params) => Ok((path.to_string(), params)),
        Err(e) => Err(e)
    }
}
