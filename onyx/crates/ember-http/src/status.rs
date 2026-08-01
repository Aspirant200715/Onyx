/// Standard HTTP status codes supported by Onyx
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,
    Created,
    NoContent,

    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,

    InternalServerError,
}

impl StatusCode {
    pub fn as_u16(self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::NoContent => 204,

            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,

            StatusCode::InternalServerError => 500,
        }
    }

    pub fn reason_phrase(self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::NoContent => "No Content",

            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",

            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_status_code() {
        assert_eq!(StatusCode::Ok.as_u16(), 200);
    }

    #[test]
    fn not_found_reason_phrase() {
        assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
    }

    #[test]
    fn internal_server_error_status_code() {
        assert_eq!(StatusCode::InternalServerError.as_u16(), 500);
    }
}
