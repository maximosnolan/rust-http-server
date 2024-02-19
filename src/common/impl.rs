use std::error::Error;
use std::fmt;

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP Error");
    }
}