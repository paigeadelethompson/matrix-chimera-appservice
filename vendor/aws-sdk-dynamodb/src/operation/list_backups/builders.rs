// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_backups::_list_backups_output::ListBackupsOutputBuilder;

pub use crate::operation::list_backups::_list_backups_input::ListBackupsInputBuilder;

impl crate::operation::list_backups::builders::ListBackupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_backups::ListBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_backups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBackups`.
///
/// <p>List DynamoDB backups that are associated with an Amazon Web Services account and weren't made with Amazon Web Services Backup. To list these backups for a given table, specify <code>TableName</code>. <code>ListBackups</code> returns a paginated list of results with at most 1 MB worth of items in a page. You can also specify a maximum number of entries to be returned in a page.</p>
/// <p>In the request, start time is inclusive, but end time is exclusive. Note that these boundaries are for the time at which the original backup was requested.</p>
/// <p>You can call <code>ListBackups</code> a maximum of five times per second.</p>
/// <p>If you want to retrieve the complete list of backups made with Amazon Web Services Backup, use the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/API_ListBackupJobs.html">Amazon Web Services Backup list API.</a></p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBackupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_backups::builders::ListBackupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_backups::ListBackupsOutput,
        crate::operation::list_backups::ListBackupsError,
    > for ListBackupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_backups::ListBackupsOutput,
            crate::operation::list_backups::ListBackupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListBackupsFluentBuilder {
    /// Creates a new `ListBackupsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListBackups as a reference.
    pub fn as_input(&self) -> &crate::operation::list_backups::builders::ListBackupsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_backups::ListBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_backups::ListBackups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_backups::ListBackups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_backups::ListBackupsOutput,
        crate::operation::list_backups::ListBackupsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Lists the backups from the table specified in <code>TableName</code>. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn time_range_lower_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.time_range_lower_bound(input);
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn set_time_range_lower_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_time_range_lower_bound(input);
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn get_time_range_lower_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_time_range_lower_bound()
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn time_range_upper_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.time_range_upper_bound(input);
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn set_time_range_upper_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_time_range_upper_bound(input);
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive.</p>
    pub fn get_time_range_upper_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_time_range_upper_bound()
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn exclusive_start_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.exclusive_start_backup_arn(input.into());
        self
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn set_exclusive_start_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_exclusive_start_backup_arn(input);
        self
    }
    /// <p><code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results.</p>
    pub fn get_exclusive_start_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_exclusive_start_backup_arn()
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
        self.inner = self.inner.backup_type(input);
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
        self.inner = self.inner.set_backup_type(input);
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
        self.inner.get_backup_type()
    }
}
