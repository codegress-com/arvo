use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`HttpStatusCode`].
pub type HttpStatusCodeInput = u16;

/// Output type for [`HttpStatusCode`].
pub type HttpStatusCodeOutput = u16;

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
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct HttpStatusCode(u16);

impl ValueObject for HttpStatusCode {
    type Input = HttpStatusCodeInput;
    type Output = HttpStatusCodeOutput;
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

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
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

impl TryFrom<&str> for HttpStatusCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<u16>().map_err(|_| ValidationError::invalid("HttpStatusCode", value))?;
        Self::new(parsed)
    }
}

#[cfg(feature = "sql")]
impl sqlx::Type<sqlx::Postgres> for HttpStatusCode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sql")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for HttpStatusCode {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&(self.0 as i32), buf)
    }
}

#[cfg(feature = "sql")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for HttpStatusCode {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let n = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        let u = u16::try_from(n).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)?;
        Self::new(u).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
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
}
