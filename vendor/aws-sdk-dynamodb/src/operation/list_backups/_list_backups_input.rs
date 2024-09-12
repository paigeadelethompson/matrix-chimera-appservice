// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListBackupsInput {
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>Maximum number of backups to return at once.</p>
    pub limit: ::std::option::Option<i32>,
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub time_range_lower_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub time_range_upper_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub exclusive_start_backup_arn: ::std::option::Option<::std::string::String>,
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p></li>
    /// <li>
    /// <p><code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p></li>
    /// </ul>
    pub backup_type: ::std::option::Option<crate::types::BackupTypeFilter>,
}
impl ListBackupsInput {
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn time_range_lower_bound(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.time_range_lower_bound.as_ref()
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn time_range_upper_bound(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.time_range_upper_bound.as_ref()
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn exclusive_start_backup_arn(&self) -> ::std::option::Option<&str> {
        self.exclusive_start_backup_arn.as_deref()
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p></li>
    /// <li>
    /// <p><code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p></li>
    /// </ul>
    pub fn backup_type(&self) -> ::std::option::Option<&crate::types::BackupTypeFilter> {
        self.backup_type.as_ref()
    }
}
impl ListBackupsInput {
    /// Creates a new builder-style object to manufacture [`ListBackupsInput`](crate::operation::list_backups::ListBackupsInput).
    pub fn builder() -> crate::operation::list_backups::builders::ListBackupsInputBuilder {
        crate::operation::list_backups::builders::ListBackupsInputBuilder::default()
    }
}

/// A builder for [`ListBackupsInput`](crate::operation::list_backups::ListBackupsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListBackupsInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
    pub(crate) time_range_lower_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) time_range_upper_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) exclusive_start_backup_arn: ::std::option::Option<::std::string::String>,
    pub(crate) backup_type: ::std::option::Option<crate::types::BackupTypeFilter>,
}
impl ListBackupsInputBuilder {
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        &self.limit
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn time_range_lower_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.time_range_lower_bound = ::std::option::Option::Some(input);
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn set_time_range_lower_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.time_range_lower_bound = input;
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn get_time_range_lower_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.time_range_lower_bound
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn time_range_upper_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.time_range_upper_bound = ::std::option::Option::Some(input);
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn set_time_range_upper_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.time_range_upper_bound = input;
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn get_time_range_upper_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.time_range_upper_bound
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn exclusive_start_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.exclusive_start_backup_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn set_exclusive_start_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.exclusive_start_backup_arn = input;
        self
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn get_exclusive_start_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.exclusive_start_backup_arn
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p></li>
    /// <li>
    /// <p><code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p></li>
    /// </ul>
    pub fn backup_type(mut self, input: crate::types::BackupTypeFilter) -> Self {
        self.backup_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p></li>
    /// <li>
    /// <p><code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p></li>
    /// </ul>
    pub fn set_backup_type(mut self, input: ::std::option::Option<crate::types::BackupTypeFilter>) -> Self {
        self.backup_type = input;
        self
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p></li>
    /// <li>
    /// <p><code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p></li>
    /// <li>
    /// <p><code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p></li>
    /// </ul>
    pub fn get_backup_type(&self) -> &::std::option::Option<crate::types::BackupTypeFilter> {
        &self.backup_type
    }
    /// Consumes the builder and constructs a [`ListBackupsInput`](crate::operation::list_backups::ListBackupsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_backups::ListBackupsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_backups::ListBackupsInput {
            table_name: self.table_name,
            limit: self.limit,
            time_range_lower_bound: self.time_range_lower_bound,
            time_range_upper_bound: self.time_range_upper_bound,
            exclusive_start_backup_arn: self.exclusive_start_backup_arn,
            backup_type: self.backup_type,
        })
    }
}
