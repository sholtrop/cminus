use std::error::Error;
use std::fmt;

pub struct SyntaxBuilderError(String);

impl fmt::Display for SyntaxBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for SyntaxBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SyntaxBuilderError as fmt::Display>::fmt(self, f)
    }
}

impl Error for SyntaxBuilderError {}
