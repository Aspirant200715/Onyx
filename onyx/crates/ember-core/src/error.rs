/// Errors that can occur within the Ember framework.
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
