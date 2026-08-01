/// Errors that can occur within the Ember framework.
use ember_http::status::StatusCode;

impl EmberError {
    /// Returns the HTTP status code associated with this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidConfiguration(_) => StatusCode::InternalServerError,
            Self::Network(_) => StatusCode::InternalServerError,

            Self::BadRequest => StatusCode::BadRequest,
            Self::Unauthorized => StatusCode::Unauthorized,
            Self::Forbidden => StatusCode::Forbidden,
            Self::NotFound => StatusCode::NotFound,
            Self::MethodNotAllowed => StatusCode::MethodNotAllowed,
            Self::InternalServerError => StatusCode::InternalServerError,

            Self::MissingPathParameter(_) => StatusCode::BadRequest,
            Self::MissingQueryParameter(_) => StatusCode::BadRequest,
            Self::MissingHeader(_) => StatusCode::BadRequest,
            Self::Json(_) => StatusCode::InternalServerError,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum EmberError {
    InvalidConfiguration(String),
    Network(String),
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    MissingPathParameter(String),
    MissingQueryParameter(String),
    MissingHeader(String),
    Json(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_not_found_error() {
        let error = EmberError::NotFound;

        assert_eq!(error, EmberError::NotFound);
    }

    #[test]
    fn creates_network_error() {
        let error = EmberError::Network("connection lost".into());

        assert!(matches!(error, EmberError::Network(_)));
    }

    #[test]
    fn creates_json_error() {
        let error = EmberError::Json("serialization failed".into());

        assert!(matches!(error, EmberError::Json(_)));
    }
}

#[cfg(test)]
mod status_tests {
    use super::*;

    #[test]
    fn maps_not_found() {
        assert_eq!(EmberError::NotFound.status_code(), StatusCode::NotFound);
    }

    #[test]
    fn maps_bad_request() {
        assert_eq!(EmberError::BadRequest.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn maps_network_error() {
        assert_eq!(
            EmberError::Network("oops".into()).status_code(),
            StatusCode::InternalServerError
        );
    }

    #[test]
    fn maps_missing_header() {
        assert_eq!(
            EmberError::MissingHeader("Host".into()).status_code(),
            StatusCode::BadRequest
        );
    }
}
