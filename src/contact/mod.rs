//! Contact-related value objects: email, phone, country code, postal address, and website.
mod country_code;
mod email_address;
mod phone_number;
mod website;

pub use country_code::CountryCode;
pub use email_address::EmailAddress;
pub use phone_number::PhoneNumber;
pub use website::Website;
pub use website::WebsiteInput;
