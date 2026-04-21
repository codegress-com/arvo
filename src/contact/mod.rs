//! Contact-related value objects: email, phone, country code, postal address, and website.
mod country_code;
mod email_address;
mod phone_number;
mod postal_address;
mod website;

pub use country_code::CountryCode;
pub use email_address::EmailAddress;
pub use phone_number::{PhoneNumber, PhoneNumberInput, PhoneNumberOutput};
pub use postal_address::PostalAddress;
pub use postal_address::PostalAddressInput;
pub use website::Website;
pub use website::WebsiteInput;
