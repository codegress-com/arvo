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
//! arvo = { version = "0.9", features = ["contact", "finance"] }
//! ```
//!
//! Available features: `contact`, `finance`, `geo`, `identifiers`, `measurement`, `net`,
//! `primitives`, `temporal`, `serde`, `full`.
//! See [ROADMAP.md](https://github.com/codegress-com/arvo/blob/main/ROADMAP.md) for the full type list.
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
//! country_code: CountryCode::new("CZ".into())?,
//! number: "123456789".into(),
//! })?;
//! assert_eq!(phone.value(), "+420123456789");
//! # Ok::<(), arvo::errors::ValidationError>(())
//! ```

pub mod errors;
pub mod traits;

#[cfg(feature = "contact")]
pub mod contact;

#[cfg(feature = "finance")]
pub mod finance;

#[cfg(feature = "geo")]
pub mod geo;

#[cfg(feature = "measurement")]
pub mod measurement;

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "identifiers")]
pub mod identifiers;

#[cfg(feature = "primitives")]
pub mod primitives;

#[cfg(feature = "temporal")]
pub mod temporal;

/// Convenience re-exports for the most commonly used types.
///
/// Add `use arvo::prelude::*;` to bring the `ValueObject` trait and
/// the most common value object types into scope without long paths.
pub mod prelude {
    pub use crate::errors::ValidationError;
    pub use crate::traits::{PrimitiveValue, ValueObject};

    #[cfg(feature = "contact")]
    pub use crate::contact::{
        CountryCode, CountryCodeInput, EmailAddress, EmailAddressInput, PhoneNumber,
        PhoneNumberInput, PostalAddress, PostalAddressInput, Website, WebsiteInput,
    };

    #[cfg(feature = "finance")]
    pub use crate::finance::{
        Bic, BicInput, CardExpiryDate, CardExpiryDateInput, CreditCardNumber,
        CreditCardNumberInput, CurrencyCode, CurrencyCodeInput, ExchangeRate, ExchangeRateInput,
        Iban, IbanInput, Money, MoneyInput, Percentage, PercentageInput, VatNumber, VatNumberInput,
    };

    #[cfg(feature = "geo")]
    pub use crate::geo::{
        BoundingBox, BoundingBoxInput, Coordinate, CoordinateInput, CountryRegion,
        CountryRegionInput, Latitude, LatitudeInput, Longitude, LongitudeInput, TimeZone,
        TimeZoneInput,
    };

    #[cfg(feature = "identifiers")]
    pub use crate::identifiers::{
        Ean8, Ean8Input, Ean13, Ean13Input, Isbn10, Isbn10Input, Isbn13, Isbn13Input, Issn,
        IssnInput, Slug, SlugInput, Vin, VinInput,
    };

    #[cfg(feature = "measurement")]
    pub use crate::measurement::{
        Area, AreaInput, AreaUnit, Energy, EnergyInput, EnergyUnit, Frequency, FrequencyInput,
        FrequencyUnit, Length, LengthInput, LengthUnit, Power, PowerInput, PowerUnit, Pressure,
        PressureInput, PressureUnit, Speed, SpeedInput, SpeedUnit, Temperature, TemperatureInput,
        TemperatureUnit, Volume, VolumeInput, VolumeUnit, Weight, WeightInput, WeightUnit,
    };

    #[cfg(feature = "net")]
    pub use crate::net::{
        ApiKey, ApiKeyInput, Domain, DomainInput, HttpStatusCode, HttpStatusCodeInput, IpAddress,
        IpAddressInput, IpV4Address, IpV4AddressInput, IpV6Address, IpV6AddressInput, MacAddress,
        MacAddressInput, MimeType, MimeTypeInput, Port, PortInput, Url, UrlInput,
    };

    #[cfg(feature = "primitives")]
    pub use crate::primitives::{
        Base64String, Base64StringInput, BoundedString, HexColor, HexColorInput, Locale,
        LocaleInput, NonEmptyString, NonEmptyStringInput, NonNegativeDecimal,
        NonNegativeDecimalInput, NonNegativeInt, NonNegativeIntInput, PositiveDecimal,
        PositiveDecimalInput, PositiveInt, PositiveIntInput, Probability, ProbabilityInput,
    };

    #[cfg(feature = "temporal")]
    pub use crate::temporal::{
        BirthDate, BirthDateInput, BusinessHours, BusinessHoursInput, ExpiryDate, ExpiryDateInput,
        TimeRange, TimeRangeInput, UnixTimestamp, UnixTimestampInput,
    };
}
