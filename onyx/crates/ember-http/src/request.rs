use crate::{
    method::Method,
    version::HttpVersion,
    headers::Header,
};

///Http request identification and creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Vec<Header>,
}

