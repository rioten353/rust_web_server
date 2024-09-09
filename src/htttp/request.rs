use super::method::Method;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_string: Option<String>,
}
