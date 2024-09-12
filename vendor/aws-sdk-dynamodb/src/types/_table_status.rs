// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `TableStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let tablestatus = unimplemented!();
/// match tablestatus {
///     TableStatus::Active => { /* ... */ },
///     TableStatus::Archived => { /* ... */ },
///     TableStatus::Archiving => { /* ... */ },
///     TableStatus::Creating => { /* ... */ },
///     TableStatus::Deleting => { /* ... */ },
///     TableStatus::InaccessibleEncryptionCredentials => { /* ... */ },
///     TableStatus::Updating => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `tablestatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TableStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TableStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TableStatus::NewFeature` is defined.
/// Specifically, when `tablestatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TableStatus::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum TableStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Archived,
    #[allow(missing_docs)] // documentation missing in model
    Archiving,
    #[allow(missing_docs)] // documentation missing in model
    Creating,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    #[allow(missing_docs)] // documentation missing in model
    InaccessibleEncryptionCredentials,
    #[allow(missing_docs)] // documentation missing in model
    Updating,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for TableStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => TableStatus::Active,
            "ARCHIVED" => TableStatus::Archived,
            "ARCHIVING" => TableStatus::Archiving,
            "CREATING" => TableStatus::Creating,
            "DELETING" => TableStatus::Deleting,
            "INACCESSIBLE_ENCRYPTION_CREDENTIALS" => TableStatus::InaccessibleEncryptionCredentials,
            "UPDATING" => TableStatus::Updating,
            other => TableStatus::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for TableStatus {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(TableStatus::from(s))
    }
}
impl TableStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TableStatus::Active => "ACTIVE",
            TableStatus::Archived => "ARCHIVED",
            TableStatus::Archiving => "ARCHIVING",
            TableStatus::Creating => "CREATING",
            TableStatus::Deleting => "DELETING",
            TableStatus::InaccessibleEncryptionCredentials => "INACCESSIBLE_ENCRYPTION_CREDENTIALS",
            TableStatus::Updating => "UPDATING",
            TableStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "ARCHIVED",
            "ARCHIVING",
            "CREATING",
            "DELETING",
            "INACCESSIBLE_ENCRYPTION_CREDENTIALS",
            "UPDATING",
        ]
    }
}
impl ::std::convert::AsRef<str> for TableStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl TableStatus {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for TableStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TableStatus::Active => write!(f, "ACTIVE"),
            TableStatus::Archived => write!(f, "ARCHIVED"),
            TableStatus::Archiving => write!(f, "ARCHIVING"),
            TableStatus::Creating => write!(f, "CREATING"),
            TableStatus::Deleting => write!(f, "DELETING"),
            TableStatus::InaccessibleEncryptionCredentials => write!(f, "INACCESSIBLE_ENCRYPTION_CREDENTIALS"),
            TableStatus::Updating => write!(f, "UPDATING"),
            TableStatus::Unknown(value) => write!(f, "{}", value),
        }
    }
}
