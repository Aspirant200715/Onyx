use ember_http::request::Request;

/// Types that can be extracted from an HTTP request.
pub trait FromRequest {
    type Output;
    type Error;

    fn from_request(self, request: &Request) -> Result<Self::Output, Self::Error>;
}
