use std::collections::HashMap;


use crate::{headers::Header, method::Method, version::HttpVersion};

///Http request identification and creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Vec<Header>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl Request {
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(String::as_str)
    }

    pub fn query(&self, key: &str) -> Option<&str> {
        self.query.get(key).map(String::as_str)
    }
}
