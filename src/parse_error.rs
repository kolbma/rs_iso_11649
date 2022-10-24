//!

/// The `ParseError` enum is a collection of all the possible
/// reasons parsing fail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// invalid character not parseable
    InvalidCharacter(String),
    /// checksum has invalid format
    InvalidChecksum(String),
    /// invalid format not parseable
    InvalidFormat(String),
    /// identifier is not RF
    InvalidIdentifier(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::InvalidCharacter(m) => write!(f, "invalid character not parseable [{}]", m),
            ParseError::InvalidChecksum(m) => write!(f, "checksum has invalid format [{}]", m),
            ParseError::InvalidFormat(m) => write!(f, "invalid format not parseable [{}]", m),
            ParseError::InvalidIdentifier(m) => write!(f, "identifier is not RF [{}]", m),
        }
    }
}

impl std::error::Error for ParseError {}
