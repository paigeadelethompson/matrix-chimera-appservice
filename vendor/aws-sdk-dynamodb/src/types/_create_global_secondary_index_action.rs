// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a new global secondary index to be added to an existing table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be created.</p>
    pub index_name: ::std::string::String,
    /// <p>The key schema for the global secondary index.</p>
    pub key_schema: ::std::vec::Vec<crate::types::KeySchemaElement>,
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub projection: ::std::option::Option<crate::types::Projection>,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p>
    /// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub provisioned_throughput: ::std::option::Option<crate::types::ProvisionedThroughput>,
    /// <p>The maximum number of read and write units for the global secondary index being created. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
    pub on_demand_throughput: ::std::option::Option<crate::types::OnDemandThroughput>,
}
impl CreateGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be created.</p>
    pub fn index_name(&self) -> &str {
        use std::ops::Deref;
        self.index_name.deref()
    }
    /// <p>The key schema for the global secondary index.</p>
    pub fn key_schema(&self) -> &[crate::types::KeySchemaElement] {
        use std::ops::Deref;
        self.key_schema.deref()
    }
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn projection(&self) -> ::std::option::Option<&crate::types::Projection> {
        self.projection.as_ref()
    }
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p>
    /// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn provisioned_throughput(&self) -> ::std::option::Option<&crate::types::ProvisionedThroughput> {
        self.provisioned_throughput.as_ref()
    }
    /// <p>The maximum number of read and write units for the global secondary index being created. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
    pub fn on_demand_throughput(&self) -> ::std::option::Option<&crate::types::OnDemandThroughput> {
        self.on_demand_throughput.as_ref()
    }
}
impl CreateGlobalSecondaryIndexAction {
    /// Creates a new builder-style object to manufacture [`CreateGlobalSecondaryIndexAction`](crate::types::CreateGlobalSecondaryIndexAction).
    pub fn builder() -> crate::types::builders::CreateGlobalSecondaryIndexActionBuilder {
        crate::types::builders::CreateGlobalSecondaryIndexActionBuilder::default()
    }
}

/// A builder for [`CreateGlobalSecondaryIndexAction`](crate::types::CreateGlobalSecondaryIndexAction).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateGlobalSecondaryIndexActionBuilder {
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) key_schema: ::std::option::Option<::std::vec::Vec<crate::types::KeySchemaElement>>,
    pub(crate) projection: ::std::option::Option<crate::types::Projection>,
    pub(crate) provisioned_throughput: ::std::option::Option<crate::types::ProvisionedThroughput>,
    pub(crate) on_demand_throughput: ::std::option::Option<crate::types::OnDemandThroughput>,
}
impl CreateGlobalSecondaryIndexActionBuilder {
    /// <p>The name of the global secondary index to be created.</p>
    /// This field is required.
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the global secondary index to be created.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The name of the global secondary index to be created.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// Appends an item to `key_schema`.
    ///
    /// To override the contents of this collection use [`set_key_schema`](Self::set_key_schema).
    ///
    /// <p>The key schema for the global secondary index.</p>
    pub fn key_schema(mut self, input: crate::types::KeySchemaElement) -> Self {
        let mut v = self.key_schema.unwrap_or_default();
        v.push(input);
        self.key_schema = ::std::option::Option::Some(v);
        self
    }
    /// <p>The key schema for the global secondary index.</p>
    pub fn set_key_schema(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::KeySchemaElement>>) -> Self {
        self.key_schema = input;
        self
    }
    /// <p>The key schema for the global secondary index.</p>
    pub fn get_key_schema(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::KeySchemaElement>> {
        &self.key_schema
    }
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    /// This field is required.
    pub fn projection(mut self, input: crate::types::Projection) -> Self {
        self.projection = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn set_projection(mut self, input: ::std::option::Option<crate::types::Projection>) -> Self {
        self.projection = input;
        self
    }
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn get_projection(&self) -> &::std::option::Option<crate::types::Projection> {
        &self.projection
    }
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p>
    /// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn provisioned_throughput(mut self, input: crate::types::ProvisionedThroughput) -> Self {
        self.provisioned_throughput = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p>
    /// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_provisioned_throughput(mut self, input: ::std::option::Option<crate::types::ProvisionedThroughput>) -> Self {
        self.provisioned_throughput = input;
        self
    }
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p>
    /// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_provisioned_throughput(&self) -> &::std::option::Option<crate::types::ProvisionedThroughput> {
        &self.provisioned_throughput
    }
    /// <p>The maximum number of read and write units for the global secondary index being created. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
    pub fn on_demand_throughput(mut self, input: crate::types::OnDemandThroughput) -> Self {
        self.on_demand_throughput = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of read and write units for the global secondary index being created. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
    pub fn set_on_demand_throughput(mut self, input: ::std::option::Option<crate::types::OnDemandThroughput>) -> Self {
        self.on_demand_throughput = input;
        self
    }
    /// <p>The maximum number of read and write units for the global secondary index being created. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
    pub fn get_on_demand_throughput(&self) -> &::std::option::Option<crate::types::OnDemandThroughput> {
        &self.on_demand_throughput
    }
    /// Consumes the builder and constructs a [`CreateGlobalSecondaryIndexAction`](crate::types::CreateGlobalSecondaryIndexAction).
    /// This method will fail if any of the following fields are not set:
    /// - [`index_name`](crate::types::builders::CreateGlobalSecondaryIndexActionBuilder::index_name)
    /// - [`key_schema`](crate::types::builders::CreateGlobalSecondaryIndexActionBuilder::key_schema)
    pub fn build(self) -> ::std::result::Result<crate::types::CreateGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CreateGlobalSecondaryIndexAction {
            index_name: self.index_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_name",
                    "index_name was not specified but it is required when building CreateGlobalSecondaryIndexAction",
                )
            })?,
            key_schema: self.key_schema.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key_schema",
                    "key_schema was not specified but it is required when building CreateGlobalSecondaryIndexAction",
                )
            })?,
            projection: self.projection,
            provisioned_throughput: self.provisioned_throughput,
            on_demand_throughput: self.on_demand_throughput,
        })
    }
}