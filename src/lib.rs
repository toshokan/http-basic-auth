mod error;

use error::ParseError;

#[derive(Debug, PartialEq)]
pub struct Credential {
    user_id: String,
    password: String
}

pub fn decode(s: &str) -> Result<Credential, ParseError> {
    match s {
	s if s.starts_with("Basic") => {
	    let decoded = base64::decode(&s[6..])?;
	    let decoded = std::str::from_utf8(&decoded)?;
	    let parts: Vec<&str> = decoded.splitn(2, ":").collect();
	    match &parts.as_slice() {
		[user, pass] => Ok(Credential { user_id: user.to_string(), password: pass.to_string() }),
		_ => Err(ParseError::Format)
	    }
	}
	_ => Err(ParseError::Scheme)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn encode(c: &Credential) -> String {
	let unencoded = format!("{}:{}", c.user_id, c.password);
	let encoded = base64::encode(unencoded);
	format!("Basic {}", encoded)
    }

    fn encode_str(s: &str) -> String {
	let encoded = base64::encode(s);
	format!("Basic {}", encoded)
    }

    #[test]
    fn identify_scheme() {
	let c = Credential {
	    user_id: "Aladdin".to_string(),
	    password: "open sesame".to_string()
	};
	let s = encode(&c);
	let c2 = decode(&s);
	assert_eq!(Ok(c), c2);
    }

    #[test]
    fn no_colon() {
	let s = encode_str("john");
	let c = decode(&s);
	assert_eq!(Err(ParseError::Format), c);
    }

    #[test]
    fn allow_empty_password() {
	let s = encode_str("john:");
	let c = decode(&s);
	assert_ne!(Err(ParseError::Format), c);
    }

    #[test]
    fn bad_base64() {
	let s = "Basic abcdefg";
	let c = decode(&s);
	assert_eq!(Err(ParseError::Decode), c);
    }

    #[test]
    fn bad_scheme() {
	let c = Credential {
	    user_id: "john".to_string(),
	    password: "hunter2".to_string(),
	};
	let unencoded = format!("{}:{}", c.user_id, c.password);
	let encoded = base64::encode(unencoded);
	let formatted = format!("Bearer {}", encoded);
	let c = decode(&formatted);
	assert_eq!(Err(ParseError::Scheme), c);
    }
}
