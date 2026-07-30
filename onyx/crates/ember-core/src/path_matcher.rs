pub struct PathMatcher;

impl PathMatcher {
    pub fn matches(route: &str, request: &str) -> bool {
        let route_segments: Vec<&str> = route
            .trim_matches('/')
            .split('/')
            .collect();

        let request_segments: Vec<&str> = request
            .trim_matches('/')
            .split('/')
            .collect();

        // Different number of segments -> no match
        if route_segments.len() != request_segments.len() {
            return false;
        }

        for (route_seg, request_seg) in
            route_segments.iter().zip(request_segments.iter())
        {
            if route_seg.starts_with(':') {
                continue;
            }

            if route_seg != request_seg {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_static_route() {
        assert!(PathMatcher::matches("/", "/"));
    }

    #[test]
    fn matches_dynamic_route() {
        assert!(PathMatcher::matches(
            "/users/:id",
            "/users/42",
        ));
    }

    #[test]
    fn matches_multiple_dynamic_segments() {
        assert!(PathMatcher::matches(
            "/users/:user/posts/:post",
            "/users/10/posts/55",
        ));
    }

    #[test]
    fn rejects_wrong_segment_count() {
        assert!(!PathMatcher::matches(
            "/users/:id",
            "/users",
        ));
    }

    #[test]
    fn rejects_wrong_static_segment() {
        assert!(!PathMatcher::matches(
            "/users/:id",
            "/posts/42",
        ));
    }

    #[test]
    fn matches_nested_static_route() {
        assert!(PathMatcher::matches(
            "/users/profile",
            "/users/profile",
        ));
    }
}