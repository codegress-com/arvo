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
//! Available features: `contact`, `geo`, `finance`, `temporal`,
//! `identifiers`, `net`, `measurement`, `primitives`, `serde`, `sqlx`, `full`.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use arvo::prelude::*;
//!
//! let email = EmailAddress::new("user@example.com".into())?;
//! println!("{}", email); // user@example.com
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
