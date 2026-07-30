/// Represents a type that can become an HTTP response.
pub trait Responder {
    fn respond(&self) -> String;
}