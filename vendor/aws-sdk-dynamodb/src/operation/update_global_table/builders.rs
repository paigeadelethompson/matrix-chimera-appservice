// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_global_table::_update_global_table_output::UpdateGlobalTableOutputBuilder;

pub use crate::operation::update_global_table::_update_global_table_input::UpdateGlobalTableInputBuilder;

impl crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_global_table::UpdateGlobalTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_global_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateGlobalTable`.
///
/// <p>Adds or removes replicas in the specified global table. The global table must already exist to be able to use this operation. Any replica to be added must be empty, have the same name as the global table, have the same key schema, have DynamoDB Streams enabled, and have the same provisioned and maximum write capacity units.</p><important>
/// <p>This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. Customers should use <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GlobalTables.html">Global Tables version 2019.11.21 (Current)</a> when possible, because it provides greater flexibility, higher efficiency, and consumes less write capacity than 2017.11.29 (Legacy).</p>
/// <p>To determine which version you're using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the global table version you are using</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html">Upgrading global tables</a>.</p>
/// </important> <note>
/// <p>For global tables, this operation only applies to global tables using Version 2019.11.21 (Current version). If you are using global tables <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GlobalTables.html">Version 2019.11.21</a> you can use <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateTable.html">UpdateTable</a> instead.</p>
/// <p>Although you can use <code>UpdateGlobalTable</code> to add replicas and remove replicas in a single request, for simplicity we recommend that you issue separate requests for adding or removing replicas.</p>
/// </note>
/// <p>If global secondary indexes are specified, then the following conditions must also be met:</p>
/// <ul>
/// <li>
/// <p>The global secondary indexes must have the same name.</p></li>
/// <li>
/// <p>The global secondary indexes must have the same hash key and sort key (if present).</p></li>
/// <li>
/// <p>The global secondary indexes must have the same provisioned and maximum write capacity units.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateGlobalTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        crate::operation::update_global_table::UpdateGlobalTableError,
    > for UpdateGlobalTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_global_table::UpdateGlobalTableOutput,
            crate::operation::update_global_table::UpdateGlobalTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateGlobalTableFluentBuilder {
    /// Creates a new `UpdateGlobalTableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateGlobalTable as a reference.
    pub fn as_input(&self) -> &crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder {
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
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_global_table::UpdateGlobalTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_global_table::UpdateGlobalTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_global_table::UpdateGlobalTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        crate::operation::update_global_table::UpdateGlobalTableError,
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
    /// <p>The global table name.</p>
    pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_table_name(input.into());
        self
    }
    /// <p>The global table name.</p>
    pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_table_name(input);
        self
    }
    /// <p>The global table name.</p>
    pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_table_name()
    }
    ///
    /// Appends an item to `ReplicaUpdates`.
    ///
    /// To override the contents of this collection use [`set_replica_updates`](Self::set_replica_updates).
    ///
    /// <p>A list of Regions that should be added or removed from the global table.</p>
    pub fn replica_updates(mut self, input: crate::types::ReplicaUpdate) -> Self {
        self.inner = self.inner.replica_updates(input);
        self
    }
    /// <p>A list of Regions that should be added or removed from the global table.</p>
    pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicaUpdate>>) -> Self {
        self.inner = self.inner.set_replica_updates(input);
        self
    }
    /// <p>A list of Regions that should be added or removed from the global table.</p>
    pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReplicaUpdate>> {
        self.inner.get_replica_updates()
    }
}