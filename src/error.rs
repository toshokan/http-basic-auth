#[derive(Debug, PartialEq)]
pub enum ParseError {
    Scheme,
    Decode,
    Format
}

impl From<base64::DecodeError> for ParseError {
    fn from(_: base64::DecodeError) -> Self {
	ParseError::Decode
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(_: std::str::Utf8Error) -> Self {
	ParseError::Decode
    }
}
