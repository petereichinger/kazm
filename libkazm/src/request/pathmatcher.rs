use std::collections::HashMap;

const PARAM_BEGIN_DELIMITER: &str = "?";
const PARAM_DELIMITER: &str = "&";
const PARAM_SPLITTER: &str = "=";


fn parse_params(params_string: &str) -> Result<HashMap<String, String>, String> {
    let mut params_map = HashMap::new();

    for param in params_string.split(PARAM_DELIMITER) {
        let kvp: Vec<&str> = param.split(PARAM_SPLITTER).collect();

        if kvp.len() == 2 {
            if params_map.contains_key(kvp[0]) {
                return Err(format!("Duplicate parameter {}", kvp[0]));
            }

            params_map.insert(kvp[0].to_owned(), kvp[1].to_owned());
        }
    }
    Ok(params_map)
}

pub fn parse_path(path: &str) -> Result<(String, HashMap<String, String>), String> {
    let path_end_index = path.find(PARAM_BEGIN_DELIMITER).unwrap_or_else(|| path.len());

    let (path, params_string) = path.split_at(path_end_index);

    match parse_params(params_string.strip_prefix(PARAM_BEGIN_DELIMITER).unwrap_or(params_string)) {
        Ok(params) => { Ok((path.to_string(), params)) }
        Err(e) => { Err(e) }
    }
}