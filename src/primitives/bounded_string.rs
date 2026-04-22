use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// A string whose length (in Unicode characters) is constrained to `MIN..=MAX`.
///
/// Surrounding whitespace is stripped before the length check. The type encodes
/// the allowed range at compile time via const generics, making length
/// constraints self-documenting at the call site:
///
/// ```rust,ignore
/// type Username = BoundedString<3, 32>;
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::BoundedString;
/// use arvo::traits::ValueObject;
///
/// let name: BoundedString<2, 50> = BoundedString::new("Alice".into()).unwrap();
/// assert_eq!(name.value(), "Alice");
///
/// assert!(BoundedString::<2, 50>::new("A".into()).is_err()); // too short
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BoundedString<const MIN: usize, const MAX: usize>(String);

impl<const MIN: usize, const MAX: usize> ValueObject for BoundedString<MIN, MAX> {
    type Input = String;
    type Output = String;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if MIN > MAX {
            return Err(ValidationError::Custom {
                type_name: "BoundedString",
                message: format!("MIN ({MIN}) must be <= MAX ({MAX})"),
            });
        }
        let trimmed = value.trim().to_owned();
        let len = trimmed.chars().count();
        if len < MIN || len > MAX {
            return Err(ValidationError::OutOfRange {
                type_name: "BoundedString",
                min: MIN.to_string(),
                max: MAX.to_string(),
                actual: len.to_string(),
            });
        }
        Ok(Self(trimmed))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<&str> for BoundedString<MIN, MAX> {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

#[cfg(feature = "sql")]
impl<const MIN: usize, const MAX: usize> sqlx::Type<sqlx::Postgres> for BoundedString<MIN, MAX> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sql")]
impl<'q, const MIN: usize, const MAX: usize> sqlx::Encode<'q, sqlx::Postgres>
    for BoundedString<MIN, MAX>
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

#[cfg(feature = "sql")]
impl<'r, const MIN: usize, const MAX: usize> sqlx::Decode<'r, sqlx::Postgres>
    for BoundedString<MIN, MAX>
{
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::new(s).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Display for BoundedString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_string_within_bounds() {
        let s: BoundedString<2, 10> = BoundedString::new("hello".into()).unwrap();
        assert_eq!(s.value(), "hello");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let s: BoundedString<1, 10> = BoundedString::new("  hi  ".into()).unwrap();
        assert_eq!(s.value(), "hi");
    }

    #[test]
    fn rejects_too_short() {
        assert!(BoundedString::<3, 10>::new("ab".into()).is_err());
    }

    #[test]
    fn rejects_too_long() {
        assert!(BoundedString::<1, 3>::new("toolong".into()).is_err());
    }

    #[test]
    fn accepts_exact_min() {
        let s: BoundedString<3, 10> = BoundedString::new("abc".into()).unwrap();
        assert_eq!(s.value(), "abc");
    }

    #[test]
    fn accepts_exact_max() {
        let s: BoundedString<1, 5> = BoundedString::new("hello".into()).unwrap();
        assert_eq!(s.value(), "hello");
    }

    #[test]
    fn counts_unicode_chars_not_bytes() {
        // "café" is 4 chars but 5 bytes
        let s: BoundedString<1, 4> = BoundedString::new("café".into()).unwrap();
        assert_eq!(s.value(), "café");
    }

    #[test]
    fn try_from_str() {
        let s: BoundedString<1, 10> = "test".try_into().unwrap();
        assert_eq!(s.value(), "test");
    }
}
