// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Access to the target resource is no longer available at the origin server. This condition is likely to be permanent.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GoneException {
    #[allow(missing_docs)] // documentation missing in model
    pub code: ::std::option::Option<crate::types::ErrorCode>,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl GoneException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(&self) -> ::std::option::Option<&crate::types::ErrorCode> {
        self.code.as_ref()
    }
}
impl GoneException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for GoneException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "GoneException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for GoneException {}
impl ::aws_types::request_id::RequestId for crate::types::error::GoneException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GoneException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl GoneException {
    /// Creates a new builder-style object to manufacture [`GoneException`](crate::types::error::GoneException).
    pub fn builder() -> crate::types::error::builders::GoneExceptionBuilder {
        crate::types::error::builders::GoneExceptionBuilder::default()
    }
}

/// A builder for [`GoneException`](crate::types::error::GoneException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GoneExceptionBuilder {
    pub(crate) code: ::std::option::Option<crate::types::ErrorCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl GoneExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(mut self, input: crate::types::ErrorCode) -> Self {
        self.code = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_code(mut self, input: ::std::option::Option<crate::types::ErrorCode>) -> Self {
        self.code = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_code(&self) -> &::std::option::Option<crate::types::ErrorCode> {
        &self.code
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`GoneException`](crate::types::error::GoneException).
    pub fn build(self) -> crate::types::error::GoneException {
        crate::types::error::GoneException {
            code: self.code,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
