// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_global_tables::_list_global_tables_output::ListGlobalTablesOutputBuilder;

pub use crate::operation::list_global_tables::_list_global_tables_input::ListGlobalTablesInputBuilder;

impl crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_global_tables::ListGlobalTablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_global_tables();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGlobalTables`.
///
/// <p>Lists all global tables that have a replica in the specified Region.</p><important>
/// <p>This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. Customers should use <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GlobalTables.html">Global Tables version 2019.11.21 (Current)</a> when possible, because it provides greater flexibility, higher efficiency, and consumes less write capacity than 2017.11.29 (Legacy).</p>
/// <p>To determine which version you're using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the global table version you are using</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html">Upgrading global tables</a>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGlobalTablesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        crate::operation::list_global_tables::ListGlobalTablesError,
    > for ListGlobalTablesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_global_tables::ListGlobalTablesOutput,
            crate::operation::list_global_tables::ListGlobalTablesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListGlobalTablesFluentBuilder {
    /// Creates a new `ListGlobalTablesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListGlobalTables as a reference.
    pub fn as_input(&self) -> &crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder {
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
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_global_tables::ListGlobalTablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_global_tables::ListGlobalTables::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_global_tables::ListGlobalTables::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        crate::operation::list_global_tables::ListGlobalTablesError,
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
    /// <p>The first global table name that this operation will evaluate.</p>
    pub fn exclusive_start_global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.exclusive_start_global_table_name(input.into());
        self
    }
    /// <p>The first global table name that this operation will evaluate.</p>
    pub fn set_exclusive_start_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_exclusive_start_global_table_name(input);
        self
    }
    /// <p>The first global table name that this operation will evaluate.</p>
    pub fn get_exclusive_start_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_exclusive_start_global_table_name()
    }
    /// <p>The maximum number of table names to return, if the parameter is not specified DynamoDB defaults to 100.</p>
    /// <p>If the number of global tables DynamoDB finds reaches this limit, it stops the operation and returns the table names collected up to that point, with a table name in the <code>LastEvaluatedGlobalTableName</code> to apply in a subsequent operation to the <code>ExclusiveStartGlobalTableName</code> parameter.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of table names to return, if the parameter is not specified DynamoDB defaults to 100.</p>
    /// <p>If the number of global tables DynamoDB finds reaches this limit, it stops the operation and returns the table names collected up to that point, with a table name in the <code>LastEvaluatedGlobalTableName</code> to apply in a subsequent operation to the <code>ExclusiveStartGlobalTableName</code> parameter.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of table names to return, if the parameter is not specified DynamoDB defaults to 100.</p>
    /// <p>If the number of global tables DynamoDB finds reaches this limit, it stops the operation and returns the table names collected up to that point, with a table name in the <code>LastEvaluatedGlobalTableName</code> to apply in a subsequent operation to the <code>ExclusiveStartGlobalTableName</code> parameter.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>Lists the global tables in a specific Region.</p>
    pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.region_name(input.into());
        self
    }
    /// <p>Lists the global tables in a specific Region.</p>
    pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_region_name(input);
        self
    }
    /// <p>Lists the global tables in a specific Region.</p>
    pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_region_name()
    }
}