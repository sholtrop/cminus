use std::error::Error;
use std::fmt;

pub struct SyntaxBuilderError(pub String);

impl SyntaxBuilderError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

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

impl From<&str> for SyntaxBuilderError {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<String> for SyntaxBuilderError {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl Error for SyntaxBuilderError {}

pub struct SyntaxBuilderWarning(pub String);

impl fmt::Display for SyntaxBuilderWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for SyntaxBuilderWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SyntaxBuilderWarning as fmt::Display>::fmt(self, f)
    }
}

impl From<&str> for SyntaxBuilderWarning {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<String> for SyntaxBuilderWarning {
    fn from(input: String) -> Self {
        Self(input)
    }
}
