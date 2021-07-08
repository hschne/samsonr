use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SamsonrError {
    pub message: String,
}

impl fmt::Display for SamsonrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error ocurred")
    }
}

impl Error for SamsonrError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl From<reqwest::Error> for SamsonrError {
    fn from(error: reqwest::Error) -> Self {
        SamsonrError { message: format!("Reqwest error, error={}", error) }
    }
}
