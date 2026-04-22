use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`HttpStatusCode`].
pub type HttpStatusCodeInput = u16;

/// Output type for [`HttpStatusCode`].

/// A validated HTTP status code in the range `100..=599`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::HttpStatusCode;
/// use arvo::traits::ValueObject;
///
/// let code = HttpStatusCode::new(200)?;
/// assert_eq!(*code.value(), 200);
/// assert!(code.is_success());
///
/// assert!(HttpStatusCode::new(600).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub struct HttpStatusCode(u16);

impl ValueObject for HttpStatusCode {
    type Input = HttpStatusCodeInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if !(100..=599).contains(&value) {
            return Err(ValidationError::invalid(
                "HttpStatusCode",
                &value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for HttpStatusCode {
    type Primitive = u16;
    fn value(&self) -> &u16 {
        &self.0
    }
}

impl HttpStatusCode {
    /// Returns `true` for 1xx informational codes.
    pub fn is_informational(&self) -> bool {
        (100..=199).contains(&self.0)
    }

    /// Returns `true` for 2xx success codes.
    pub fn is_success(&self) -> bool {
        (200..=299).contains(&self.0)
    }

    /// Returns `true` for 3xx redirection codes.
    pub fn is_redirection(&self) -> bool {
        (300..=399).contains(&self.0)
    }

    /// Returns `true` for 4xx client error codes.
    pub fn is_client_error(&self) -> bool {
        (400..=499).contains(&self.0)
    }

    /// Returns `true` for 5xx server error codes.
    pub fn is_server_error(&self) -> bool {
        (500..=599).contains(&self.0)
    }
}

impl TryFrom<u16> for HttpStatusCode {
    type Error = ValidationError;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<HttpStatusCode> for u16 {
    fn from(v: HttpStatusCode) -> u16 {
        v.0
    }
}
impl TryFrom<&str> for HttpStatusCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<u16>().map_err(|_| ValidationError::invalid("HttpStatusCode", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_200() {
        let code = HttpStatusCode::new(200).unwrap();
        assert_eq!(*code.value(), 200);
    }

    #[test]
    fn accepts_boundaries() {
        assert!(HttpStatusCode::new(100).is_ok());
        assert!(HttpStatusCode::new(599).is_ok());
    }

    #[test]
    fn rejects_below_100() {
        assert!(HttpStatusCode::new(99).is_err());
    }

    #[test]
    fn rejects_600_and_above() {
        assert!(HttpStatusCode::new(600).is_err());
    }

    #[test]
    fn category_helpers() {
        assert!(HttpStatusCode::new(100).unwrap().is_informational());
        assert!(HttpStatusCode::new(200).unwrap().is_success());
        assert!(HttpStatusCode::new(301).unwrap().is_redirection());
        assert!(HttpStatusCode::new(404).unwrap().is_client_error());
        assert!(HttpStatusCode::new(500).unwrap().is_server_error());
    }

    #[test]
    fn display() {
        let code = HttpStatusCode::new(404).unwrap();
        assert_eq!(code.to_string(), "404");
    }

    #[test]
    fn into_inner_roundtrip() {
        let code = HttpStatusCode::new(201).unwrap();
        assert_eq!(code.into_inner(), 201);
    }

    #[test]
    fn try_from_parses_valid() {
        let c = HttpStatusCode::try_from("200").unwrap();
        assert_eq!(*c.value(), 200);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(HttpStatusCode::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_out_of_range() {
        assert!(HttpStatusCode::try_from("99").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = HttpStatusCode::new(200).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "200");
        let back: HttpStatusCode = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<HttpStatusCode, _> = serde_json::from_str("99");
        assert!(result.is_err());
    }
}
