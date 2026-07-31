pub struct PathMatcher;
use std::collections::HashMap;



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


    pub fn extract_params(route: &str, request: &str) -> HashMap<String, String> {
    let route_segments: Vec<&str> = route
        .trim_matches('/')
        .split('/')
        .collect();

    let request_segments: Vec<&str> = request
        .trim_matches('/')
        .split('/')
        .collect();

    let mut params = HashMap::new();

    for (route_seg, request_seg) in
        route_segments.iter().zip(request_segments.iter())
    {
        if let Some(name) = route_seg.strip_prefix(':') {
            params.insert(
                name.to_string(),
                request_seg.to_string(),
            );
        }
    }

    params
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

#[test]
fn extracts_single_parameter() {
    let params = PathMatcher::extract_params(
        "/users/:id",
        "/users/42",
    );

    assert_eq!(
        params.get("id"),
        Some(&"42".to_string())
    );
}

#[test]
fn extracts_multiple_parameters() {
    let params = PathMatcher::extract_params(
        "/users/:user/posts/:post",
        "/users/7/posts/99",
    );

    assert_eq!(
        params.get("user"),
        Some(&"7".to_string())
    );

    assert_eq!(
        params.get("post"),
        Some(&"99".to_string())
    );
}