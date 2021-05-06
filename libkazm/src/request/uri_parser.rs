use std::collections::HashMap;

const PARAM_BEGIN_DELIMITER: &str = "?";
const PARAM_DELIMITER: &str = "&";
const PARAM_SPLITTER: &str = "=";

#[derive(Debug, PartialEq)]
pub enum UriParseError {
    MissingPath,
    DuplicateKey(String),
}

/// Contains the path and parameters of the uri of a request
pub struct UriValues {
    pub path: String,
    pub parameters: HashMap<String, String>,
}

impl UriValues {
    fn create_params_map(params_string: &str) -> Result<HashMap<String, String>, UriParseError> {
        let mut params_map = HashMap::new();

        for param in params_string.split(PARAM_DELIMITER) {
            let kvp: Vec<&str> = param.split(PARAM_SPLITTER).collect();

            if kvp.len() == 2 {
                if params_map.contains_key(kvp[0]) {
                    return Err(UriParseError::DuplicateKey(kvp[0].to_string()));
                }

                params_map.insert(kvp[0].to_owned(), kvp[1].to_owned());
            }
        }
        Ok(params_map)
    }

    /// Split the URL of an HTTP request into the path (e.g. `/foo/bar`) and a map of the params (e.g `test=1&bar=foo`)
    ///
    /// # Params
    ///
    /// - `path`: Uri to get the values from
    ///
    /// # Returns
    ///
    /// An instance of [UriValues] if parsing was successful, otherwise an [UriMatcherError] indicating what went wrong.
    pub fn from(uri: &str) -> Result<UriValues, UriParseError> {
        let path_end_index = uri.find(PARAM_BEGIN_DELIMITER).unwrap_or_else(|| uri.len());

        if path_end_index == 0 {
            return Err(UriParseError::MissingPath);
        }

        let (path, params_string) = uri.split_at(path_end_index);

        match UriValues::create_params_map(params_string.strip_prefix(PARAM_BEGIN_DELIMITER).unwrap_or(params_string)) {
            Ok(params) => Ok(UriValues { path: path.to_string(), parameters: params }),
            Err(e) => Err(e)
        }
    }
}

