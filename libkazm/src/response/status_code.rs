#[derive(strum_macros::EnumString, strum_macros::ToString, Debug, Copy, Clone)]
pub enum StatusCode {
    #[strum(serialize="OK")]
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn to_response(status_code: StatusCode) -> String {
        format!("{} {}", status_code as u16, status_code.to_string())
    }
}
