use ember_http::request::Request;

pub trait FromRequest {
    type Output;
    type Error;

    fn extract(self, request: &Request) -> Result<Self::Output, Self::Error>;
}
