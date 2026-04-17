/// Core trait for all value objects in arvo.
///
/// A value object is an immutable, validated wrapper around a raw value.
/// It guarantees that once constructed, the inner value always satisfies
/// the domain rules defined in [`ValueObject::new`].
///
/// # Type parameters
///
/// - `Raw` — the underlying primitive (e.g. `String`, `u8`, `(f64, f64)`)
/// - `Error` — the error returned when validation fails
///
/// # Implementing the trait
///
/// ```rust,ignore
/// use arvo::traits::ValueObject;
/// use arvo::errors::ValidationError;
///
/// /// A percentage value constrained to 0.0–100.0.
/// pub struct Percentage(f64);
///
/// impl ValueObject for Percentage {
///     type Raw   = f64;
///     type Error = ValidationError;
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
pub trait ValueObject: Sized + Clone + PartialEq {
    /// The raw underlying type before validation.
    type Raw;

    /// The error produced when validation fails.
    type Error: std::error::Error;

    /// Constructs a new value object, validating the raw input.
    ///
    /// Returns `Err` if the value does not satisfy domain constraints.
    /// This is the **only** way to create a valid instance — there is
    /// no public struct constructor.
    fn new(value: Self::Raw) -> Result<Self, Self::Error>;

    /// Returns a reference to the validated inner value.
    fn value(&self) -> &Self::Raw;

    /// Consumes the value object and returns the inner raw value.
    fn into_inner(self) -> Self::Raw;
}
