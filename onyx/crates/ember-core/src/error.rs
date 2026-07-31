/// Errors that can occur within the Ember framework.
#[derive(Debug)]
pub enum EmberError {
    InvalidConfiguration(String),
    Network(String),
}