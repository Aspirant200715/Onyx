/// Stores application routes.
pub struct Router {
    routes: Vec<String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: impl Into<String>) {
        self.routes.push(path.into());
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn routes(&self) -> &[String] {
        &self.routes
    }
}