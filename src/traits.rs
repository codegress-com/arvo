/// Core trait for all value objects in arvo.
///
/// A value object is an immutable, validated wrapper around a raw value.
/// Construction via [`new`](ValueObject::new) is the **only** way to obtain
/// a valid instance — invalid states are unrepresentable at the type level.
///
/// # Type parameters
///
/// - `Input` — the type accepted by [`new`](ValueObject::new).
///   For simple types this is the raw primitive (e.g. `String`).
///   For composite types this is a dedicated input struct.
/// - `Error` — the error returned when validation fails.
///
/// Simple types (single-primitive wrappers) additionally implement
/// [`PrimitiveValue`], which exposes the inner value via [`value()`](PrimitiveValue::value).
/// Composite types expose their data through dedicated accessor methods instead.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::traits::{ValueObject, PrimitiveValue};
/// use arvo::errors::ValidationError;
///
/// pub struct NonNegative(f64);
///
/// impl ValueObject for NonNegative {
///     type Input = f64;
///     type Error = ValidationError;
///
///     fn new(value: f64) -> Result<Self, ValidationError> {
///         if value < 0.0 {
///             return Err(ValidationError::invalid("NonNegative", &value.to_string()));
///         }
///         Ok(Self(value))
///     }
///
///     fn into_inner(self) -> f64 { self.0 }
/// }
///
/// impl PrimitiveValue for NonNegative {
///     type Primitive = f64;
///     fn value(&self) -> &f64 { &self.0 }
/// }
/// ```
pub trait ValueObject: Sized + Clone + PartialEq {
    /// The type accepted by [`new`](ValueObject::new).
    type Input;

    /// The error produced when validation fails.
    type Error: std::error::Error;

    /// Constructs a new value object, validating and normalising the input.
    ///
    /// Returns `Err` if the value does not satisfy domain constraints.
    fn new(value: Self::Input) -> Result<Self, Self::Error>;

    /// Consumes the value object and returns the original input value.
    fn into_inner(self) -> Self::Input;
}

/// Extension of [`ValueObject`] for simple single-primitive newtypes.
///
/// Implemented by every type whose validated representation is a single
/// primitive value (e.g. `EmailAddress` wraps `String`, `Latitude` wraps `f64`).
/// Composite types (e.g. `Money`, `PostalAddress`) do **not** implement this
/// trait — they expose their data through dedicated accessor methods.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::EmailAddress;
/// use arvo::traits::{ValueObject, PrimitiveValue};
///
/// let email = EmailAddress::new("user@example.com".into())?;
/// assert_eq!(email.value(), "user@example.com");
///
/// // Generic bound for code that only needs the inner primitive:
/// fn print_value<T: PrimitiveValue<Primitive = str>>(v: &T) {
///     println!("{}", v.value());
/// }
/// ```
pub trait PrimitiveValue: ValueObject {
    /// The primitive type wrapped by this value object.
    type Primitive: ?Sized;

    /// Returns a reference to the validated inner value.
    fn value(&self) -> &Self::Primitive;
}
