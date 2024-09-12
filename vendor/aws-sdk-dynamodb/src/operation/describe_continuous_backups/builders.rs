// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_continuous_backups::_describe_continuous_backups_output::DescribeContinuousBackupsOutputBuilder;

pub use crate::operation::describe_continuous_backups::_describe_continuous_backups_input::DescribeContinuousBackupsInputBuilder;

impl crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_continuous_backups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeContinuousBackups`.
///
/// <p>Checks the status of continuous backups and point in time recovery on the specified table. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p>
/// <p>After continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>.</p>
/// <p><code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days.</p>
/// <p>You can call <code>DescribeContinuousBackups</code> at a maximum rate of 10 times per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeContinuousBackupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
    > for DescribeContinuousBackupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
            crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeContinuousBackupsFluentBuilder {
    /// Creates a new `DescribeContinuousBackupsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeContinuousBackups as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder {
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
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_continuous_backups::DescribeContinuousBackups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_continuous_backups::DescribeContinuousBackups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
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
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    /// <p>You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    /// <p>You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    /// <p>You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
}
