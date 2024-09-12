// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to perform a <code>DeleteItem</code> operation on an item.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    pub key: ::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>,
}
impl DeleteRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    pub fn key(&self) -> &::std::collections::HashMap<::std::string::String, crate::types::AttributeValue> {
        &self.key
    }
}
impl DeleteRequest {
    /// Creates a new builder-style object to manufacture [`DeleteRequest`](crate::types::DeleteRequest).
    pub fn builder() -> crate::types::builders::DeleteRequestBuilder {
        crate::types::builders::DeleteRequestBuilder::default()
    }
}

/// A builder for [`DeleteRequest`](crate::types::DeleteRequest).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteRequestBuilder {
    pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
}
impl DeleteRequestBuilder {
    /// Adds a key-value pair to `key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    pub fn key(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        let mut hash_map = self.key.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.key = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>) -> Self {
        self.key = input;
        self
    }
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        &self.key
    }
    /// Consumes the builder and constructs a [`DeleteRequest`](crate::types::DeleteRequest).
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](crate::types::builders::DeleteRequestBuilder::key)
    pub fn build(self) -> ::std::result::Result<crate::types::DeleteRequest, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::DeleteRequest {
            key: self.key.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key",
                    "key was not specified but it is required when building DeleteRequest",
                )
            })?,
        })
    }
}
