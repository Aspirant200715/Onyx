use std::path::Path;

/// Returns the MIME type for a file path.
pub fn mime_type(path: impl AsRef<Path>) -> &'static str {
    match path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("txt") => "text/plain",

        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html() {
        assert_eq!(
            mime_type("index.html"),
            "text/html"
        );
    }

    #[test]
    fn css() {
        assert_eq!(
            mime_type("style.css"),
            "text/css"
        );
    }

    #[test]
    fn js() {
        assert_eq!(
            mime_type("app.js"),
            "application/javascript"
        );
    }

    #[test]
    fn png() {
        assert_eq!(
            mime_type("logo.png"),
            "image/png"
        );
    }

    #[test]
    fn unknown() {
        assert_eq!(
            mime_type("file.xyz"),
            "application/octet-stream"
        );
    }
}

