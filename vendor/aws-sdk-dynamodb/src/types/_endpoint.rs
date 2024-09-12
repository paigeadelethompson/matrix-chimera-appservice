// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An endpoint information details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Endpoint {
    /// <p>IP address of the endpoint.</p>
    pub address: ::std::string::String,
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub cache_period_in_minutes: i64,
}
impl Endpoint {
    /// <p>IP address of the endpoint.</p>
    pub fn address(&self) -> &str {
        use std::ops::Deref;
        self.address.deref()
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn cache_period_in_minutes(&self) -> i64 {
        self.cache_period_in_minutes
    }
}
impl Endpoint {
    /// Creates a new builder-style object to manufacture [`Endpoint`](crate::types::Endpoint).
    pub fn builder() -> crate::types::builders::EndpointBuilder {
        crate::types::builders::EndpointBuilder::default()
    }
}

/// A builder for [`Endpoint`](crate::types::Endpoint).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EndpointBuilder {
    pub(crate) address: ::std::option::Option<::std::string::String>,
    pub(crate) cache_period_in_minutes: ::std::option::Option<i64>,
}
impl EndpointBuilder {
    /// <p>IP address of the endpoint.</p>
    /// This field is required.
    pub fn address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>IP address of the endpoint.</p>
    pub fn set_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.address = input;
        self
    }
    /// <p>IP address of the endpoint.</p>
    pub fn get_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.address
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    /// This field is required.
    pub fn cache_period_in_minutes(mut self, input: i64) -> Self {
        self.cache_period_in_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn set_cache_period_in_minutes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.cache_period_in_minutes = input;
        self
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn get_cache_period_in_minutes(&self) -> &::std::option::Option<i64> {
        &self.cache_period_in_minutes
    }
    /// Consumes the builder and constructs a [`Endpoint`](crate::types::Endpoint).
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](crate::types::builders::EndpointBuilder::address)
    pub fn build(self) -> ::std::result::Result<crate::types::Endpoint, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Endpoint {
            address: self.address.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "address",
                    "address was not specified but it is required when building Endpoint",
                )
            })?,
            cache_period_in_minutes: self.cache_period_in_minutes.unwrap_or_default(),
        })
    }
}
