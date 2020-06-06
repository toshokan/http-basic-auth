pub enum ParseError {
    Decode,
    Format
}

impl From<base64::DecodeError> for ParseError {
    fn from(_: base64::DecodeError) -> Self {
	ParseError::Decode
    }
}
