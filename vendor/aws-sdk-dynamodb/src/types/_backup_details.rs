// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the backup created for the table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BackupDetails {
    /// <p>ARN associated with the backup.</p>
    pub backup_arn: ::std::string::String,
    /// <p>Name of the requested backup.</p>
    pub backup_name: ::std::string::String,
    /// <p>Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    pub backup_size_bytes: ::std::option::Option<i64>,
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    pub backup_status: crate::types::BackupStatus,
    /// <p>BackupType:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - You create and manage these using the on-demand backup feature.</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - If you delete a table with point-in-time recovery enabled, a <code>SYSTEM</code> backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion.</p></li>
    /// <li>
    /// <p><code>AWS_BACKUP</code> - On-demand backup created by you from Backup service.</p></li>
    /// </ul>
    pub backup_type: crate::types::BackupType,
    /// <p>Time at which the backup was created. This is the request time of the backup.</p>
    pub backup_creation_date_time: ::aws_smithy_types::DateTime,
    /// <p>Time at which the automatic on-demand backup created by DynamoDB will expire. This <code>SYSTEM</code> on-demand backup expires automatically 35 days after its creation.</p>
    pub backup_expiry_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BackupDetails {
    /// <p>ARN associated with the backup.</p>
    pub fn backup_arn(&self) -> &str {
        use std::ops::Deref;
        self.backup_arn.deref()
    }
    /// <p>Name of the requested backup.</p>
    pub fn backup_name(&self) -> &str {
        use std::ops::Deref;
        self.backup_name.deref()
    }
    /// <p>Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    pub fn backup_size_bytes(&self) -> ::std::option::Option<i64> {
        self.backup_size_bytes
    }
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    pub fn backup_status(&self) -> &crate::types::BackupStatus {
        &self.backup_status
    }
    /// <p>BackupType:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - You create and manage these using the on-demand backup feature.</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - If you delete a table with point-in-time recovery enabled, a <code>SYSTEM</code> backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion.</p></li>
    /// <li>
    /// <p><code>AWS_BACKUP</code> - On-demand backup created by you from Backup service.</p></li>
    /// </ul>
    pub fn backup_type(&self) -> &crate::types::BackupType {
        &self.backup_type
    }
    /// <p>Time at which the backup was created. This is the request time of the backup.</p>
    pub fn backup_creation_date_time(&self) -> &::aws_smithy_types::DateTime {
        &self.backup_creation_date_time
    }
    /// <p>Time at which the automatic on-demand backup created by DynamoDB will expire. This <code>SYSTEM</code> on-demand backup expires automatically 35 days after its creation.</p>
    pub fn backup_expiry_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.backup_expiry_date_time.as_ref()
    }
}
impl BackupDetails {
    /// Creates a new builder-style object to manufacture [`BackupDetails`](crate::types::BackupDetails).
    pub fn builder() -> crate::types::builders::BackupDetailsBuilder {
        crate::types::builders::BackupDetailsBuilder::default()
    }
}

