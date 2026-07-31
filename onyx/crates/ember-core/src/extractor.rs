use ember_http::request::Request;

/// Types that can be extracted from an HTTP request.
pub trait FromRequest: Sized {
    type Error;

    fn from_request(request: &Request) -> Result<Self, Self::Error>;
}
