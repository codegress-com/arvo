/// Core trait for all value objects in arvo.
///
/// A value object is an immutable, validated wrapper around a raw value.
/// It guarantees that once constructed, the inner value always satisfies
/// the domain rules defined in [`ValueObject::new`].
///
/// # Type parameters
///
/// - `Input` — the type accepted by [`new`](ValueObject::new).
///   For simple types this is the raw primitive (e.g. `String`).
///   For composite types this is a dedicated input struct.
/// - `Output` — the type returned by [`value`](ValueObject::value).
///   For simple types `Input` and `Output` are the same.
///   For composite types `Output` is the canonical representation
///   (e.g. an E.164 string for a phone number).
/// - `Error` — the error returned when validation fails.
///
/// # Simple type example
///
/// ```rust,ignore
/// use arvo::traits::ValueObject;
/// use arvo::errors::ValidationError;
///
/// pub type PercentageInput  = f64;
/// pub type PercentageOutput = f64;
///
/// pub struct Percentage(f64);
///
/// impl ValueObject for Percentage {
///     type Input  = PercentageInput;
///     type Output = PercentageOutput;
///     type Error  = ValidationError;
///
///     fn new(value: f64) -> Result<Self, ValidationError> {
///         if !(0.0..=100.0).contains(&value) {
///             return Err(ValidationError::OutOfRange {
///                 type_name: "Percentage",
///                 min:    "0".into(),
///                 max:    "100".into(),
///                 actual: value.to_string(),
///             });
///         }
///         Ok(Self(value))
///     }
///
///     fn value(&self) -> &f64    { &self.0 }
///     fn into_inner(self) -> f64 { self.0 }
/// }
/// ```
///
/// # Composite type example
///
/// ```rust,ignore
/// use arvo::traits::ValueObject;
/// use arvo::errors::ValidationError;
///
/// pub struct PhoneNumberInput {
///     pub country_code: CountryCode,
///     pub number: String,
/// }
/// pub type PhoneNumberOutput = String; // canonical E.164: "+420123456789"
///
/// pub struct PhoneNumber {
///     input: PhoneNumberInput,
///     e164: String,
/// }
///
/// impl ValueObject for PhoneNumber {
///     type Input  = PhoneNumberInput;
///     type Output = PhoneNumberOutput;
///     type Error  = ValidationError;
///
///     fn new(value: PhoneNumberInput) -> Result<Self, ValidationError> { /* ... */ }
///     fn value(&self) -> &String    { &self.e164 }  // "+420123456789"
///     fn into_inner(self) -> PhoneNumberInput { self.input }
/// }
/// ```
pub trait ValueObject: Sized + Clone + PartialEq {
    /// The type accepted by [`new`](ValueObject::new).
    type Input;

    /// The type returned by [`value`](ValueObject::value).
    type Output: ?Sized;

    /// The error produced when validation fails.
    type Error: std::error::Error;

    /// Constructs a new value object, validating the input.
    ///
    /// Returns `Err` if the value does not satisfy domain constraints.
    /// This is the **only** way to create a valid instance — there is
    /// no public struct constructor.
    fn new(value: Self::Input) -> Result<Self, Self::Error>;

    /// Returns a reference to the validated output value.
    fn value(&self) -> &Self::Output;

    /// Consumes the value object and returns the original input value.
    fn into_inner(self) -> Self::Input;
}
