use thiserror::Error;

/// Describes why a value object failed to construct.
///
/// Each variant carries the `type_name` of the rejecting value object,
/// making error messages self-explanatory without inspecting the call site.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::errors::ValidationError;
///
/// let err = ValidationError::invalid("EmailAddress", "not-an-email");
/// assert_eq!(err.to_string(), "'not-an-email' is not a valid EmailAddress");
/// ```
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    /// The value does not match the expected format or pattern.
    #[error("'{value}' is not a valid {type_name}")]
    InvalidFormat {
        type_name: &'static str,
        value: String,
    },

    /// The value falls outside the permitted numeric or length range.
    #[error("{type_name} must be between {min} and {max}, got {actual}")]
    OutOfRange {
        type_name: &'static str,
        min: String,
        max: String,
        actual: String,
    },

    /// The value was empty or consisted solely of whitespace.
    #[error("{type_name} must not be empty")]
    Empty { type_name: &'static str },

    /// A domain-specific rule was violated that does not fit other variants.
    #[error("{type_name}: {message}")]
    Custom {
        type_name: &'static str,
        message: String,
    },
}

impl ValidationError {
    /// Shorthand for constructing an [`InvalidFormat`](Self::InvalidFormat) error.
    pub fn invalid(type_name: &'static str, value: &str) -> Self {
        Self::InvalidFormat {
            type_name,
            value: value.to_owned(),
        }
    }

    /// Shorthand for constructing an [`Empty`](Self::Empty) error.
    pub fn empty(type_name: &'static str) -> Self {
        Self::Empty { type_name }
    }
}
