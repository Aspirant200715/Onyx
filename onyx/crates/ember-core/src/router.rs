use crate::route::{Handler, Route};

use ember_http::method::Method;

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

    pub fn get(
        &mut self,
        path: impl Into<String>,
        handler: Handler,
    ) {
        self.routes.push(Route::new(
            Method::Get,
            path,
            handler,
        ));
    }

    pub fn post(
        &mut self,
        path: impl Into<String>,
        handler: Handler,
    ) {
        self.routes.push(Route::new(
            Method::Post,
            path,
            handler,
        ));
    }

    pub fn put(
        &mut self,
        path: impl Into<String>,
        handler: Handler,
    ) {
        self.routes.push(Route::new(
            Method::Put,
            path,
            handler,
        ));
    }

    pub fn delete(
        &mut self,
        path: impl Into<String>,
        handler: Handler,
    ) {
        self.routes.push(Route::new(
            Method::Delete,
            path,
            handler,
        ));
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ember_http::{
        request::Request,
        response::Response,
        status::StatusCode,
    };

    fn home(_: Request) -> Response {
        Response::new(StatusCode::Ok)
            .body("Home")
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
}