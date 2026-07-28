use crate::{headers::Header, method::Method, version::HttpVersion};

///Http request identification and creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Vec<Header>,
}
