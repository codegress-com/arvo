//! Contact-related value objects: email, phone, country code, and postal address.
mod country_code;
mod email_address;
mod phone_number;

pub use country_code::CountryCode;
pub use email_address::EmailAddress;
pub use phone_number::PhoneNumber;
