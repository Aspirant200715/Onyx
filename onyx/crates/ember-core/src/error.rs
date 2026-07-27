/// Errors that can occur within the Ember framework.
#[derive(Debug)]
pub enum EmberError {
    /// An invalid server configuration.
    InvalidConfiguration(String),

    /// A networking operation failed.
    Network(String),
}