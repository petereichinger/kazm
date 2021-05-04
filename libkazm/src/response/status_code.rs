#[derive(strum_macros::EnumString, strum_macros::ToString, Debug, Copy, Clone)]
pub enum StatusCode {
    #[strum(serialize="OK")]
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn to_response(self) -> String {
        format!("{} {}", self as u16, self.to_string())
    }
}
