//! # arvo
//!
//! **arvo** (Finnish: *value*) — validated, immutable types for common domain values.
//!
//! Each type guarantees that if a value exists, it is valid. Construction always
//! goes through `::new()` which returns `Result`, making invalid states
//! unrepresentable.
//!
//! ## Feature flags
//!
//! Enable only the modules you need:
//!
//! ```toml
//! [dependencies]
//! arvo = { version = "0.1", features = ["contact", "finance"] }
//! ```
//!
//! Available features: `contact`, `serde`, `full`.
//! See [ROADMAP.md](https://github.com/codegress-com/arvo/blob/main/ROADMAP.md) for planned modules.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use arvo::contact::{CountryCode, PhoneNumber, PhoneNumberInput};
//! use arvo::prelude::*;
//!
//! // Simple value object — validated and normalised on construction
//! let email = EmailAddress::new("User@Example.COM".into())?;
//! assert_eq!(email.value(), "user@example.com");
//! assert_eq!(email.domain(), "example.com");
//!
//! // Composite value object — structured input, canonical output
//! let phone = PhoneNumber::new(PhoneNumberInput {
//!     country_code: CountryCode::new("CZ".into())?,
//!     number: "123456789".into(),
//! })?;
//! assert_eq!(phone.value(), "+420123456789");
//! # Ok::<(), arvo::errors::ValidationError>(())
//! ```

pub mod errors;
pub mod traits;

#[cfg(feature = "contact")]
pub mod contact;

/// Convenience re-exports for the most commonly used types.
///
/// Add `use arvo::prelude::*;` to bring the `ValueObject` trait and
/// the most common value object types into scope without long paths.
pub mod prelude {
    pub use crate::errors::ValidationError;
    pub use crate::traits::ValueObject;

    #[cfg(feature = "contact")]
    pub use crate::contact::{CountryCode, EmailAddress};
}
