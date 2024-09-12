// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GlobalTableGlobalSecondaryIndexSettingsUpdate {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub index_name: ::std::string::String,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
    pub provisioned_write_capacity_units: ::std::option::Option<i64>,
    /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
    pub provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl GlobalTableGlobalSecondaryIndexSettingsUpdate {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn index_name(&self) -> &str {
        use std::ops::Deref;
        self.index_name.deref()
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
    pub fn provisioned_write_capacity_units(&self) -> ::std::option::Option<i64> {
        self.provisioned_write_capacity_units
    }
    /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings_update(&self) -> ::std::option::Option<&crate::types::AutoScalingSettingsUpdate> {
        self.provisioned_write_capacity_auto_scaling_settings_update.as_ref()
    }
}
impl GlobalTableGlobalSecondaryIndexSettingsUpdate {
    /// Creates a new builder-style object to manufacture [`GlobalTableGlobalSecondaryIndexSettingsUpdate`](crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate).
    pub fn builder() -> crate::types::builders::GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder {
        crate::types::builders::GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder::default()
    }
}

/// A builder for [`GlobalTableGlobalSecondaryIndexSettingsUpdate`](crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder {
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) provisioned_write_capacity_units: ::std::option::Option<i64>,
    pub(crate) provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    /// This field is required.
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
    pub fn provisioned_write_capacity_units(mut self, input: i64) -> Self {
        self.provisioned_write_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
    pub fn set_provisioned_write_capacity_units(mut self, input: ::std::option::Option<i64>) -> Self {
        self.provisioned_write_capacity_units = input;
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
    pub fn get_provisioned_write_capacity_units(&self) -> &::std::option::Option<i64> {
        &self.provisioned_write_capacity_units
    }
    /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings_update(mut self, input: crate::types::AutoScalingSettingsUpdate) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
    pub fn set_provisioned_write_capacity_auto_scaling_settings_update(
        mut self,
        input: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings_update = input;
        self
    }
    /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
    pub fn get_provisioned_write_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<crate::types::AutoScalingSettingsUpdate> {
        &self.provisioned_write_capacity_auto_scaling_settings_update
    }
    /// Consumes the builder and constructs a [`GlobalTableGlobalSecondaryIndexSettingsUpdate`](crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate).
    /// This method will fail if any of the following fields are not set:
    /// - [`index_name`](crate::types::builders::GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder::index_name)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate {
            index_name: self.index_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_name",
                    "index_name was not specified but it is required when building GlobalTableGlobalSecondaryIndexSettingsUpdate",
                )
            })?,
            provisioned_write_capacity_units: self.provisioned_write_capacity_units,
            provisioned_write_capacity_auto_scaling_settings_update: self.provisioned_write_capacity_auto_scaling_settings_update,
        })
    }
}
