#[derive(strum_macros::EnumString, strum_macros::ToString)]
pub enum Method {
    #[strum(serialize = "GET")]
    Get,
    #[strum(serialize = "HEAD")]
    Head,
    #[strum(serialize = "POST")]
    Post,
    #[strum(serialize = "PUT")]
    Put,
    #[strum(serialize = "DELETE")]
    Delete,
    #[strum(serialize = "CONNECT")]
    Connect,
    #[strum(serialize = "OPTIONS")]
    Options,
    #[strum(serialize = "TRACE")]
    Trace,
    #[strum(serialize = "PATCH")]
    Patch,
}
