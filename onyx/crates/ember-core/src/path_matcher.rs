pub struct PathMatcher;

impl PathMatcher {
    pub fn matches(route: &str, request: &str) -> bool {
        route == request
    }
}