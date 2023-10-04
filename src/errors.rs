use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VibinRemoteError {
    AppConfigError(String),
    InvalidKeyName { key_name: String },
}

impl Error for VibinRemoteError {}

impl fmt::Display for VibinRemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VibinRemoteError::AppConfigError(message) => write!(f, "{message}"),
            VibinRemoteError::InvalidKeyName { key_name } => write!(f, "Provided key name was invalid: {key_name}"),
        }
    }
}