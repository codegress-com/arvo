//! Contact-related value objects: email, phone, country code, postal address, and website.
mod country_code;
mod email_address;
mod phone_number;
mod postal_address;
mod website;

pub use country_code::{CountryCode, CountryCodeInput};
pub use email_address::{EmailAddress, EmailAddressInput};
pub use phone_number::{PhoneNumber, PhoneNumberInput};
pub use postal_address::{PostalAddress, PostalAddressInput};
pub use website::{Website, WebsiteInput};