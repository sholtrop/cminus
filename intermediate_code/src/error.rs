use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct ICodeError(pub String);

impl ICodeError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl fmt::Display for ICodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for ICodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <ICodeError as fmt::Display>::fmt(self, f)
    }
}

impl From<&str> for ICodeError {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<String> for ICodeError {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl Error for ICodeError {}
