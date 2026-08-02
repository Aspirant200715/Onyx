use ember_http::{
    request::Request,
    response::Response,
    status::StatusCode,
};

use super::{Middleware, Next};

/// Simple bearer-token authentication middleware.
pub struct Auth {
    token: String,
}

impl Auth {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl Middleware for Auth {
    fn handle(
        &self,
        request: Request,
        next: &Next,
    ) -> Response {
        let authorized = request
            .headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case("Authorization"))
            .map(|header| header.value == format!("Bearer {}", self.token))
            .unwrap_or(false);

        if !authorized {
            return Response::new(StatusCode::Unauthorized)
                .header("Content-Type", "text/plain")
                .body("Unauthorized");
        }

        next.run(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ember_http::{
        headers::Header,
        method::Method,
        request::Request,
        response::Response,
        status::StatusCode,
        version::HttpVersion,
    };

    #[test]
    fn rejects_missing_header() {
        let auth = Auth::new("onyx");

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::new(),
            params: Default::default(),
            query: Default::default(),
        };

        let next = Next::new(|_| {
            Response::new(StatusCode::Ok)
                .body("OK")
        });

        let response = auth.handle(request, &next);

        assert_eq!(response.status, StatusCode::Unauthorized);
    }
}