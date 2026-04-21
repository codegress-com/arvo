//! Contact-related value objects: email, phone, country code, postal address, and website.
mod country_code;
mod email_address;
mod phone_number;
mod postal_address;
mod website;

pub use country_code::{CountryCode, CountryCodeInput, CountryCodeOutput};
pub use email_address::{EmailAddress, EmailAddressInput, EmailAddressOutput};
pub use phone_number::{PhoneNumber, PhoneNumberInput, PhoneNumberOutput};
pub use postal_address::{PostalAddress, PostalAddressInput, PostalAddressOutput};
pub use website::{Website, WebsiteInput, WebsiteOutput};
