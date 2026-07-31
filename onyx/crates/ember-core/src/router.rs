use crate::route::Route;
use crate::path_matcher::PathMatcher;

use ember_http::method::Method;
use ember_http::request::Request;

use ember_http::{
    response::Response,
    status::StatusCode,
};

/// Stores all registered application routes.
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn get<F, R>(
        &mut self,
        path: impl Into<String>,
        handler: F,
    )
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: crate::responder::Responder + 'static,
    {
        self.routes.push(Route::new(
            Method::Get,
            path,
            Box::new(move |req| handler(req).into_response()),
        ));
    }

    pub fn post<F, R>(
        &mut self,
        path: impl Into<String>,
        handler: F,
    )
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: crate::responder::Responder + 'static,
    {
        self.routes.push(Route::new(
            Method::Post,
            path,
            Box::new(move |req| handler(req).into_response()),
        ));
    }

    pub fn put<F, R>(
        &mut self,
        path: impl Into<String>,
        handler: F,
    )
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: crate::responder::Responder + 'static,
    {
        self.routes.push(Route::new(
            Method::Put,
            path,
            Box::new(move |req| handler(req).into_response()),
        ));
    }

    pub fn delete<F, R>(
        &mut self,
        path: impl Into<String>,
        handler: F,     
    )
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: crate::responder::Responder + 'static,
    {
        self.routes.push(Route::new(
            Method::Delete,
            path,
            Box::new(move |req| handler(req).into_response()),
        ));
    }

    pub fn find(
        &self,
        request: &Request,
    ) -> Option<&Route> {
        self.routes.iter().find(|route| {
           route.method == request.method
           && PathMatcher::matches(
            &route.path,
            &request.path,
           )
        })
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}

impl Router {
    pub fn dispatch(
        &self,
        request: Request,
    ) -> Response {
        match self.find(&request) {
            Some(route) => {
                let mut request = request;

                request.params = PathMatcher::extract_params(
                    &route.path,
                    &request.path,
                );

                (route.handler)(request)
            },
            None => Response::new(StatusCode::NotFound)
                .header("Content-Type", "text/plain")
                .body("404 Not Found"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use ember_http::{
        request::Request,
        version::HttpVersion,
        headers::Header,
    };

    fn home(_: Request) -> &'static str {
        "Home"
    }

    #[test]
    fn registers_get_route() {
        let mut router = Router::new();

        router.get("/", home);

        assert_eq!(router.routes().len(), 1);

        let route = &router.routes()[0];

        assert_eq!(route.method, Method::Get);
        assert_eq!(route.path, "/");
  }

    #[test]
    fn finds_matching_route() {
        let mut router = Router::new();

        router.get("/", home);

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: std::collections::HashMap::new(),
        };

        let route = router.find(&request);

        assert!(route.is_some());
    }

    #[test]
    fn returns_none_for_unknown_route() {
        let mut router = Router::new();

        router.get("/", home);

        let request = Request {
            method: Method::Get,
            path: "/unknown".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: std::collections::HashMap::new(),
        };

        let route = router.find(&request);

        assert!(route.is_none());
    }
}

#[cfg(test)]
mod dispatch_tests {
    use super::*;

    use ember_http::{
        headers::Header,
        request::Request,
        status::StatusCode,
        version::HttpVersion,
    };

    fn home(_: Request) -> &'static str {
        "Home Page"
    }

    fn about(_: Request) -> String {
        "About Onyx".to_string()
    }

    #[test]
    fn dispatches_matching_handler() {
        let mut router = Router::new();

        router.get("/", home);

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: std::collections::HashMap::new(),
        };

        let response = router.dispatch(request);

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, "Home Page");
    }

    #[test]
    fn dispatches_string_handler() {
        let mut router = Router::new();

        router.get("/about", about);

        let request = Request {
            method: Method::Get,
            path: "/about".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: std::collections::HashMap::new(),
        };

        let response = router.dispatch(request);

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, "About Onyx");
    }

    #[test]
    fn returns_404_when_not_found() {
        let router = Router::new();

        let request = Request {
            method: Method::Get,
            path: "/missing".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: std::collections::HashMap::new(),
        };

        let response = router.dispatch(request);

        assert_eq!(response.status, StatusCode::NotFound);
    }
}