// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates that one or more redirect URI in the request is not supported for this operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvalidRedirectUriException {
    /// <p>Single error code. For this exception the value will be <code>invalid_redirect_uri</code>.</p>
    pub error: ::std::option::Option<::std::string::String>,
    /// <p>Human-readable text providing additional information, used to assist the client developer in understanding the error that occurred.</p>
    pub error_description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl InvalidRedirectUriException {
    /// <p>Single error code. For this exception the value will be <code>invalid_redirect_uri</code>.</p>
    pub fn error(&self) -> ::std::option::Option<&str> {
        self.error.as_deref()
    }
    /// <p>Human-readable text providing additional information, used to assist the client developer in understanding the error that occurred.</p>
    pub fn error_description(&self) -> ::std::option::Option<&str> {
        self.error_description.as_deref()
    }
}
impl InvalidRedirectUriException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for InvalidRedirectUriException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "InvalidRedirectUriException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for InvalidRedirectUriException {}
impl ::aws_types::request_id::RequestId for crate::types::error::InvalidRedirectUriException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidRedirectUriException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InvalidRedirectUriException {
    /// Creates a new builder-style object to manufacture [`InvalidRedirectUriException`](crate::types::error::InvalidRedirectUriException).
    pub fn builder() -> crate::types::error::builders::InvalidRedirectUriExceptionBuilder {
        crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default()
    }
}

/// A builder for [`InvalidRedirectUriException`](crate::types::error::InvalidRedirectUriException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InvalidRedirectUriExceptionBuilder {
    pub(crate) error: ::std::option::Option<::std::string::String>,
    pub(crate) error_description: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl InvalidRedirectUriExceptionBuilder {
    /// <p>Single error code. For this exception the value will be <code>invalid_redirect_uri</code>.</p>
    pub fn error(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Single error code. For this exception the value will be <code>invalid_redirect_uri</code>.</p>
    pub fn set_error(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error = input;
        self
    }
    /// <p>Single error code. For this exception the value will be <code>invalid_redirect_uri</code>.</p>
    pub fn get_error(&self) -> &::std::option::Option<::std::string::String> {
        &self.error
    }
    /// <p>Human-readable text providing additional information, used to assist the client developer in understanding the error that occurred.</p>
    pub fn error_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Human-readable text providing additional information, used to assist the client developer in understanding the error that occurred.</p>
    pub fn set_error_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_description = input;
        self
    }
    /// <p>Human-readable text providing additional information, used to assist the client developer in understanding the error that occurred.</p>
    pub fn get_error_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_description
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
    /// Consumes the builder and constructs a [`InvalidRedirectUriException`](crate::types::error::InvalidRedirectUriException).
    pub fn build(self) -> crate::types::error::InvalidRedirectUriException {
        crate::types::error::InvalidRedirectUriException {
            error: self.error,
            error_description: self.error_description,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