/// A builder for [`BackupDetails`](crate::types::BackupDetails).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BackupDetailsBuilder {
    pub(crate) backup_arn: ::std::option::Option<::std::string::String>,
    pub(crate) backup_name: ::std::option::Option<::std::string::String>,
    pub(crate) backup_size_bytes: ::std::option::Option<i64>,
    pub(crate) backup_status: ::std::option::Option<crate::types::BackupStatus>,
    pub(crate) backup_type: ::std::option::Option<crate::types::BackupType>,
    pub(crate) backup_creation_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) backup_expiry_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BackupDetailsBuilder {
    /// <p>ARN associated with the backup.</p>
    /// This field is required.
    pub fn backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.backup_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN associated with the backup.</p>
    pub fn set_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.backup_arn = input;
        self
    }
    /// <p>ARN associated with the backup.</p>
    pub fn get_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.backup_arn
    }
    /// <p>Name of the requested backup.</p>
    /// This field is required.
    pub fn backup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.backup_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the requested backup.</p>
    pub fn set_backup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.backup_name = input;
        self
    }
    /// <p>Name of the requested backup.</p>
    pub fn get_backup_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.backup_name
    }
    /// <p>Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    pub fn backup_size_bytes(mut self, input: i64) -> Self {
        self.backup_size_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    pub fn set_backup_size_bytes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.backup_size_bytes = input;
        self
    }
    /// <p>Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    pub fn get_backup_size_bytes(&self) -> &::std::option::Option<i64> {
        &self.backup_size_bytes
    }
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    /// This field is required.
    pub fn backup_status(mut self, input: crate::types::BackupStatus) -> Self {
        self.backup_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    pub fn set_backup_status(mut self, input: ::std::option::Option<crate::types::BackupStatus>) -> Self {
        self.backup_status = input;
        self
    }
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    pub fn get_backup_status(&self) -> &::std::option::Option<crate::types::BackupStatus> {
        &self.backup_status
    }
    /// <p>BackupType:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - You create and manage these using the on-demand backup feature.</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - If you delete a table with point-in-time recovery enabled, a <code>SYSTEM</code> backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion.</p></li>
    /// <li>
    /// <p><code>AWS_BACKUP</code> - On-demand backup created by you from Backup service.</p></li>
    /// </ul>
    /// This field is required.
    pub fn backup_type(mut self, input: crate::types::BackupType) -> Self {
        self.backup_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>BackupType:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - You create and manage these using the on-demand backup feature.</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - If you delete a table with point-in-time recovery enabled, a <code>SYSTEM</code> backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion.</p></li>
    /// <li>
    /// <p><code>AWS_BACKUP</code> - On-demand backup created by you from Backup service.</p></li>
    /// </ul>
    pub fn set_backup_type(mut self, input: ::std::option::Option<crate::types::BackupType>) -> Self {
        self.backup_type = input;
        self
    }
    /// <p>BackupType:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - You create and manage these using the on-demand backup feature.</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - If you delete a table with point-in-time recovery enabled, a <code>SYSTEM</code> backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion.</p></li>
    /// <li>
    /// <p><code>AWS_BACKUP</code> - On-demand backup created by you from Backup service.</p></li>
    /// </ul>
    pub fn get_backup_type(&self) -> &::std::option::Option<crate::types::BackupType> {
        &self.backup_type
    }
    /// <p>Time at which the backup was created. This is the request time of the backup.</p>
    /// This field is required.
    pub fn backup_creation_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.backup_creation_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Time at which the backup was created. This is the request time of the backup.</p>
    pub fn set_backup_creation_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.backup_creation_date_time = input;
        self
    }
    /// <p>Time at which the backup was created. This is the request time of the backup.</p>
    pub fn get_backup_creation_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.backup_creation_date_time
    }
    /// <p>Time at which the automatic on-demand backup created by DynamoDB will expire. This <code>SYSTEM</code> on-demand backup expires automatically 35 days after its creation.</p>
    pub fn backup_expiry_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.backup_expiry_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Time at which the automatic on-demand backup created by DynamoDB will expire. This <code>SYSTEM</code> on-demand backup expires automatically 35 days after its creation.</p>
    pub fn set_backup_expiry_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.backup_expiry_date_time = input;
        self
    }
    /// <p>Time at which the automatic on-demand backup created by DynamoDB will expire. This <code>SYSTEM</code> on-demand backup expires automatically 35 days after its creation.</p>
    pub fn get_backup_expiry_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.backup_expiry_date_time
    }
    /// Consumes the builder and constructs a [`BackupDetails`](crate::types::BackupDetails).
    /// This method will fail if any of the following fields are not set:
    /// - [`backup_arn`](crate::types::builders::BackupDetailsBuilder::backup_arn)
    /// - [`backup_name`](crate::types::builders::BackupDetailsBuilder::backup_name)
    /// - [`backup_status`](crate::types::builders::BackupDetailsBuilder::backup_status)
    /// - [`backup_type`](crate::types::builders::BackupDetailsBuilder::backup_type)
    /// - [`backup_creation_date_time`](crate::types::builders::BackupDetailsBuilder::backup_creation_date_time)
    pub fn build(self) -> ::std::result::Result<crate::types::BackupDetails, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::BackupDetails {
            backup_arn: self.backup_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "backup_arn",
                    "backup_arn was not specified but it is required when building BackupDetails",
                )
            })?,
            backup_name: self.backup_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "backup_name",
                    "backup_name was not specified but it is required when building BackupDetails",
                )
            })?,
            backup_size_bytes: self.backup_size_bytes,
            backup_status: self.backup_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "backup_status",
                    "backup_status was not specified but it is required when building BackupDetails",
                )
            })?,
            backup_type: self.backup_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "backup_type",
                    "backup_type was not specified but it is required when building BackupDetails",
                )
            })?,
            backup_creation_date_time: self.backup_creation_date_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "backup_creation_date_time",
                    "backup_creation_date_time was not specified but it is required when building BackupDetails",
                )
            })?,
            backup_expiry_date_time: self.backup_expiry_date_time,
        })
    }
}
