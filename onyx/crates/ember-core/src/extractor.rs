use ember_http::request::Request;

/// Types that can be extracted from an HTTP request.
pub trait FromRequest {
    type Output;
    type Error;

    fn extract(self, request: &Request) -> Result<Self::Output, Self::Error>;
}
