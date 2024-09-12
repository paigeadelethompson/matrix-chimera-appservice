// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a global secondary index to be deleted from an existing table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be deleted.</p>
    pub index_name: ::std::string::String,
}
impl DeleteGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be deleted.</p>
    pub fn index_name(&self) -> &str {
        use std::ops::Deref;
        self.index_name.deref()
    }
}
impl DeleteGlobalSecondaryIndexAction {
    /// Creates a new builder-style object to manufacture [`DeleteGlobalSecondaryIndexAction`](crate::types::DeleteGlobalSecondaryIndexAction).
    pub fn builder() -> crate::types::builders::DeleteGlobalSecondaryIndexActionBuilder {
        crate::types::builders::DeleteGlobalSecondaryIndexActionBuilder::default()
    }
}

/// A builder for [`DeleteGlobalSecondaryIndexAction`](crate::types::DeleteGlobalSecondaryIndexAction).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteGlobalSecondaryIndexActionBuilder {
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
}
impl DeleteGlobalSecondaryIndexActionBuilder {
    /// <p>The name of the global secondary index to be deleted.</p>
    /// This field is required.
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the global secondary index to be deleted.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The name of the global secondary index to be deleted.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// Consumes the builder and constructs a [`DeleteGlobalSecondaryIndexAction`](crate::types::DeleteGlobalSecondaryIndexAction).
    /// This method will fail if any of the following fields are not set:
    /// - [`index_name`](crate::types::builders::DeleteGlobalSecondaryIndexActionBuilder::index_name)
    pub fn build(self) -> ::std::result::Result<crate::types::DeleteGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::DeleteGlobalSecondaryIndexAction {
            index_name: self.index_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_name",
                    "index_name was not specified but it is required when building DeleteGlobalSecondaryIndexAction",
                )
            })?,
        })
    }
}
