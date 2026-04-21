use crate::errors::ValidationError;
use crate::traits::ValueObject;
use once_cell::sync::Lazy;
use regex::Regex;

use super::country_code::CountryCode;

/// Input type for [`PhoneNumber`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhoneNumberInput {
    /// ISO 3166-1 alpha-2 country code, used to derive the calling code prefix.
    pub country_code: CountryCode,
    /// Local number digits only (no spaces, dashes, or prefix).
    pub number: String,
}

/// Output type for [`PhoneNumber`] — canonical E.164 string, e.g. `"+420123456789"`.
pub type PhoneNumberOutput = String;

/// Validates the local number part: digits only, 4–14 characters.
static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4,14}$").unwrap());

/// A validated phone number stored in canonical E.164 format.
///
/// Constructed from a [`CountryCode`] and a local number string. On construction
/// the number is stripped of any non-digit characters and the calling code is
/// looked up from the country code. The stored value is always
/// `"+<calling_code><number>"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::{CountryCode, PhoneNumber, PhoneNumberInput};
/// use arvo::traits::ValueObject;
///
/// let phone = PhoneNumber::new(PhoneNumberInput {
///     country_code: CountryCode::new("CZ".into()).unwrap(),
///     number: "123456789".into(),
/// }).unwrap();
///
/// assert_eq!(phone.value(), "+420123456789");
/// assert_eq!(phone.calling_code(), "+420");
/// assert_eq!(phone.number(), "123456789");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct PhoneNumber {
    input: PhoneNumberInput,
    /// Pre-computed canonical E.164 string returned by `value()`.
    e164: String,
}

// Required by serde(into = "String")
#[cfg(feature = "serde")]
impl From<PhoneNumber> for String {
    fn from(p: PhoneNumber) -> String {
        p.e164
    }
}

impl ValueObject for PhoneNumber {
    type Input = PhoneNumberInput;
    type Output = PhoneNumberOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        // Strip any accidental formatting — keep digits only.
        let number: String = value
            .number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        if !NUMBER_REGEX.is_match(&number) {
            return Err(ValidationError::invalid("PhoneNumber", &number));
        }

        let prefix = calling_code(value.country_code.value()).ok_or_else(|| {
            ValidationError::invalid("PhoneNumber", value.country_code.value())
        })?;
        let e164 = format!("{}{}", prefix, number);

        Ok(Self {
            input: PhoneNumberInput {
                country_code: value.country_code,
                number,
            },
            e164,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.e164
    }

    fn into_inner(self) -> Self::Input {
        self.input
    }
}

impl PhoneNumber {
    /// Returns the ITU calling code prefix, e.g. `"+420"`.
    pub fn calling_code(&self) -> &str {
        calling_code(self.input.country_code.value()).unwrap_or("+0")
    }

    /// Returns the local number digits without the calling code, e.g. `"123456789"`.
    pub fn number(&self) -> &str {
        &self.input.number
    }

    /// Returns the country code, e.g. `CountryCode("CZ")`.
    pub fn country_code(&self) -> &CountryCode {
        &self.input.country_code
    }
}

/// Displays the phone number in canonical E.164 format.
impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e164)
    }
}

