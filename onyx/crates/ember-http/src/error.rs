/// Errors that can occur while parsing an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    MissingRequestLine,
    InvalidMethod,
    InvalidPath,
    InvalidVersion,
    MalformedHeader,
}
