// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_backup::_create_backup_output::CreateBackupOutputBuilder;

pub use crate::operation::create_backup::_create_backup_input::CreateBackupInputBuilder;

impl crate::operation::create_backup::builders::CreateBackupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_backup::CreateBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_backup::CreateBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_backup();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateBackup`.
///
/// <p>Creates a backup for an existing table.</p>
/// <p>Each time you create an on-demand backup, the entire table data is backed up. There is no limit to the number of on-demand backups that can be taken.</p>
/// <p>When you create an on-demand backup, a time marker of the request is cataloged, and the backup is created asynchronously, by applying all changes until the time of the request to the last full table snapshot. Backup requests are processed instantaneously and become available for restore within minutes.</p>
/// <p>You can call <code>CreateBackup</code> at a maximum rate of 50 times per second.</p>
/// <p>All backups in DynamoDB work without consuming any provisioned throughput on the table.</p>
/// <p>If you submit a backup request on 2018-12-14 at 14:25:00, the backup is guaranteed to contain all data committed to the table up to 14:24:00, and data committed after 14:26:00 will not be. The backup might contain data modifications made between 14:24:00 and 14:26:00. On-demand backup does not support causal consistency.</p>
/// <p>Along with data, the following are also included on the backups:</p>
/// <ul>
/// <li>
/// <p>Global secondary indexes (GSIs)</p></li>
/// <li>
/// <p>Local secondary indexes (LSIs)</p></li>
/// <li>
/// <p>Streams</p></li>
/// <li>
/// <p>Provisioned read and write capacity</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateBackupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_backup::builders::CreateBackupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_backup::CreateBackupOutput,
        crate::operation::create_backup::CreateBackupError,
    > for CreateBackupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_backup::CreateBackupOutput,
            crate::operation::create_backup::CreateBackupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateBackupFluentBuilder {
    /// Creates a new `CreateBackupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateBackup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_backup::builders::CreateBackupInputBuilder {
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
        crate::operation::create_backup::CreateBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_backup::CreateBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_backup::CreateBackup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_backup::CreateBackup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_backup::CreateBackupOutput,
        crate::operation::create_backup::CreateBackupError,
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
    /// <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// <p>Specified name for the backup.</p>
    pub fn backup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_name(input.into());
        self
    }
    /// <p>Specified name for the backup.</p>
    pub fn set_backup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_name(input);
        self
    }
    /// <p>Specified name for the backup.</p>
    pub fn get_backup_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_name()
    }
}