fn calling_code(country: &str) -> Option<&'static str> {
    Some(match country {
        "AF" => "+93",
        "AL" => "+355",
        "DZ" => "+213",
        "AD" => "+376",
        "AO" => "+244",
        "AG" => "+1268",
        "AR" => "+54",
        "AM" => "+374",
        "AU" => "+61",
        "AT" => "+43",
        "AZ" => "+994",
        "BS" => "+1242",
        "BH" => "+973",
        "BD" => "+880",
        "BB" => "+1246",
        "BY" => "+375",
        "BE" => "+32",
        "BZ" => "+501",
        "BJ" => "+229",
        "BT" => "+975",
        "BO" => "+591",
        "BA" => "+387",
        "BW" => "+267",
        "BR" => "+55",
        "BN" => "+673",
        "BG" => "+359",
        "BF" => "+226",
        "BI" => "+257",
        "CV" => "+238",
        "KH" => "+855",
        "CM" => "+237",
        "CA" => "+1",
        "CF" => "+236",
        "TD" => "+235",
        "CL" => "+56",
        "CN" => "+86",
        "CO" => "+57",
        "KM" => "+269",
        "CG" => "+242",
        "CD" => "+243",
        "CR" => "+506",
        "CI" => "+225",
        "HR" => "+385",
        "CU" => "+53",
        "CY" => "+357",
        "CZ" => "+420",
        "DK" => "+45",
        "DJ" => "+253",
        "DM" => "+1767",
        "DO" => "+1809",
        "EC" => "+593",
        "EG" => "+20",
        "SV" => "+503",
        "GQ" => "+240",
        "ER" => "+291",
        "EE" => "+372",
        "SZ" => "+268",
        "ET" => "+251",
        "FJ" => "+679",
        "FI" => "+358",
        "FR" => "+33",
        "GA" => "+241",
        "GM" => "+220",
        "GE" => "+995",
        "DE" => "+49",
        "GH" => "+233",
        "GI" => "+350",
        "GR" => "+30",
        "GL" => "+299",
        "GD" => "+1473",
        "GT" => "+502",
        "GN" => "+224",
        "GW" => "+245",
        "GY" => "+592",
        "HT" => "+509",
        "HN" => "+504",
        "HK" => "+852",
        "HU" => "+36",
        "IS" => "+354",
        "IN" => "+91",
        "ID" => "+62",
        "IR" => "+98",
        "IQ" => "+964",
        "IE" => "+353",
        "IL" => "+972",
        "IT" => "+39",
        "JM" => "+1876",
        "JP" => "+81",
        "JO" => "+962",
        "KZ" => "+7",
        "KE" => "+254",
        "KI" => "+686",
        "KP" => "+850",
        "KR" => "+82",
        "KW" => "+965",
        "KG" => "+996",
        "LA" => "+856",
        "LV" => "+371",
        "LB" => "+961",
        "LS" => "+266",
        "LR" => "+231",
        "LY" => "+218",
        "LI" => "+423",
        "LT" => "+370",
        "LU" => "+352",
        "MO" => "+853",
        "MG" => "+261",
        "MW" => "+265",
        "MY" => "+60",
        "MV" => "+960",
        "ML" => "+223",
        "MT" => "+356",
        "MH" => "+692",
        "MR" => "+222",
        "MU" => "+230",
        "MX" => "+52",
        "FM" => "+691",
        "MD" => "+373",
        "MC" => "+377",
        "MN" => "+976",
        "ME" => "+382",
        "MA" => "+212",
        "MZ" => "+258",
        "MM" => "+95",
        "NA" => "+264",
        "NR" => "+674",
        "NP" => "+977",
        "NL" => "+31",
        "NZ" => "+64",
        "NI" => "+505",
        "NE" => "+227",
        "NG" => "+234",
        "MK" => "+389",
        "NO" => "+47",
        "OM" => "+968",
        "PK" => "+92",
        "PW" => "+680",
        "PS" => "+970",
        "PA" => "+507",
        "PG" => "+675",
        "PY" => "+595",
        "PE" => "+51",
        "PH" => "+63",
        "PL" => "+48",
        "PT" => "+351",
        "QA" => "+974",
        "RO" => "+40",
        "RU" => "+7",
        "RW" => "+250",
        "KN" => "+1869",
        "LC" => "+1758",
        "VC" => "+1784",
        "WS" => "+685",
        "SM" => "+378",
        "ST" => "+239",
        "SA" => "+966",
        "SN" => "+221",
        "RS" => "+381",
        "SC" => "+248",
        "SL" => "+232",
        "SG" => "+65",
        "SK" => "+421",
        "SI" => "+386",
        "SB" => "+677",
        "SO" => "+252",
        "ZA" => "+27",
        "SS" => "+211",
        "ES" => "+34",
        "LK" => "+94",
        "SD" => "+249",
        "SR" => "+597",
        "SE" => "+46",
        "CH" => "+41",
        "SY" => "+963",
        "TW" => "+886",
        "TJ" => "+992",
        "TZ" => "+255",
        "TH" => "+66",
        "TL" => "+670",
        "TG" => "+228",
        "TO" => "+676",
        "TT" => "+1868",
        "TN" => "+216",
        "TR" => "+90",
        "TM" => "+993",
        "UG" => "+256",
        "UA" => "+380",
        "AE" => "+971",
        "GB" => "+44",
        "US" => "+1",
        "UY" => "+598",
        "UZ" => "+998",
        "VU" => "+678",
        "VE" => "+58",
        "VN" => "+84",
        "YE" => "+967",
        "ZM" => "+260",
        "ZW" => "+263",
        // Territories, islands and special regions
        "AX" => "+358",
        "AS" => "+1684",
        "AI" => "+1264",
        "AQ" => "+672",
        "AW" => "+297",
        "BM" => "+1441",
        "BQ" => "+599",
        "BV" => "+47",
        "IO" => "+246",
        "CK" => "+682",
        "CX" => "+61",
        "CC" => "+61",
        "CW" => "+599",
        "FK" => "+500",
        "FO" => "+298",
        "GF" => "+594",
        "PF" => "+689",
        "TF" => "+262",
        "GG" => "+44",
        "GP" => "+590",
        "GU" => "+1671",
        "HM" => "+672",
        "VA" => "+379",
        "IM" => "+44",
        "JE" => "+44",
        "YT" => "+262",
        "MQ" => "+596",
        "MF" => "+590",
        "MS" => "+1664",
        "NC" => "+687",
        "NF" => "+672",
        "NU" => "+683",
        "MP" => "+1670",
        "PN" => "+64",
        "PR" => "+1787",
        "RE" => "+262",
        "BL" => "+590",
        "SH" => "+290",
        "PM" => "+508",
        "SX" => "+1721",
        "GS" => "+500",
        "SJ" => "+47",
        "TK" => "+690",
        "TC" => "+1649",
        "TV" => "+688",
        "UM" => "+1",
        "VG" => "+1284",
        "VI" => "+1340",
        "WF" => "+681",
        "EH" => "+212",
        "KY" => "+1345",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ValueObject;

    fn cz() -> CountryCode {
        CountryCode::new("CZ".into()).unwrap()
    }

    fn us() -> CountryCode {
        CountryCode::new("US".into()).unwrap()
    }

    #[test]
    fn stores_e164_with_calling_code() {
        let p = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123456789".into(),
        })
        .unwrap();
        assert_eq!(p.value(), "+420123456789");
    }

    #[test]
    fn strips_formatting_from_number() {
        let p = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123 456 789".into(),
        })
        .unwrap();
        assert_eq!(p.value(), "+420123456789");
    }

    #[test]
    fn accessors_return_parts() {
        let p = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123456789".into(),
        })
        .unwrap();
        assert_eq!(p.calling_code(), "+420");
        assert_eq!(p.number(), "123456789");
        assert_eq!(p.country_code().value(), "CZ");
    }

    #[test]
    fn us_calling_code() {
        let p = PhoneNumber::new(PhoneNumberInput {
            country_code: us(),
            number: "2025550123".into(),
        })
        .unwrap();
        assert_eq!(p.value(), "+12025550123");
    }

    #[test]
    fn rejects_too_short_number() {
        assert!(
            PhoneNumber::new(PhoneNumberInput {
                country_code: cz(),
                number: "123".into(),
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_too_long_number() {
        assert!(
            PhoneNumber::new(PhoneNumberInput {
                country_code: cz(),
                number: "123456789012345".into(),
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_number() {
        assert!(
            PhoneNumber::new(PhoneNumberInput {
                country_code: cz(),
                number: String::new(),
            })
            .is_err()
        );
    }

    #[test]
    fn equal_after_strip() {
        let a = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123 456 789".into(),
        })
        .unwrap();
        let b = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123456789".into(),
        })
        .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn display_is_e164() {
        let p = PhoneNumber::new(PhoneNumberInput {
            country_code: cz(),
            number: "123456789".into(),
        })
        .unwrap();
        assert_eq!(p.to_string(), "+420123456789");
    }
}
